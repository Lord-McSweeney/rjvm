use crate::{output, output_to_err};
use rjvm_core::{Context, Error, NativeError, NativeMethod, Object, Value};

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/nio/charset/Charset.stringToUtf8.(Ljava/lang/String;)[B", string_to_utf8),
        ("java/lang/StdoutStream.write.(I)V", stdout_write),
        ("java/lang/StderrStream.write.(I)V", stderr_write),
        ("java/lang/System.arraycopy.(Ljava/lang/Object;ILjava/lang/Object;II)V", array_copy),
        ("java/lang/Class.isInterface.()Z", is_interface),
        ("java/lang/Object.getClass.()Ljava/lang/Class;", get_class),
        ("java/lang/Class.getNameNative.()Ljava/lang/String;", get_name_native),
        ("java/lang/System.exit.(I)V;", system_exit),
        ("java/lang/Class.getResourceData.(Ljava/lang/String;)[B", get_resource_data),
        ("java/io/File.internalInitFileData.([B)V", internal_init_file_data),
        ("java/io/File.getCanonicalPath.()Ljava/lang/String;", file_get_canonical_path),
        ("java/io/File.getAbsolutePath.()Ljava/lang/String;", file_get_absolute_path),
    ];

    context.register_native_mappings(mappings);
}

use regex::Regex;

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

// java/lang/StdoutStream : void write(int)
fn stdout_write(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting integer in args[1]; args[0] is the reciever
    let byte = args[1].int() as u8;

    output(std::str::from_utf8(&[byte]).unwrap());

    Ok(None)
}

// java/lang/StderrStream : void write(int)
fn stderr_write(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Expecting integer in args[1]; args[0] is the reciever
    let byte = args[1].int() as u8;

    output_to_err(std::str::from_utf8(&[byte]).unwrap());

    Ok(None)
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
fn is_interface(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().get_stored_class();

    if class.is_interface() {
        Ok(Some(Value::Integer(1)))
    } else {
        Ok(Some(Value::Integer(0)))
    }
}

// java/lang/Object : Class getClass()
fn get_class(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().class();

    let class_object = context.class_object_for_class(class);

    Ok(Some(Value::Object(Some(class_object))))
}

// java/lang/Class : String getNameNative()
fn get_name_native(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let class = args[0].object().unwrap().get_stored_class();

    let string_chars = class.dot_name().encode_utf16().collect::<Vec<_>>();

    Ok(Some(Value::Object(Some(
        context.create_string(&string_chars),
    ))))
}

// java/lang/System : static void exit(int)
fn system_exit(_context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    // Just pretend we exited
    Ok(None)
}

// java/lang/Class : byte[] getResourceData(String)
fn get_resource_data(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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

fn internal_init_file_data(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let file_object = args[0].object().unwrap();
    let name_object = args[1].object().unwrap();

    let name_bytes = name_object.get_array_data();

    let mut file_name = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get().int() as u8;
        file_name.push(byte);
    }

    let file_name = String::from_utf8_lossy(&file_name);

    // TODO don't hardcode separator char
    let regex = Regex::new(r"\/{1,}").unwrap();

    let file_path = regex.replace_all(&file_name, "/");

    let file_path_chars = file_path.chars().map(|c| c as u16).collect::<Vec<_>>();

    let string_name = context.create_string(&file_path_chars);

    // No filesystem on web
    let exists = false;

    file_object.set_field(0, Value::Object(Some(string_name)));
    file_object.set_field(1, Value::Integer(exists as i32));

    Ok(None)
}

fn file_get_canonical_path(_context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    unimplemented!()
}

fn file_get_absolute_path(_context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    unimplemented!()
}
