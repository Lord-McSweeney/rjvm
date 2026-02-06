use alloc::boxed::Box;
use alloc::string::ToString;
use rjvm_core::{Context, Error, NativeMethod, Object, Value};

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/Object.getClass.()Ljava/lang/Class;", get_class),
        ("java/lang/Object.clone.()Ljava/lang/Object;", object_clone),
        ("java/lang/Throwable.internalFillInStackTrace.()[Ljava/lang/StackTraceElement;", capture_stack_trace),
        ("java/lang/Object.hashCode.()I", object_hash_code),
        ("java/lang/String.intern.()Ljava/lang/String;", string_intern),
        ("java/lang/Double.doubleToRawLongBits.(D)J", double_to_raw_long_bits),

        ("java/nio/charset/Charset.stringToUtf8.(Ljava/lang/String;)[B", string_to_utf8),
        ("java/lang/Double.toString.(D)Ljava/lang/String;", double_to_string),
        ("java/lang/Float.toString.(F)Ljava/lang/String;", float_to_string),
    ];

    context.register_native_mappings(mappings);
}

// Native implementations of functions declared in globals

// java/lang/Object : Class getClass()
fn get_class(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().class();

    let class_object = class.get_or_init_object(context);

    Ok(Some(Value::Object(Some(class_object))))
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

fn object_hash_code(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let this = args[0].object().unwrap();

    let result = crate::hash_code::calc_hash_code(this);

    Ok(Some(Value::Integer(result)))
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

fn string_to_utf8(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting non-null object
    let string_object = args[0].object().unwrap();
    let string = Context::string_object_to_string(string_object);

    let bytes = string.as_bytes();
    let bytes = bytes.iter().copied().map(|b| b as i8).collect::<Box<_>>();

    let byte_array = Object::byte_array(context, bytes);

    Ok(Some(Value::Object(Some(byte_array))))
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

fn float_to_string(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let float = args[0].float();

    let string = if float.is_infinite() {
        if float < 0.0 {
            "-Infinity".to_string()
        } else {
            "Infinity".to_string()
        }
    } else {
        let string = format!("{:.3}", float);

        let mut string = string.trim_end_matches('0').to_string();
        if string.ends_with('.') {
            string.push('0');
        }

        string
    };

    let chars = string.chars().map(|c| c as u16).collect::<Box<_>>();

    Ok(Some(Value::Object(Some(context.create_string(&chars)))))
}
