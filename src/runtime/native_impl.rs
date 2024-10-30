use super::context::Context;
use super::error::{Error, NativeError};
use super::object::Object;
use super::value::Value;

use std::io::{self, Write};

pub type NativeMethod = for<'a> fn(Context, &[Value]) -> Result<Option<Value>, Error>;

// java/lang/PrintStream : static byte[] stringToUtf8(String)
pub fn string_to_utf8(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting non-null object
    let string_object = args[0].expect_as_object().unwrap();
    let char_array = string_object.get_field(0).expect_as_object().unwrap();

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
    let Value::Integer(byte) = args[1] else {
        unreachable!();
    };

    let byte = byte as u8;

    io::stdout().write(&[byte]).unwrap();

    Ok(None)
}

// java/lang/System: static void arraycopy(Object, int, Object, int, int)
pub fn array_copy(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let source_arr = args[0].expect_as_object();
    let Some(source_arr) = source_arr else {
        return Err(context.null_pointer_exception());
    };

    let Value::Integer(source_start) = args[1] else {
        unreachable!();
    };

    let dest_arr = args[2].expect_as_object();
    let Some(dest_arr) = dest_arr else {
        return Err(context.null_pointer_exception());
    };

    let Value::Integer(dest_start) = args[3] else {
        unreachable!();
    };

    let Value::Integer(length) = args[4] else {
        unreachable!();
    };

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

    if !dest_arr.is_array() || !source_arr.is_array() {
        return Err(Error::Native(NativeError::ArrayStoreException));
    }

    let source_value_type = source_arr.class().array_value_type();
    let dest_value_type = dest_arr.class().array_value_type();

    assert!(source_value_type.is_some() && dest_value_type.is_some());

    if source_value_type != dest_value_type {
        // Only throw if one of them is a primitive type; if both are object types,
        // the type-checking will be in the actual copy loop.
        if source_value_type.unwrap().is_primitive() || dest_value_type.unwrap().is_primitive() {
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
                if !obj.class().matches_descriptor(dest_value_type.unwrap()) {
                    return Err(Error::Native(NativeError::ArrayStoreException));
                }
            }
        }

        let dest_idx = dest_start + i;
        dest_data[dest_idx].set(value);
    }

    Ok(None)
}
