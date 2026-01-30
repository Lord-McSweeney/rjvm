use crate::loader_backend;
use crate::native_impl;
use crate::output_to_err;

use rjvm_core::{
    Class, Context, Jar, JvmString, MethodDescriptor, Object, ResourceLoadSource, Value,
};
use rjvm_globals::{GLOBALS_BASE_JAR, GLOBALS_DESKTOP_JAR, native_impl as base_native_impl};

fn init_main_class(
    context: &Context,
    class_name: &str,
    read_file: Vec<u8>,
    is_jar: bool,
) -> Result<Class, String> {
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

    let main_class_name = if is_jar {
        let manifest_name = "META-INF/MANIFEST.MF".to_string();

        let jar_data = Jar::from_bytes(context.gc_ctx, read_file).expect("Invalid jar file passed");
        context.add_system_jar(jar_data);

        let has_manifest = jar_data.has_file(&manifest_name);
        if !has_manifest {
            return Err("Cannot execute JAR file without MANIFEST.MF file".to_string());
        }

        let manifest_data = jar_data
            .read_file(manifest_name)
            .expect("MANIFEST should read");

        let main_class_name = get_main_class_from_manifest(manifest_data);
        let Some(main_class_name) = main_class_name else {
            return Err("Cannot execute JAR file without main class specified".to_string());
        };

        let main_class_name = JvmString::new(context.gc_ctx, main_class_name);

        let has_main_class = jar_data.has_class(main_class_name);
        if !has_main_class {
            return Err("Main class specified in MANIFEST.MF was not present in JAR!".to_string());
        }

        main_class_name
    } else {
        context
            .system_loader()
            .add_source(ResourceLoadSource::FileSystem);

        let class_name = class_name.strip_suffix(".class").unwrap_or(class_name);

        JvmString::new(context.gc_ctx, class_name.to_string())
    };

    context
        .system_loader()
        .lookup_class(context, main_class_name)
        .map_err(|e| e.display(context))
}

pub(crate) fn run_file(class_name: &str, class_data: &[u8], args: Vec<String>) {
    let is_jar = class_name.ends_with(".jar");

    // Initialize JVM
    let loader = loader_backend::WebLoaderBackend::new(class_name, class_data);
    let context = Context::new(Box::new(loader));

    // Load globals
    let globals_base_jar = Jar::from_bytes(context.gc_ctx, GLOBALS_BASE_JAR.to_vec())
        .expect("Builtin globals should be valid");
    context.add_bootstrap_jar(globals_base_jar);
    let globals_desktop_jar = Jar::from_bytes(context.gc_ctx, GLOBALS_DESKTOP_JAR.to_vec())
        .expect("Builtin globals should be valid");
    context.add_bootstrap_jar(globals_desktop_jar);

    base_native_impl::register_native_mappings(&context);
    native_impl::register_native_mappings(&context);

    context.load_builtins();

    // Load the main class from options
    let main_class = match init_main_class(&context, class_name, class_data.to_vec(), is_jar) {
        Ok(class) => class,
        Err(error) => {
            output_to_err(&format!("Error: {}", error));

            return;
        }
    };

    // Load program args
    let mut program_args = Vec::new();
    for arg in args {
        let utf16_encoded = arg.encode_utf16().collect::<Vec<_>>();

        let string = context.create_string(&utf16_encoded);

        program_args.push(Some(string));
    }

    let string_class = context.builtins().java_lang_string;
    let args_array = Value::Object(Some(Object::obj_array(
        &context,
        string_class,
        program_args.into_boxed_slice(),
    )));

    // Call main method
    let main_name = JvmString::new(context.gc_ctx, "main".to_string());
    let main_descriptor_name = JvmString::new(context.gc_ctx, "([Ljava/lang/String;)V".to_string());

    let main_descriptor =
        MethodDescriptor::from_string(&context, main_descriptor_name).expect("Valid descriptor");

    let method_idx = main_class
        .static_method_vtable()
        .lookup((main_name, main_descriptor));

    if let Some(method_idx) = method_idx {
        let method = main_class.static_methods()[method_idx];
        let result = context.exec_method(method, &[args_array]);

        if let Err(error) = result {
            output_to_err(&format!(
                "Error while running main: {}",
                error.display(&context)
            ));
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
