use rjvm_core::{Class, Context, Error, NativeMethod, Object, PrimitiveType, Value};

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/nio/charset/Charset.stringToUtf8.(Ljava/lang/String;)[B", string_to_utf8),
        ("java/lang/System.arraycopy.(Ljava/lang/Object;ILjava/lang/Object;II)V", array_copy),
        ("java/lang/Class.isInterface.()Z", is_interface),
        ("java/lang/Class.isPrimitive.()Z", is_primitive),
        ("java/lang/Object.getClass.()Ljava/lang/Class;", get_class),
        ("java/lang/Class.getNameNative.()Ljava/lang/String;", get_name_native),
        ("java/lang/Class.getResourceData.(Ljava/lang/String;)[B", get_resource_data),
        ("java/lang/Math.atan2.(DD)D", math_atan2),
        ("java/lang/Math.log.(D)D", math_log),
        ("java/lang/Math.pow.(DD)D", math_pow),
        ("java/lang/Math.sqrt.(D)D", math_sqrt),
        ("java/lang/Object.clone.()Ljava/lang/Object;", object_clone),
        ("java/lang/Throwable.internalFillInStackTrace.()Ljava/lang/String;", capture_stack_trace),
        ("java/lang/Class.getPrimitiveClass.(I)Ljava/lang/Class;", get_primitive_class),
        ("java/lang/Object.hashCode.()I", object_hash_code),
    ];

    context.register_native_mappings(mappings);
}

// Native implementations of functions declared in globals

// java/lang/PrintStream : static byte[] stringToUtf8(String)
fn string_to_utf8(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting non-null object
    let string_object = args[0].object().unwrap();
    let char_array = string_object.get_field(0).object().unwrap();

    let length = char_array.array_length();
    let mut chars_vec = Vec::with_capacity(length);
    for i in 0..length {
        chars_vec.push(char_array.get_char_at_index(i));
    }

    let string = String::from_utf16_lossy(&chars_vec);
    let bytes = string.as_bytes();

    let byte_array = Object::byte_array(context, bytes);

    Ok(Some(Value::Object(Some(byte_array))))
}

// java/lang/System: static void arraycopy(Object, int, Object, int, int)
fn array_copy(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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

    if source_start + length > source_arr.array_length()
        || dest_start + length > dest_arr.array_length()
    {
        return Err(context.array_index_oob_exception());
    }

    let (Some(source_value_type), Some(dest_value_type)) = (
        source_arr.class().array_value_type(),
        dest_arr.class().array_value_type(),
    ) else {
        return Err(context.array_store_exception());
    };

    if source_value_type != dest_value_type {
        // Only throw if either of them is a primitive type; if both are object types,
        // the type-checking will be in the actual copy loop.
        if source_value_type.is_primitive() || dest_value_type.is_primitive() {
            return Err(context.array_store_exception());
        }
    }

    let source_data = source_arr.get_array_data();
    let dest_data = dest_arr.get_array_data();

    let mut temp_arr = Vec::with_capacity(length);

    for i in 0..length {
        let source_idx = source_start + i;
        temp_arr.push(source_data[source_idx].get());
    }

    for i in 0..length {
        let value = temp_arr[i];
        if let Value::Object(obj) = value {
            if let Some(obj) = obj {
                if !obj.class().matches_descriptor(dest_value_type) {
                    return Err(context.array_store_exception());
                }
            }
        }

        let dest_idx = dest_start + i;
        dest_data[dest_idx].set(value);
    }

    Ok(None)
}

// java/lang/Class : boolean isInterface()
fn is_interface(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class = context.get_class_for_java_class(class_obj);

    if class.is_interface() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

// java/lang/Class : boolean isPrimitive()
fn is_primitive(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class = context.get_class_for_java_class(class_obj);

    if class.is_primitive() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

// java/lang/Object : Class getClass()
fn get_class(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().class();

    let class_object = context.get_or_init_java_class_for_class(class);

    Ok(Some(Value::Object(Some(class_object))))
}

// java/lang/Class : String getNameNative()
fn get_name_native(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class = context.get_class_for_java_class(class_obj);

    let string_chars = class.dot_name().encode_utf16().collect::<Vec<_>>();

    Ok(Some(Value::Object(Some(
        context.create_string(&string_chars),
    ))))
}

// java/lang/Class : byte[] getResourceData(String)
fn get_resource_data(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class_obj = args[0].object().unwrap();
    let class = context.get_class_for_java_class(class_obj);

    // First argument should never be null
    let resource_name_data = args[1].object().unwrap().get_field(0).object().unwrap();

    let length = resource_name_data.array_length();
    let mut chars_vec = Vec::with_capacity(length);
    for i in 0..length {
        chars_vec.push(resource_name_data.get_char_at_index(i));
    }

    let resource_name = String::from_utf16_lossy(&chars_vec);

    if let Some(resource_data) = class.load_resource(context, &resource_name) {
        let resource_bytes = Object::byte_array(context, &resource_data);

        Ok(Some(Value::Object(Some(resource_bytes))))
    } else {
        Ok(Some(Value::Object(None)))
    }
}

fn math_atan2(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let y = args[0].double();
    let x = args[2].double();

    // TODO docs say this has some special-cases

    Ok(Some(Value::Double(y.atan2(x))))
}

fn math_log(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(value.ln())))
}

fn math_pow(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let base = args[0].double();
    let exp = args[2].double();

    Ok(Some(Value::Double(base.powf(exp))))
}

fn math_sqrt(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let value = args[0].double();

    Ok(Some(Value::Double(value.sqrt())))
}

fn object_clone(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let this = args[0].object().unwrap();
    let this_class = this.class();

    let cloneable_iface = context
        .lookup_class(context.common.java_lang_cloneable)
        .expect("Cloneable class should exist");

    let implements_cloneable = this_class.implements_interface(cloneable_iface);

    if implements_cloneable || this_class.array_value_type().is_some() {
        let cloned_object = this.create_clone(context.gc_ctx);

        Ok(Some(Value::Object(Some(cloned_object))))
    } else {
        Err(context.clone_not_supported_exception())
    }
}

fn capture_stack_trace(context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    let stack_data = context.capture_call_stack();
    let chars = stack_data.chars().map(|c| c as u16).collect::<Vec<_>>();

    Ok(Some(Value::Object(Some(context.create_string(&chars)))))
}

fn get_primitive_class(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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
    let class_obj = context.get_or_init_java_class_for_class(class);

    Ok(Some(Value::Object(Some(class_obj))))
}

fn object_hash_code(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let this = args[0].object().unwrap();
    let this_class = this.class();

    let ptr_obj = Object::as_ptr(this) as usize;
    let ptr_cls = Class::as_ptr(this_class) as usize;

    let mut result = (ptr_cls << 8) + ptr_obj;
    result >>= 3;
    result ^= 0xed0f87;
    result ^= (91 + (result & 0xFF)) << 24;
    result += 143;

    Ok(Some(Value::Integer(result as i32)))
}
