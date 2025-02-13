use rjvm_core::{
    Class, ClassFile, Context, Jar, JvmString, MethodDescriptor, Object, ResourceLoadType,
    ResourceLoader, Value,
};
use rjvm_globals::GLOBALS_JAR;

struct WebResourceLoader {}

impl ResourceLoader for WebResourceLoader {
    fn load_resource(
        &self,
        load_type: &ResourceLoadType,
        class_name: &String,
        resource_name: &String,
    ) -> Option<Vec<u8>> {
        match load_type {
            ResourceLoadType::FileSystem => todo!(),
            ResourceLoadType::Jar(jar) => {
                let resolved_name = if let Some(absolute_path) = class_name.strip_prefix('/') {
                    // TODO do absolute paths actually work?
                    absolute_path.to_string()
                } else {
                    // TODO should this handle paths starting with "./", maybe?
                    let mut path_sections = class_name.split('/').collect::<Vec<_>>();
                    path_sections.pop();
                    path_sections.push(resource_name);

                    path_sections.join("/")
                };

                if jar.has_file(&resolved_name) {
                    jar.read_file(&resolved_name).ok()
                } else {
                    None
                }
            }
        }
    }
}

fn init_main_class(context: Context, read_file: Vec<u8>) -> Class {
    let class_file = ClassFile::from_data(context.gc_ctx, read_file).unwrap();

    let main_class =
        Class::from_class_file(context, ResourceLoadType::FileSystem, class_file)
            .expect("Failed to load main class");

    context.register_class(main_class);

    main_class
        .load_methods(context)
        .expect("Failed to load main class method data");

    main_class
}

pub(crate) fn run_file(class_data: &[u8]) {
    // Initialize JVM
    let loader = WebResourceLoader {};
    let context = Context::new(Box::new(loader));

    // Load globals
    let globals_jar = Jar::from_bytes(context.gc_ctx, GLOBALS_JAR.to_vec())
        .expect("Builtin globals should be valid");
    context.add_jar(globals_jar);

    crate::native_impl::register_native_mappings(context);

    // Load the main class from options
    let main_class = init_main_class(context, class_data.to_vec());

    let string_class = context
        .lookup_class(context.common.java_lang_string)
        .expect("String class should exist");

    // TODO get args somehow?
    let args_array = Value::Object(Some(Object::obj_array(
        context,
        string_class,
        &[],
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
