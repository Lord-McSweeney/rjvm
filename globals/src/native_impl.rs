use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::Cell;
use rjvm_core::{
    Array, ClassLoader, Context, Error, JvmString, NativeMethod, Object, PrimitiveType,
    ResolvedDescriptor, Value,
};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/nio/charset/Charset.stringToUtf8.(Ljava/lang/String;)[B", string_to_utf8),
        ("java/lang/System.arraycopy.(Ljava/lang/Object;ILjava/lang/Object;II)V", array_copy),
        ("java/lang/Class.isArray.()Z", is_array),
        ("java/lang/Class.isInterface.()Z", is_interface),
        ("java/lang/Class.isPrimitive.()Z", is_primitive),
        ("java/lang/Object.getClass.()Ljava/lang/Class;", get_class),
        ("java/lang/Class.getNameNative.()Ljava/lang/String;", get_name_native),
        ("java/lang/Math.atan2.(DD)D", math_atan2),
        ("java/lang/Math.floor.(D)D", math_floor),
        ("java/lang/Math.log.(D)D", math_log),
        ("java/lang/Math.pow.(DD)D", math_pow),
        ("java/lang/Math.sqrt.(D)D", math_sqrt),
        ("java/lang/Object.clone.()Ljava/lang/Object;", object_clone),
        ("java/lang/Throwable.internalFillInStackTrace.()[Ljava/lang/StackTraceElement;", capture_stack_trace),
        ("java/lang/Class.getPrimitiveClass.(I)Ljava/lang/Class;", get_primitive_class),
        ("java/lang/Object.hashCode.()I", object_hash_code),
        ("java/lang/ClassLoader.loadClassNative.(Ljava/lang/String;)Ljava/lang/Class;", load_class_native),
        ("java/lang/Class.getConstructors.()[Ljava/lang/reflect/Constructor;", get_constructors),
        ("java/lang/reflect/Constructor.newInstanceNative.([Ljava/lang/Object;)Ljava/lang/Object;", new_instance_native),
        ("java/lang/reflect/Constructor.getParameterCount.()I", exec_get_parameter_count),
        ("java/lang/reflect/Method.getParameterCount.()I", exec_get_parameter_count),
        ("java/lang/String.intern.()Ljava/lang/String;", string_intern),
        ("java/lang/Double.doubleToRawLongBits.(D)J", double_to_raw_long_bits),
        ("java/lang/Double.toString.(D)Ljava/lang/String;", double_to_string),
        ("java/lang/Class.getClassLoader.()Ljava/lang/ClassLoader;", class_get_class_loader),
        ("java/lang/Class.getComponentType.()Ljava/lang/Class;", class_get_component_type),
        ("java/lang/Class.getSuperclass.()Ljava/lang/Class;", class_get_superclass),
        ("java/lang/Class.getMethodNative.(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;", class_get_method),
        ("java/lang/System.identityHashCode.(Ljava/lang/Object;)I", identity_hash_code),
        ("java/lang/reflect/Constructor.getDeclaringClass.()Ljava/lang/Class;", exec_get_declaring_class),
        ("java/lang/reflect/Method.getDeclaringClass.()Ljava/lang/Class;", exec_get_declaring_class),
        ("java/lang/reflect/Method.getName.()Ljava/lang/String;", method_get_name),
        ("java/lang/Class.isInstance.(Ljava/lang/Object;)Z", class_is_instance),

        ("jvm/internal/ClassLoaderUtils.makePlatformLoader.(Ljava/lang/ClassLoader;)V", make_platform_loader),
        ("jvm/internal/ClassLoaderUtils.makeSystemLoader.(Ljava/lang/ClassLoader;Ljava/lang/ClassLoader;)V", make_sys_loader),
        ("jvm/internal/SystemClassLoader.getResourceData.(Ljava/lang/String;)[B", get_resource_data),
    ];

    context.register_native_mappings(mappings);
}

// Native implementations of functions declared in globals

// java/lang/PrintStream : static byte[] stringToUtf8(String)
fn string_to_utf8(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting non-null object
    let string_object = args[0].object().unwrap();
    let string = Context::string_object_to_string(string_object);

    let bytes = string.as_bytes();
    let bytes = bytes.iter().copied().map(|b| b as i8).collect::<Box<_>>();

    let byte_array = Object::byte_array(context, bytes);

    Ok(Some(Value::Object(Some(byte_array))))
}

// java/lang/System: static void arraycopy(Object, int, Object, int, int)
fn array_copy(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let source_arr = args[0].object();
    let Some(source_arr) = source_arr else {
        return Err(context.null_pointer_exception());
    };

    let source_start = args[1].int();

    let dest_arr = args[2].object();
    let Some(dest_arr) = dest_arr else {
        return Err(context.null_pointer_exception());
    };

    let dest_start = args[3].int();

    let length = args[4].int();

    if source_start < 0 || dest_start < 0 || length < 0 {
        return Err(context.array_index_oob_exception());
    }

    let source_start = source_start as usize;
    let dest_start = dest_start as usize;
    let length = length as usize;

    let (Some(_), Some(dest_value_type)) = (
        source_arr.class().array_value_type(),
        dest_arr.class().array_value_type(),
    ) else {
        return Err(context.array_store_exception());
    };

    let source_array_data = source_arr.array_data();
    let dest_array_data = dest_arr.array_data();

    if source_start + length > source_array_data.len()
        || dest_start + length > dest_array_data.len()
    {
        return Err(context.array_index_oob_exception());
    }

    match (source_array_data, dest_array_data) {
        (Array::ByteArray(source_data), Array::ByteArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::CharArray(source_data), Array::CharArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::DoubleArray(source_data), Array::DoubleArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::FloatArray(source_data), Array::FloatArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::IntArray(source_data), Array::IntArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::LongArray(source_data), Array::LongArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::ShortArray(source_data), Array::ShortArray(dest_data)) => {
            primitive_array_copy::<_>(&source_data, &dest_data, source_start, dest_start, length);
        }
        (Array::ObjectArray(source_data), Array::ObjectArray(dest_data)) => {
            let Some(dest_value_class) = dest_value_type.class() else {
                unreachable!()
            };

            let mut temp_arr = Vec::with_capacity(length);

            for i in 0..length {
                let source_idx = source_start + i;
                temp_arr.push(source_data[source_idx].get());
            }

            for i in 0..length {
                let obj = temp_arr[i];
                if let Some(obj) = obj {
                    if !obj.class().check_cast(dest_value_class) {
                        return Err(context.array_store_exception());
                    }
                }

                let dest_idx = dest_start + i;
                dest_data[dest_idx].set(obj);
            }
        }
        (_, _) => {
            return Err(context.array_store_exception());
        }
    }

    Ok(None)
}

#[inline(never)]
fn primitive_array_copy<T: Copy + Default>(
    source_data: &Box<[Cell<T>]>,
    dest_data: &Box<[Cell<T>]>,
    source_start: usize,
    dest_start: usize,
    length: usize,
) {
    #[inline(never)]
    fn copy_nonoverlapping<T: Copy + Default>(
        source_data: &[Cell<T>],
        dest_data: &[Cell<T>],
        source_start: usize,
        dest_start: usize,
        length: usize,
    ) {
        // TODO optimize this

        for i in 0..length {
            let source_idx = source_start + i;
            let dest_idx = dest_start + i;
            dest_data[dest_idx].set(source_data[source_idx].get());
        }
    }

    #[inline(never)]
    fn copy_overlapping<T: Copy + Default>(
        source_data: &[Cell<T>],
        dest_data: &[Cell<T>],
        source_start: usize,
        dest_start: usize,
        length: usize,
    ) {
        // TODO: Can we avoid the temporary allocation?

        let temp_arr = vec![Cell::new(T::default()); length];

        for i in 0..length {
            let source_idx = source_start + i;
            temp_arr[i].set(source_data[source_idx].get());
        }

        for i in 0..length {
            let dest_idx = dest_start + i;
            dest_data[dest_idx].set(temp_arr[i].get());
        }
    }

    let overlapping = if core::ptr::eq(&**source_data, &**dest_data) {
        let dst_start_in_source = source_start <= dest_start && source_start + length > dest_start;
        let src_start_in_dest = dest_start <= source_start && dest_start + length > source_start;

        dst_start_in_source || src_start_in_dest
    } else {
        // Not the same array, can't be overlapping
        false
    };

    if overlapping {
        copy_overlapping(source_data, dest_data, source_start, dest_start, length);
    } else {
        copy_nonoverlapping(source_data, dest_data, source_start, dest_start, length);
    }
}

// java/lang/Class : boolean isArray()
fn is_array(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    if class.array_value_type().is_some() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

// java/lang/Class : boolean isInterface()
fn is_interface(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    if class.is_interface() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
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

// java/lang/Object : Class getClass()
fn get_class(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().class();

    let class_object = class.get_or_init_object(context);

    Ok(Some(Value::Object(Some(class_object))))
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

fn math_atan2(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let y = args[0].double();
    let x = args[2].double();

    // TODO docs say this has some special-cases

    Ok(Some(Value::Double(libm::atan2(y, x))))
}

fn math_floor(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(libm::floor(value))))
}

fn math_log(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(libm::log(value))))
}

fn math_pow(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let base = args[0].double();
    let exp = args[2].double();

    Ok(Some(Value::Double(libm::pow(base, exp))))
}

fn math_sqrt(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(libm::sqrt(value))))
}

fn object_clone(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let this = args[0].object().unwrap();
    let this_class = this.class();

    let cloneable_iface = context.builtins().java_lang_cloneable;
    let implements_cloneable = this_class.implements_interface(cloneable_iface);

    if implements_cloneable || this_class.array_value_type().is_some() {
        let cloned_object = this.create_clone(context.gc_ctx);

        Ok(Some(Value::Object(Some(cloned_object))))
    } else {
        Err(context.clone_not_supported_exception())
    }
}

fn capture_stack_trace(context: &Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    // Skip the first two entries because they are
    // `Throwable.internalFillInStackTrace` and `Throwable.fillInStackTrace`
    let array = context.format_call_stack(2);

    Ok(Some(Value::Object(Some(array))))
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

fn object_hash_code(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let this = args[0].object().unwrap();

    let result = crate::hash_code::calc_hash_code(this);

    Ok(Some(Value::Integer(result)))
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

fn get_constructors(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class_id = class_obj.get_field(0).int();
    let class = context.class_object_by_id(class_id);

    let constructors = crate::reflect::constructors_for_class(context, class);
    let constructors_arr = constructors
        .iter()
        .map(|m| Some(m.get_or_init_object(context)))
        .collect::<Box<_>>();

    let constructor_class = context.builtins().java_lang_reflect_constructor;
    let constructors_arr = Object::obj_array(context, constructor_class, constructors_arr);

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
        crate::reflect::args_for_instance_call(context, ctor_method, instance, &args_array)?;

    if let Err(e) = context.exec_method(ctor_method, &real_args) {
        // FIXME this should throw `InvocationTargetException`
        Err(e)
    } else {
        Ok(Some(Value::Object(Some(instance))))
    }
}

fn exec_get_parameter_count(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let exec_obj = args[0].object().unwrap();
    let exec_id = exec_obj.get_field(0).int();
    let method = context.executable_object_by_id(exec_id);

    Ok(Some(Value::Integer(method.arg_count() as i32)))
}

fn string_intern(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let string_obj = args[0].object().unwrap();

    let interned = context.intern_string_obj(string_obj);

    Ok(Some(Value::Object(Some(interned))))
}

fn double_to_raw_long_bits(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let double = args[0].double();
    let bits = f64::to_bits(double);

    Ok(Some(Value::Long(bits as i64)))
}

fn double_to_string(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let double = args[0].double();

    let string = if double.is_infinite() {
        if double < 0.0 {
            "-Infinity".to_string()
        } else {
            "Infinity".to_string()
        }
    } else {
        let string = format!("{:.6}", double);

        let mut string = string.trim_end_matches('0').to_string();
        if string.ends_with('.') {
            string.push('0');
        }

        string
    };

    let chars = string.chars().map(|c| c as u16).collect::<Box<_>>();

    Ok(Some(Value::Object(Some(context.create_string(&chars)))))
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

fn identity_hash_code(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let object = args[0].object();

    // `identityHashCode(null)` is `0`
    let result = object.map(crate::hash_code::calc_hash_code).unwrap_or(0);

    Ok(Some(Value::Integer(result)))
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
