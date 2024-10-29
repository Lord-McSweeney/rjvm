use super::context::Context;
use super::error::Error;
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
