use alloc::boxed::Box;
use alloc::vec::Vec;
use rjvm_core::{
    Context, Descriptor, Error, JvmString, MethodFlags, NativeMethod, Object, PrimitiveType,
    ResolvedDescriptor, Value,
};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/Class.isPrimitive.()Z", is_primitive),
        ("java/lang/Class.getNameNative.()Ljava/lang/String;", get_name_native),
        ("java/lang/Class.getPrimitiveClass.(I)Ljava/lang/Class;", get_primitive_class),
        ("java/lang/Class.getConstructors.()[Ljava/lang/reflect/Constructor;", get_constructors),
        ("java/lang/Class.getDeclaredConstructors.()[Ljava/lang/reflect/Constructor;", get_declared_constructors),
        ("java/lang/reflect/Constructor.newInstanceNative.([Ljava/lang/Object;)Ljava/lang/Object;", new_instance_native),
        ("java/lang/reflect/Constructor.getParameterTypes.()[Ljava/lang/Class;", exec_get_parameter_types),
        ("java/lang/reflect/Method.getParameterTypes.()[Ljava/lang/Class;", exec_get_parameter_types),
        ("java/lang/Class.getClassLoader.()Ljava/lang/ClassLoader;", class_get_class_loader),
        ("java/lang/Class.getComponentType.()Ljava/lang/Class;", class_get_component_type),
        ("java/lang/Class.getSuperclass.()Ljava/lang/Class;", class_get_superclass),
        ("java/lang/Class.getMethodNative.(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", class_get_method),
        ("java/lang/reflect/Constructor.getDeclaringClass.()Ljava/lang/Class;", exec_get_declaring_class),
        ("java/lang/reflect/Method.getDeclaringClass.()Ljava/lang/Class;", exec_get_declaring_class),
        ("java/lang/reflect/Method.getName.()Ljava/lang/String;", method_get_name),
        ("java/lang/Class.isInstance.(Ljava/lang/Object;)Z", class_is_instance),
        ("java/lang/Class.getModifiers.()I", class_get_modifiers),
        ("java/lang/reflect/Method.invokeNative.(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;", invoke_native),
        ("java/lang/Class.getDeclaredMethods.()[Ljava/lang/reflect/Method;", get_declared_methods),
        ("java/lang/Class.getInterfaces.()[Ljava/lang/Class;", class_get_interfaces),
        ("java/lang/Class.getDeclaringClass.()Ljava/lang/Class;", class_get_declaring_class),
    ];

    context.register_native_mappings(mappings);
}

// java/lang/Class : boolean isPrimitive()
fn is_primitive(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    if class.is_primitive() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

// java/lang/Class : String getNameNative()
fn get_name_native(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let string_chars = class.dot_name().encode_utf16().collect::<Vec<_>>();

    Ok(Some(Value::Object(Some(
        context.create_string(&string_chars),
    ))))
}

fn get_primitive_class(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let id = args[0].int();
    let primitive_type = match id {
        0 => PrimitiveType::Boolean,
        1 => PrimitiveType::Byte,
        2 => PrimitiveType::Char,
        3 => PrimitiveType::Short,
        4 => PrimitiveType::Int,
        5 => PrimitiveType::Long,
        6 => PrimitiveType::Float,
        7 => PrimitiveType::Double,
        8 => PrimitiveType::Void,
        _ => panic!("Called getPrimitiveClass with invalid arg"),
    };

    let class = context.primitive_class_for(primitive_type);
    let class_obj = class.get_or_init_object(context);

    Ok(Some(Value::Object(Some(class_obj))))
}

fn get_constructors(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let constructors_arr = class
        .instance_method_vtable()
        .elements_for_name(context.common.init_name)
        .iter()
        // Make sure we're only picking up public initializers defined in this class
        .filter(|m| m.class() == class && m.flags().contains(MethodFlags::PUBLIC))
        .map(|m| Some(m.get_or_init_object(context)))
        .collect::<Box<_>>();

    let constructors_arr = Object::obj_array(
        context,
        context.builtins().java_lang_reflect_constructor,
        constructors_arr,
    );

    Ok(Some(Value::Object(Some(constructors_arr))))
}

fn get_declared_constructors(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let constructors_arr = class
        .instance_method_vtable()
        .elements_for_name(context.common.init_name)
        .iter()
        // Make sure we're only picking up initializers defined in this class
        .filter(|m| m.class() == class)
        .map(|m| Some(m.get_or_init_object(context)))
        .collect::<Box<_>>();

    let constructors_arr = Object::obj_array(
        context,
        context.builtins().java_lang_reflect_constructor,
        constructors_arr,
    );

    Ok(Some(Value::Object(Some(constructors_arr))))
}

fn new_instance_native(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let ctor_obj = args[0].object().unwrap();
    let ctor_id = ctor_obj.get_field(0).int();
    let ctor_method = context.executable_object_by_id(ctor_id);

    let raw_args = args[1].object().unwrap();
    let raw_args = raw_args.array_data().as_object_array();

    let length = raw_args.len();
    let mut args_array = Vec::with_capacity(length);
    for i in 0..length {
        args_array.push(Value::Object(raw_args[i].get()));
    }

    if ctor_method.class().cant_instantiate() {
        return Err(context.instantiation_exception());
    }

    let instance = Object::from_class(context.gc_ctx, ctor_method.class());

    let real_args =
        crate::reflect::args_for_instance_call(context, ctor_method, Some(instance), &args_array)?;

    if let Err(e) = context.exec_method(ctor_method, &real_args) {
        // FIXME this should throw `InvocationTargetException`
        Err(e)
    } else {
        Ok(Some(Value::Object(Some(instance))))
    }
}

fn exec_get_parameter_types(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let exec_obj = args[0].object().unwrap();
    let exec_id = exec_obj.get_field(0).int();
    let method = context.executable_object_by_id(exec_id);

    let descriptor = method.get_or_init_resolved_descriptor(context)?;
    let params = descriptor.args();

    let resulting_classes = params
        .iter()
        .map(|p| p.reflection_class(context.gc_ctx))
        .map(|c| Some(c.get_or_init_object(context)))
        .collect::<Box<[_]>>();

    let created_array = Object::obj_array(
        context,
        context.builtins().java_lang_class,
        resulting_classes,
    );

    Ok(Some(Value::Object(Some(created_array))))
}

fn class_get_class_loader(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let Some(class_loader) = class.loader() else {
        return Ok(Some(Value::Object(None)));
    };

    let class_loader_object = class_loader.object();

    Ok(Some(Value::Object(class_loader_object)))
}

fn class_get_component_type(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let Some(component_type) = class.array_value_type() else {
        // Return `null` if this class isn't an array type
        return Ok(Some(Value::Object(None)));
    };

    let class = match component_type {
        ResolvedDescriptor::Class(class) | ResolvedDescriptor::Array(class) => class,
        ResolvedDescriptor::Boolean => context.primitive_class_for(PrimitiveType::Boolean),
        ResolvedDescriptor::Byte => context.primitive_class_for(PrimitiveType::Byte),
        ResolvedDescriptor::Character => context.primitive_class_for(PrimitiveType::Char),
        ResolvedDescriptor::Double => context.primitive_class_for(PrimitiveType::Double),
        ResolvedDescriptor::Float => context.primitive_class_for(PrimitiveType::Float),
        ResolvedDescriptor::Integer => context.primitive_class_for(PrimitiveType::Int),
        ResolvedDescriptor::Long => context.primitive_class_for(PrimitiveType::Long),
        ResolvedDescriptor::Short => context.primitive_class_for(PrimitiveType::Short),
        ResolvedDescriptor::Void => unreachable!(),
    };

    let class_object = class.get_or_init_object(context);

    Ok(Some(Value::Object(Some(class_object))))
}

fn class_get_superclass(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    // Special-case: `getSuperclass` on an interface class always returns `null`
    // (see JDK-1262086)
    if class.is_interface() {
        return Ok(Some(Value::Object(None)));
    }

    let Some(super_class) = class.super_class() else {
        // Return `null` if this class has no superclass
        return Ok(Some(Value::Object(None)));
    };

    let class_object = super_class.get_or_init_object(context);

    Ok(Some(Value::Object(Some(class_object))))
}

fn class_get_method(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let method_name = args[1].object().unwrap();
    let method_name = Context::string_object_to_string(method_name);
    let method_name = JvmString::new(context.gc_ctx, method_name);

    let arg_classes = args[2].object().unwrap();
    let arg_classes = arg_classes.array_data().as_object_array();

    let mut arg_descriptors = Vec::with_capacity(arg_classes.len());
    for arg_class in arg_classes {
        let arg_class = arg_class.get();

        // `null` args always result in `NoSuchMethodFoundException`
        let Some(arg_class_obj) = arg_class else {
            return Ok(Some(Value::Object(None)));
        };

        let arg_class_id = arg_class_obj.get_field(0).int();
        let arg_class = context.class_object_by_id(arg_class_id);

        arg_descriptors.push(crate::reflect::descriptor_for_class(context, arg_class));
    }

    let method = crate::reflect::get_class_method(class, method_name, &arg_descriptors);

    if let Some(method) = method {
        let method_object = method.get_or_init_object(context);
        Ok(Some(Value::Object(Some(method_object))))
    } else {
        Ok(Some(Value::Object(None)))
    }
}

fn exec_get_declaring_class(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let exec_obj = args[0].object().unwrap();
    let exec_id = exec_obj.get_field(0).int();
    let method = context.executable_object_by_id(exec_id);

    let class = method.class();
    let class_object = class.get_or_init_object(context);

    Ok(Some(Value::Object(Some(class_object))))
}

fn method_get_name(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let exec_obj = args[0].object().unwrap();
    let exec_id = exec_obj.get_field(0).int();
    let method = context.executable_object_by_id(exec_id);

    let name = method.name();
    let name_object = context.str_to_string(&name);

    Ok(Some(Value::Object(Some(name_object))))
}

fn class_is_instance(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let object = args[1].object();
    if object.is_some_and(|o| o.class().check_cast(class)) {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

fn class_get_modifiers(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    Ok(Some(Value::Integer(class.modifiers() as i32)))
}

fn invoke_native(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let method_obj = args[0].object().unwrap();
    let method_id = method_obj.get_field(0).int();
    let method = context.executable_object_by_id(method_id);

    let receiver_arg = args[1].object();

    let raw_args = args[2].object().unwrap();
    let raw_args = raw_args.array_data().as_object_array();

    let length = raw_args.len();
    let mut args_array = Vec::with_capacity(length + 1);
    for i in 0..length {
        args_array.push(Value::Object(raw_args[i].get()));
    }

    let receiver_arg = if !method.is_static() {
        if let Some(receiver_arg) = receiver_arg {
            // TODO verify that receiver matches class
            Some(receiver_arg)
        } else {
            // Receiver cannot be null
            return Err(context.null_pointer_exception());
        }
    } else {
        None
    };

    let real_args =
        crate::reflect::args_for_instance_call(context, method, receiver_arg, &args_array)?;

    match context.exec_method(method, &real_args) {
        Ok(Some(value)) => {
            let return_type = method.descriptor().return_type();
            match return_type {
                Descriptor::Class(_) | Descriptor::Array(_) => {
                    // This method returns pointer type, just return the value
                    Ok(Some(value))
                }
                Descriptor::Void => {
                    unreachable!("Condition handled below")
                }
                _ => {
                    // TODO implement boxing
                    unimplemented!("Boxing method return value");
                }
            }
        }
        Ok(None) => {
            // Return `null` if this is a void method
            Ok(Some(Value::Object(None)))
        }
        Err(e) => {
            // FIXME this should throw `InvocationTargetException`
            Err(e)
        }
    }
}

fn get_declared_methods(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    if class.array_value_type().is_some() {
        // Array classes have no declared methods
        let created_array = Object::obj_array(
            context,
            context.builtins().java_lang_reflect_method,
            Box::new([]),
        );
        return Ok(Some(Value::Object(Some(created_array))));
    }

    let static_methods = class.static_methods();
    let instance_methods = class.instance_method_vtable();
    let instance_methods = instance_methods.elements();

    let mut result = Vec::with_capacity(static_methods.len() + instance_methods.len());

    for static_method in &*static_methods {
        // clinit method isn't included
        if *static_method.name() != "<clinit>" {
            result.push(*static_method);
        }
    }

    for instance_method in instance_methods {
        // We only use instance methods declared by this class
        if instance_method.class() == class {
            // init methods aren't included
            if *instance_method.name() != "<init>" {
                result.push(*instance_method);
            }
        }
    }

    let result = result
        .iter()
        .map(|m| Some(m.get_or_init_object(context)))
        .collect::<Box<_>>();

    let created_array =
        Object::obj_array(context, context.builtins().java_lang_reflect_method, result);

    Ok(Some(Value::Object(Some(created_array))))
}

fn class_get_interfaces(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let interfaces = class
        .own_interfaces()
        .iter()
        .map(|m| Some(m.get_or_init_object(context)))
        .collect::<Box<_>>();

    let created_array = Object::obj_array(context, context.builtins().java_lang_class, interfaces);

    Ok(Some(Value::Object(Some(created_array))))
}

fn class_get_declaring_class(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let declaring_class = class.find_declaring_class(context)?;

    if let Some(declaring_class) = declaring_class {
        let object = declaring_class.get_or_init_object(context);

        Ok(Some(Value::Object(Some(object))))
    } else {
        Ok(Some(Value::Object(None)))
    }
}
