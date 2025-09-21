use rjvm_core::{
    Class, ClassFile, Context, Jar, JvmString, MethodDescriptor, Object, ResourceLoadType, Value,
};
use rjvm_globals::{GLOBALS_JAR, native_impl as base_native_impl};

mod loader_backend;
mod native_impl;

use std::env;
use std::fs;

enum FileType {
    Class,
    // If `main_class` is `None`, use the main class specified in the manifest;
    // if it's `Some`, use that value, regardless of what the manifest specifies
    Jar { main_class: Option<String> },
    Unknown,
}

fn get_main_class_from_manifest(manifest_data: Vec<u8>) -> Option<String> {
    let stringified_data = String::from_utf8_lossy(&manifest_data);

    let headers = stringified_data.split("\r\n");

    let mut main_class = None;
    for header in headers {
        let split_once = header.split_once(": ");
        if let Some((before, after)) = split_once {
            if before == "Main-Class" {
                main_class = Some(after.to_string());
            }
        }
    }

    // Main class is specified with `.` delimiter, but we want `/` delimiter
    main_class.map(|c| c.replace('.', "/").to_string())
}

struct PassedOptions {
    file_type: FileType,
    file_name: String,

    program_args: Vec<String>,
}

impl PassedOptions {
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        if args.len() < 2 {
            return Err(format!("Run as {} [file.class]", args[0]));
        }

        let mut file_type = FileType::Unknown;
        let mut file_name = String::new();
        let mut program_args = Vec::new();

        let mut started_args = false;

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];

            if started_args {
                program_args.push(arg.clone());
            } else {
                // We haven't started receiving program arguments yet; we still
                // have to handle the file being loaded
                if arg == "--jar" {
                    if i + 1 > args.len() {
                        return Err("--jar flag requires a file".to_string());
                    }

                    file_type = FileType::Jar { main_class: None };
                    file_name = args[i + 1].clone();

                    i += 1;

                    started_args = true;
                } else if arg == "--jar-with-main" {
                    if i + 2 > args.len() {
                        return Err("--jar-with-main flag requires a file".to_string());
                    }

                    // Main class is specified with `.` delimiter, but we want
                    // `/` delimiter
                    let main_class = args[i + 2].replace('.', "/").to_string();

                    file_type = FileType::Jar {
                        main_class: Some(main_class),
                    };
                    file_name = args[i + 1].clone();

                    i += 2;

                    started_args = true;
                } else {
                    file_type = FileType::Class;
                    file_name = arg.clone();

                    started_args = true;
                }
            }

            i += 1;
        }

        Ok(Self {
            file_type,
            file_name,
            program_args,
        })
    }
}

fn init_main_class(
    context: Context,
    options: PassedOptions,
    read_file: Vec<u8>,
) -> Result<Class, &'static str> {
    match options.file_type {
        FileType::Class => {
            let class_file = ClassFile::from_data(context.gc_ctx, read_file).unwrap();

            let main_class =
                Class::from_class_file(context, ResourceLoadType::FileSystem, class_file)
                    .expect("Failed to load main class");

            context.register_class(main_class);

            main_class
                .load_methods(context)
                .expect("Failed to load main class method data");

            Ok(main_class)
        }
        FileType::Jar { main_class } => {
            let manifest_name = "META-INF/MANIFEST.MF".to_string();

            let jar_data =
                Jar::from_bytes(context.gc_ctx, read_file).expect("Invalid jar file passed");
            context.add_jar(jar_data);

            let main_class_name = if let Some(main_class) = main_class {
                Some(main_class)
            } else {
                let has_manifest = jar_data.has_file(&manifest_name);
                if !has_manifest {
                    return Err("Cannot execute JAR file without MANIFEST.MF file");
                }

                let manifest_data = jar_data
                    .read_file(&manifest_name)
                    .expect("MANIFEST should read");

                get_main_class_from_manifest(manifest_data)
            };

            if let Some(main_class_name) = main_class_name {
                let main_class_name = JvmString::new(context.gc_ctx, main_class_name);

                let has_main_class = jar_data.has_class(main_class_name);
                if !has_main_class {
                    return Err("Main class specified was not present in JAR!");
                }

                let main_class_data = jar_data
                    .read_class(main_class_name)
                    .expect("Main class should read");

                let class_file = ClassFile::from_data(context.gc_ctx, main_class_data).unwrap();

                let main_class =
                    Class::from_class_file(context, ResourceLoadType::Jar(jar_data), class_file)
                        .expect("Failed to load main class");

                context.register_class(main_class);

                main_class
                    .load_methods(context)
                    .expect("Failed to load main class method data");

                Ok(main_class)
            } else {
                Err("Cannot execute JAR file without main class specified")
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let options = match PassedOptions::from_args(args) {
        Ok(opts) => opts,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };

    let read_file = match fs::read(&options.file_name) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Error: {}", error.to_string());
            return;
        }
    };

    // Initialize JVM
    let loader = loader_backend::DesktopLoaderBackend::new();
    let context = Context::new(Box::new(loader));

    // Load globals
    let globals_jar = Jar::from_bytes(context.gc_ctx, GLOBALS_JAR.to_vec())
        .expect("Builtin globals should be valid");
    context.add_jar(globals_jar);

    base_native_impl::register_native_mappings(context);
    native_impl::register_native_mappings(context);

    // Load program args
    let mut program_args = Vec::new();
    for arg in &options.program_args {
        let utf16_encoded = arg.encode_utf16().collect::<Vec<_>>();

        let string = context.create_string(&utf16_encoded);

        program_args.push(Some(string));
    }

    // Load the main class from options
    let main_class = match init_main_class(context, options, read_file) {
        Ok(main_class) => main_class,
        Err(error_msg) => {
            eprintln!("{}", error_msg);
            return;
        }
    };

    let string_class = context
        .lookup_class(context.common.java_lang_string)
        .expect("String class should exist");

    let args_array = Value::Object(Some(Object::obj_array(
        context,
        string_class,
        &program_args,
    )));

    // Store this on the stack so that GC doesn't decide to collect it
    context.frame_data.borrow()[0].set(args_array);
    context.frame_index.set(1);

    // Call main method
    let main_name = JvmString::new(context.gc_ctx, "main".to_string());
    let main_descriptor_name = JvmString::new(context.gc_ctx, "([Ljava/lang/String;)V".to_string());

    let main_descriptor = MethodDescriptor::from_string(context.gc_ctx, main_descriptor_name)
        .expect("Valid descriptor");

    let method_idx = main_class
        .static_method_vtable()
        .lookup((main_name, main_descriptor));

    if let Some(method_idx) = method_idx {
        let method = main_class.static_methods()[method_idx];
        let result = method.exec(context, &[args_array]);

        if let Err(error) = result {
            eprintln!("Error while running main: {:?}", error);
        }
    } else {
        eprintln!(
            "Class {} has no `void main(String[] args)` method",
            main_class.dot_name()
        );
    }

    unsafe {
        context.gc_ctx.collect(&context);

        context.gc_ctx.drop();
    }
}
