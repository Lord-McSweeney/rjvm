use super::context::Context;
use super::error::{Error, NativeError};
use super::object::Object;
use super::value::Value;

use std::io::{self, Write};

pub type NativeMethod = for<'a> fn(Context, &[Value]) -> Result<Option<Value>, Error>;

// java/lang/PrintStream : static byte[] stringToUtf8(String)
pub fn string_to_utf8(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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

// java/lang/StdoutStream : void write(int)
pub fn stdout_write(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting integer in args[1]; args[0] is the reciever
    let byte = args[1].int() as u8;

    io::stdout().write(&[byte]).unwrap();

    Ok(None)
}

// java/lang/System: static void arraycopy(Object, int, Object, int, int)
pub fn array_copy(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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
        return Err(Error::Native(NativeError::ArrayStoreException));
    };

    if source_value_type != dest_value_type {
        // Only throw if either of them is a primitive type; if both are object types,
        // the type-checking will be in the actual copy loop.
        if source_value_type.is_primitive() || dest_value_type.is_primitive() {
            return Err(Error::Native(NativeError::ArrayStoreException));
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
                    return Err(Error::Native(NativeError::ArrayStoreException));
                }
            }
        }

        let dest_idx = dest_start + i;
        dest_data[dest_idx].set(value);
    }

    Ok(None)
}

// java/lang/Class : boolean isInterface()
pub fn is_interface(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().get_stored_class();

    if class.is_interface() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

// java/lang/Object : Class getClass()
pub fn get_class(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().class();

    let class_object = context.class_object_for_class(class);

    Ok(Some(Value::Object(Some(class_object))))
}

// java/lang/Class : String getNameInternal()
pub fn get_name_native(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().get_stored_class();

    let string_chars = class.dot_name().encode_utf16().collect::<Vec<_>>();
    let chars_array_object = Object::char_array(context, &string_chars);

    let string_class = context
        .lookup_class(context.common.java_lang_string)
        .expect("String class should exist");

    let string_instance = string_class.new_instance(context.gc_ctx);
    string_instance
        .call_construct(
            context,
            context.common.arg_char_array_void_desc,
            &[
                Value::Object(Some(string_instance)),
                Value::Object(Some(chars_array_object)),
            ],
        )
        .expect("String class should construct");

    Ok(Some(Value::Object(Some(string_instance))))
}
