use alloc::boxed::Box;
use rjvm_core::{ClassLoader, Context, Error, JvmString, NativeMethod, Object, Value};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/ClassLoader.loadClassNative.(Ljava/lang/String;)Ljava/lang/Class;", load_class_native),

        ("jvm/internal/ClassLoaderUtils.makePlatformLoader.(Ljava/lang/ClassLoader;)V", make_platform_loader),
        ("jvm/internal/ClassLoaderUtils.makeSystemLoader.(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V", make_sys_loader),
        ("jvm/internal/SystemClassLoader.getResourceData.(Ljava/lang/String;)[B", get_resource_data),
    ];

    context.register_native_mappings(mappings);
}

fn load_class_native(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_loader_obj = args[0].object().unwrap();
    let class_loader_id = class_loader_obj.get_field(0).int();
    let class_loader = context.class_loader_object_by_id(class_loader_id);

    // Second argument should never be null
    let class_name = args[1].object().unwrap();
    let class_name = Context::string_object_to_string(class_name);

    // FIXME fix this- we need to make sure `/` doesn't work as a delimiter somehow
    let class_name = class_name.replace('/', "*");
    // Make `.`s `/`s
    let class_name = class_name.replace('.', "/");
    let class_name = JvmString::new(context.gc_ctx, class_name);

    let class = class_loader.lookup_class(context, class_name);

    if let Ok(class) = class {
        class.run_clinit(context)?;

        let class = class.get_or_init_object(context);
        Ok(Some(Value::Object(Some(class))))
    } else {
        // If the class doesn't exist, we return `null`. Java code will throw
        // a `ClassNotFoundException`.
        Ok(Some(Value::Object(None)))
    }
}

fn make_platform_loader(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let class_loader_obj = args[0].object().unwrap();

    // We are passed an invalid loader object and are supposed to make it
    // into a valid loader, the platform loader
    let loader = ClassLoader::with_parent(
        context.gc_ctx,
        context.bootstrap_loader(),
        class_loader_obj,
        context.loader_backend,
    );

    let id = context.add_class_loader_object(loader);
    class_loader_obj.set_field(0, Value::Integer(id));

    Ok(None)
}

fn make_sys_loader(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Args should never be null
    let platform_loader_obj = args[0].object().unwrap();
    let platform_loader_id = platform_loader_obj.get_field(0).int();
    let platform_loader = context.class_loader_object_by_id(platform_loader_id);

    let class_loader_obj = args[1].object().unwrap();

    // We are passed an invalid loader object and are supposed to make it
    // into a valid loader, the system loader
    let loader = ClassLoader::with_parent(
        context.gc_ctx,
        platform_loader,
        class_loader_obj,
        context.loader_backend,
    );

    let id = context.add_class_loader_object(loader);
    class_loader_obj.set_field(0, Value::Integer(id));

    context.init_system_loader(loader);

    Ok(None)
}

// jvm/internal/SysClassLoader : byte[] getResourceData(String)
fn get_resource_data(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_loader_obj = args[0].object().unwrap();
    let class_loader_id = class_loader_obj.get_field(0).int();
    let class_loader = context.class_loader_object_by_id(class_loader_id);

    // First argument should never be null
    let resource_name = args[1].object().unwrap();
    let resource_name = Context::string_object_to_string(resource_name);

    let resource_data = class_loader.load_resource(&resource_name);

    if let Some(resource_data) = resource_data {
        let resource_data = resource_data.iter().map(|d| *d as i8).collect::<Box<_>>();
        let resource_bytes = Object::byte_array(context, resource_data);

        Ok(Some(Value::Object(Some(resource_bytes))))
    } else {
        Ok(Some(Value::Object(None)))
    }
}
