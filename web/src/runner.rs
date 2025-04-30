use crate::loader_backend;
use crate::native_impl;
use crate::output_to_err;

use rjvm_core::{
    Class, ClassFile, Context, Jar, JvmString, MethodDescriptor, Object, ResourceLoadType, Value,
};
use rjvm_globals::{native_impl as base_native_impl, GLOBALS_JAR};

fn init_main_class(
    context: Context,
    read_file: Vec<u8>,
    is_jar: bool,
) -> Result<Class, &'static str> {
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

        main_class.map(|c| c.replace('.', "/").to_string())
    }

    let main_class = if is_jar {
        let manifest_name = "META-INF/MANIFEST.MF".to_string();

        let jar_data = Jar::from_bytes(context.gc_ctx, read_file).expect("Invalid jar file passed");
        context.add_jar(jar_data);

        let has_manifest = jar_data.has_file(&manifest_name);
        if !has_manifest {
            return Err("Cannot execute JAR file without MANIFEST.MF file");
        }

        let manifest_data = jar_data
            .read_file(&manifest_name)
            .expect("MANIFEST should read");

        let main_class_name = get_main_class_from_manifest(manifest_data);
        let Some(main_class_name) = main_class_name else {
            return Err("Cannot execute JAR file without main class specified");
        };

        let main_class_name = JvmString::new(context.gc_ctx, main_class_name);

        let has_main_class = jar_data.has_class(main_class_name);
        if !has_main_class {
            return Err("Main class specified in MANIFEST.MF was not present in JAR!");
        }

        let main_class_data = jar_data
            .read_class(main_class_name)
            .expect("Main class should read");

        let class_file = ClassFile::from_data(context.gc_ctx, main_class_data).unwrap();

        Class::from_class_file(context, ResourceLoadType::Jar(jar_data), class_file)
            .expect("Failed to load main class")
    } else {
        let class_file = ClassFile::from_data(context.gc_ctx, read_file).unwrap();

        Class::from_class_file(context, ResourceLoadType::FileSystem, class_file)
            .expect("Failed to load main class")
    };

    context.register_class(main_class);

    main_class
        .load_methods(context)
        .expect("Failed to load main class method data");

    Ok(main_class)
}

pub(crate) fn run_file(class_data: &[u8], args: Vec<String>, is_jar: bool) {
    // Initialize JVM
    let loader = loader_backend::WebLoaderBackend::new();
    let context = Context::new(Box::new(loader));

    // Load globals
    let globals_jar = Jar::from_bytes(context.gc_ctx, GLOBALS_JAR.to_vec())
        .expect("Builtin globals should be valid");
    context.add_jar(globals_jar);

    base_native_impl::register_native_mappings(context);
    native_impl::register_native_mappings(context);

    // Load the main class from options
    let main_class = match init_main_class(context, class_data.to_vec(), is_jar) {
        Ok(class) => class,
        Err(error) => {
            output_to_err(&format!("Error: {}\n", error));

            return;
        }
    };

    let string_class = context
        .lookup_class(context.common.java_lang_string)
        .expect("String class should exist");

    // Load program args
    let mut program_args = Vec::new();
    for arg in args {
        let utf16_encoded = arg.encode_utf16().collect::<Vec<_>>();

        let string = context.create_string(&utf16_encoded);

        program_args.push(Some(string));
    }

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
            output_to_err(&format!("Error while running main: {:?}\n", error));
        }
    } else {
        output_to_err(&format!(
            "Class {} has no `void main(String[] args)` method\n",
            main_class.dot_name(),
        ));
    }

    unsafe {
        context.gc_ctx.collect(&context);

        context.gc_ctx.drop();
    }
}
