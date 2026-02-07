use crate::output;
use crate::output_to_err;

use rjvm_core::{Context, Error, NativeMethod, Value};
use wasm_bindgen::prelude::*;

pub fn register_native_mappings(context: &Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/Runtime.exit.(I)V", system_exit),
        ("java/lang/System.currentTimeMillis.()J", system_current_time_millis),
        ("java/io/File.internalInitFileData.([B)V", internal_init_file_data),
        ("java/io/File.getCanonicalPath.()Ljava/lang/String;", file_get_canonical_path),
        ("java/io/File.getAbsolutePath.()Ljava/lang/String;", file_get_absolute_path),
        ("java/io/FileOutputStream.writeNative.(I)V", file_stream_write),
        ("java/io/FileOutputStream.writeMultipleNative.([BII)V", file_stream_write_multiple),
        ("java/io/FileOutputStream.flushNative.()V", file_stream_flush),
        ("java/io/FileInputStream.readNative.()I", file_stream_read),
        ("java/io/FileInputStream.readMultipleNative.([BII)I", file_stream_read_multiple),
        ("java/io/FileInputStream.availableNative.()I", file_stream_available),
        ("java/io/FileDescriptor.internalWriteableDescriptorFromPath.(Ljava/lang/String;)I", writeable_descriptor_from_path),
        ("java/io/FileDescriptor.internalReadableDescriptorFromPath.(Ljava/lang/String;)I", readable_descriptor_from_path),
    ];

    context.register_native_mappings(mappings);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date, js_name = "now")]
    fn date_now() -> f64;
}

// java/lang/System : static void exit(int)
fn system_exit(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let exit_code = args[1].int();

    // No exit function on web
    panic!("System.exit called (code {})", exit_code)
}

fn system_current_time_millis(_context: &Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    let millisecs = date_now() as i64;
    Ok(Some(Value::Long(millisecs)))
}

fn internal_init_file_data(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let file_object = args[0].object().unwrap();
    let name_object = args[1].object().unwrap();

    let name_bytes = name_object.array_data().as_byte_array();

    let mut file_name = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        let byte = value.get() as u8;
        file_name.push(byte);
    }

    file_name.dedup_by(|a, b| *a == b'/' && *b == b'/');

    let file_name = String::from_utf8_lossy(&file_name);

    let file_name = if file_name == "/" {
        &file_name
    } else if let Some(stripped) = file_name.strip_suffix('/') {
        stripped
    } else {
        &file_name
    };

    let file_name_chars = file_name.chars().map(|c| c as u16).collect::<Vec<_>>();

    let string_name = context.create_string(&file_name_chars);

    // No filesystem on web
    let exists = false;
    let is_dir = false;

    file_object.set_field(0, Value::Object(Some(string_name)));
    file_object.set_field(1, Value::Integer(exists as i32));
    file_object.set_field(2, Value::Integer(is_dir as i32));

    Ok(None)
}

fn file_get_canonical_path(_context: &Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    unimplemented!("File.getCanonicalPath is unimplemented on web")
}

fn file_get_absolute_path(_context: &Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    unimplemented!("File.getAbsolutePath is unimplemented on web")
}

fn file_stream_write(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    let write_data = args[1].int() as u8;

    match stream_descriptor {
        0 => {
            // Writing to stdin is a noop
        }
        1 => {
            // stdout
            output(&*String::from_utf8_lossy(&[write_data]));
        }
        2 => {
            // stderr
            output_to_err(&*String::from_utf8_lossy(&[write_data]));
        }
        _ => unreachable!("cannot have descriptors >2 on web"),
    }

    Ok(None)
}

fn file_stream_write_multiple(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    // Java code has already checked these values for us
    let write_arr = args[1].object().unwrap();
    let arr_start = args[2].int() as usize;
    let arr_len = args[3].int() as usize;

    let array_data = write_arr.array_data().as_byte_array();
    let write_data = array_data.iter().map(|b| b.get() as u8).collect::<Vec<_>>();
    let write_data = &write_data[arr_start..arr_start + arr_len];

    match stream_descriptor {
        0 => {
            // Writing to stdin is a noop
        }
        1 => {
            // stdout
            output(&*String::from_utf8_lossy(write_data));
        }
        2 => {
            // stderr
            output_to_err(&*String::from_utf8_lossy(write_data));
        }
        _ => unreachable!("cannot have descriptors >2 on web"),
    }

    Ok(None)
}

fn file_stream_flush(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    match stream_descriptor {
        0 => {
            // Flushing stdin is a noop
        }
        1 => {
            // `output` does not buffer, so no need to flush
        }
        2 => {
            // `output_to_err` does not buffer, so no need to flush
        }
        _ => unreachable!("cannot have descriptors >2 on web"),
    }

    Ok(None)
}

fn file_stream_read(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    match stream_descriptor {
        0 => {
            // TODO implement
            Ok(Some(Value::Integer(-1)))
        }
        1 | 2 => {
            // Output streams never yield input
            loop {}
        }
        _ => unreachable!("cannot have descriptors >2 on web"),
    }
}

fn file_stream_read_multiple(_context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    match stream_descriptor {
        0 => {
            // TODO implement
        }
        1 | 2 => {
            // Output streams never yield input
            loop {}
        }
        _ => unreachable!("cannot have descriptors >2 on web"),
    }

    // TODO implement
    Ok(Some(Value::Integer(0)))
}

fn file_stream_available(_context: &Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    // No files on web
    Ok(Some(Value::Integer(0)))
}

fn writeable_descriptor_from_path(
    _context: &Context,
    _args: &[Value],
) -> Result<Option<Value>, Error> {
    // No files on web
    Ok(Some(Value::Integer(-1)))
}

fn readable_descriptor_from_path(
    _context: &Context,
    _args: &[Value],
) -> Result<Option<Value>, Error> {
    // No files on web
    Ok(Some(Value::Integer(-1)))
}
