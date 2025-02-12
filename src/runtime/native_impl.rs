use super::context::Context;
use super::error::{Error, NativeError};
use super::object::Object;
use super::value::Value;

use crate::gc::Trace;

use std::io::{self, Write};

pub type NativeMethod = for<'a> fn(Context, &[Value]) -> Result<Option<Value>, Error>;

impl Trace for NativeMethod {
    fn trace(&self) {}
}

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

// java/lang/StderrStream : void write(int)
pub fn stderr_write(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting integer in args[1]; args[0] is the reciever
    let byte = args[1].int() as u8;

    io::stderr().write(&[byte]).unwrap();

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

// java/lang/Class : String getNameNative()
pub fn get_name_native(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().get_stored_class();

    let string_chars = class.dot_name().encode_utf16().collect::<Vec<_>>();

    Ok(Some(Value::Object(Some(
        context.create_string(&string_chars),
    ))))
}

// java/lang/System : static void exit(int)
pub fn system_exit(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    use std::process;

    let exit_code = args[0].int();

    process::exit(exit_code)
}

// java/lang/Class : byte[] getResourceData(String)
pub fn get_resource_data(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().get_stored_class();

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

pub fn internal_init_file_data(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    use std::fs;

    let file_object = args[0].object().unwrap();
    let name_object = args[1].object().unwrap();

    let name_bytes = name_object.get_array_data();

    let mut file_name = String::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get().int() as u8;
        file_name.push(byte as char);
    }

    let exists = fs::exists(file_name).unwrap_or(false);

    file_object.set_field(0, Value::Object(Some(name_object)));
    file_object.set_field(1, Value::Integer(exists as i32));

    Ok(None)
}

pub fn file_get_canonical_path(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    use regex::Regex;
    use std::path;

    let file_object = args[0].object().unwrap();
    let name_object = file_object.get_field(0).object().unwrap();

    let name_bytes = name_object.get_array_data();

    let mut file_name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get().int() as u8;
        file_name_data.push(byte);
    }

    let file_name = String::from_utf8_lossy(&file_name_data);

    // This is very expensive but seems to exactly match Java, except (FIXME)
    // we should throw an IOException instead of the `unwrap_or_default`
    // TODO use correct file separator instead of assuming it must be '/'
    let first_regex = Regex::new(r"[^\/]{1,}\/\.\.").unwrap();
    let second_regex = Regex::new(r"\/{1,}").unwrap();

    let canonicalized_path = path::absolute(&*file_name)
        .map(|p| {
            let bytes = p.into_os_string().into_encoded_bytes();
            let string = String::from_utf8_lossy(&bytes);

            let first_replace = first_regex.replace_all(&string, "/");
            let second_replace = second_regex.replace_all(&first_replace, "/");

            if let Some(stripped) = second_replace.strip_suffix('/') {
                stripped.to_string()
            } else {
                second_replace.to_string()
            }
        })
        .unwrap_or_default();

    let mut chars_vec = Vec::with_capacity(canonicalized_path.len());
    for byte in canonicalized_path.chars() {
        chars_vec.push(byte as u16);
    }

    let string_object = context.create_string(&chars_vec);

    Ok(Some(Value::Object(Some(string_object))))
}

pub fn file_get_parent(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    use std::path;

    let file_object = args[0].object().unwrap();
    let name_object = file_object.get_field(0).object().unwrap();

    let name_bytes = name_object.get_array_data();

    let mut file_name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get().int() as u8;
        file_name_data.push(byte);
    }

    let file_name = String::from_utf8_lossy(&file_name_data);

    let path_buf = path::PathBuf::from(&*file_name);
    let parent = path_buf.parent();
    let Some(parent) = parent else {
        return Ok(Some(Value::Object(None)));
    };

    let parent_string = String::from_utf8_lossy(parent.as_os_str().as_encoded_bytes());

    let mut chars_vec = Vec::with_capacity(parent_string.len());
    for byte in parent_string.chars() {
        chars_vec.push(byte as u16);
    }

    let string_object = context.create_string(&chars_vec);

    Ok(Some(Value::Object(Some(string_object))))
}

pub fn file_get_name(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let file_object = args[0].object().unwrap();
    let name_object = file_object.get_field(0).object().unwrap();

    let name_bytes = name_object.get_array_data();

    let mut file_name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get().int() as u8;
        file_name_data.push(byte);
    }

    let file_name = String::from_utf8_lossy(&file_name_data);

    // TODO don't hardcode separator char
    let file_name = file_name.split('/').last();
    let Some(file_name) = file_name else {
        // Return an empty string if there is no file name
        return Ok(Some(Value::Object(Some(context.create_string(&[])))));
    };

    let file_name_string = String::from_utf8_lossy(file_name.as_bytes());

    let mut chars_vec = Vec::with_capacity(file_name_string.len());
    for byte in file_name_string.chars() {
        chars_vec.push(byte as u16);
    }

    let string_object = context.create_string(&chars_vec);

    Ok(Some(Value::Object(Some(string_object))))
}

pub fn file_get_path(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    use regex::Regex;

    let file_object = args[0].object().unwrap();
    let name_object = file_object.get_field(0).object().unwrap();

    let name_bytes = name_object.get_array_data();

    let mut file_name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get().int() as u8;
        file_name_data.push(byte);
    }

    let file_name = String::from_utf8_lossy(&file_name_data);

    // TODO don't hardcode separator char
    let regex = Regex::new(r"\/{1,}").unwrap();

    let file_path = regex.replace_all(&file_name, "/");

    let file_path_string = String::from_utf8_lossy(file_path.as_bytes());

    let mut chars_vec = Vec::with_capacity(file_path_string.len());
    for byte in file_path_string.chars() {
        chars_vec.push(byte as u16);
    }

    let string_object = context.create_string(&chars_vec);

    Ok(Some(Value::Object(Some(string_object))))
}
