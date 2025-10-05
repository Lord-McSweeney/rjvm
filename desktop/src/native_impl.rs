use rjvm_core::{Context, Error, NativeMethod, Value};

use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path;
use std::process;
use std::sync::Mutex;
use std::time::SystemTime;

static ALL_FILES: Mutex<Vec<fs::File>> = Mutex::new(Vec::new());

pub fn register_native_mappings(context: Context) {
    #[rustfmt::skip]
    let mappings: &[(&str, NativeMethod)] = &[
        ("java/lang/Runtime.exit.(I)V", system_exit),
        ("java/lang/System.currentTimeMillis.()J", system_current_time_millis),
        ("java/io/File.internalInitFileData.([B)V", internal_init_file_data),
        ("java/io/File.getCanonicalPath.()Ljava/lang/String;", file_get_canonical_path),
        ("java/io/File.getAbsolutePath.()Ljava/lang/String;", file_get_absolute_path),
        ("java/io/FileOutputStream.writeInternal.(I)V", file_stream_write_internal),
        ("java/io/FileOutputStream.flushInternal.()V", file_stream_flush_internal),
        ("java/io/FileInputStream.readInternal.()I", file_stream_read_internal),
        ("java/io/FileInputStream.readMultiInternal.([BII)I", file_stream_read_multi_internal),
        ("java/io/FileInputStream.availableInternal.()I", file_stream_available_internal),
        ("java/io/FileDescriptor.internalWriteableDescriptorFromPath.(Ljava/lang/String;)I", writeable_descriptor_from_path),
        ("java/io/FileDescriptor.internalReadableDescriptorFromPath.(Ljava/lang/String;)I", readable_descriptor_from_path),
    ];

    context.register_native_mappings(mappings);
}

// java/lang/System : static void exit(int)
fn system_exit(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let exit_code = args[1].int();

    process::exit(exit_code)
}

fn system_current_time_millis(_context: Context, _args: &[Value]) -> Result<Option<Value>, Error> {
    let millisecs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("User didn't set their clock to 1969")
        .as_millis();

    Ok(Some(Value::Long(millisecs as i64)))
}

fn internal_init_file_data(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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

    let exists = fs::exists(&*file_name).unwrap_or(false);
    let metadata = fs::metadata(&*file_name).ok();

    file_object.set_field(0, Value::Object(Some(string_name)));
    file_object.set_field(1, Value::Integer(exists as i32));
    file_object.set_field(
        2,
        Value::Integer(metadata.is_some_and(|m| m.is_dir()) as i32),
    );

    Ok(None)
}

fn file_get_canonical_path(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let file_object = args[0].object().unwrap();
    let name_object = file_object.get_field(0).object().unwrap();

    let name_array = name_object.get_field(0).object().unwrap();
    let name_bytes = name_array.array_data().as_char_array();

    let mut file_name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        file_name_data.push(value.get());
    }

    let file_name = String::from_utf16_lossy(&file_name_data);

    // This is very expensive but seems to exactly match Java, except (FIXME)
    // we should throw an IOException instead of the `unwrap_or_default`
    // TODO use correct file separator instead of assuming it must be '/'
    let first_regex = Regex::new(r"[^\/]{1,}\/\.\.").unwrap();
    let second_regex = Regex::new(r"\/{1,}").unwrap();

    let canonical_path = path::absolute(&*file_name)
        .map(|p| {
            let bytes = p.into_os_string().into_encoded_bytes();
            let string = String::from_utf8_lossy(&bytes);

            let first_replace = first_regex.replace_all(&string, "/");
            let second_replace = second_regex.replace_all(&first_replace, "/");

            if second_replace == "/" {
                second_replace.to_string()
            } else if let Some(stripped) = second_replace.strip_suffix('/') {
                stripped.to_string()
            } else {
                second_replace.to_string()
            }
        })
        .unwrap_or_default();

    let mut chars_vec = Vec::with_capacity(canonical_path.len());
    for byte in canonical_path.chars() {
        chars_vec.push(byte as u16);
    }

    let string_object = context.create_string(&chars_vec);

    Ok(Some(Value::Object(Some(string_object))))
}

fn file_get_absolute_path(context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let file_object = args[0].object().unwrap();
    let name_object = file_object.get_field(0).object().unwrap();

    let name_array = name_object.get_field(0).object().unwrap();
    let name_bytes = name_array.array_data().as_char_array();

    let mut file_name_data = Vec::with_capacity(name_bytes.len());
    for value in name_bytes {
        file_name_data.push(value.get());
    }

    let file_name = String::from_utf16_lossy(&file_name_data);

    let mut result = env::current_dir().unwrap_or_default();
    let path = path::PathBuf::from(&*file_name);

    result.push(path);

    let result_bytes = result.into_os_string().into_encoded_bytes();

    let absolute_path = String::from_utf8_lossy(&result_bytes).to_string();

    let mut chars_vec = Vec::with_capacity(absolute_path.len());
    for byte in absolute_path.chars() {
        chars_vec.push(byte as u16);
    }

    let string_object = context.create_string(&chars_vec);

    Ok(Some(Value::Object(Some(string_object))))
}

fn file_stream_write_internal(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
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
            io::stdout().write(&[write_data]).unwrap();
        }
        2 => {
            // stderr
            io::stderr().write(&[write_data]).unwrap();
        }
        3.. => {
            // -3 to account for stdin, stdout, and stderr descriptors
            let mut file = &ALL_FILES.lock().unwrap()[stream_descriptor as usize - 3];

            file.write(&[write_data]).unwrap();
        }
    }

    Ok(None)
}

fn file_stream_flush_internal(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    match stream_descriptor {
        0 => {
            // Flushing stdin is a noop
        }
        1 => {
            // stdout
            io::stdout().flush().unwrap();
        }
        2 => {
            // stderr
            io::stderr().flush().unwrap();
        }
        3.. => {
            // -3 to account for stdin, stdout, and stderr descriptors
            let mut file = &ALL_FILES.lock().unwrap()[stream_descriptor as usize - 3];

            file.flush().unwrap();
        }
    }

    Ok(None)
}

fn file_stream_read_internal(_context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    let mut write_buf = [0; 1];

    match stream_descriptor {
        0 => {
            io::stdin().read(&mut write_buf).unwrap();

            Ok(Some(Value::Integer(write_buf[0] as i32)))
        }
        1 | 2 => {
            // Output streams never yield input
            loop {}
        }
        3.. => {
            // -3 to account for stdin, stdout, and stderr descriptors
            let mut file = &ALL_FILES.lock().unwrap()[stream_descriptor as usize - 3];

            let bytes_read = file.read(&mut write_buf).unwrap();
            if bytes_read == 0 {
                Ok(Some(Value::Integer(-1)))
            } else {
                Ok(Some(Value::Integer(write_buf[0] as i32)))
            }
        }
    }
}

fn file_stream_read_multi_internal(
    _context: Context,
    args: &[Value],
) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    let write_arr = args[1].object().unwrap();
    let requested_offset = args[2].int() as usize;
    let requested_length = args[3].int() as usize;

    let mut write_buf = vec![0; requested_length];

    let bytes_read = match stream_descriptor {
        0 => io::stdin().read(&mut write_buf).unwrap(),
        1 | 2 => {
            // Output streams never yield input
            loop {}
        }
        3.. => {
            // -3 to account for stdin, stdout, and stderr descriptors
            let mut file = &ALL_FILES.lock().unwrap()[stream_descriptor as usize - 3];

            file.read(&mut write_buf).unwrap()
        }
    };

    let array_data = write_arr.array_data().as_byte_array();

    for (dest, src) in array_data
        .iter()
        .skip(requested_offset)
        .zip(write_buf.iter())
    {
        dest.set(*src as i8);
    }

    Ok(Some(Value::Integer(bytes_read as i32)))
}

fn file_stream_available_internal(
    _context: Context,
    args: &[Value],
) -> Result<Option<Value>, Error> {
    let stream = args[0].object().unwrap();
    let stream_fd = stream.get_field(0).object().unwrap();
    let stream_descriptor = stream_fd.get_field(0).int() as u32;

    let result = match stream_descriptor {
        0 | 1 | 2 => 0,
        3.. => {
            // -3 to account for stdin, stdout, and stderr descriptors
            let mut file = &ALL_FILES.lock().unwrap()[stream_descriptor as usize - 3];

            // Taken from code of `File::stream_len`, an unstable function
            let old_pos = file.stream_position().unwrap();
            let len = file.seek(SeekFrom::End(0)).unwrap();

            // Avoid seeking a third time when we were already at the end of the
            // stream. The branch is usually way cheaper than a seek operation.
            if old_pos != len {
                file.seek(SeekFrom::Start(old_pos)).unwrap();
            }

            len
        }
    };

    Ok(Some(Value::Integer(result as i32)))
}

fn writeable_descriptor_from_path(
    _context: Context,
    args: &[Value],
) -> Result<Option<Value>, Error> {
    let path_object = args[0].object().unwrap();

    let path_array = path_object.get_field(0).object().unwrap();
    let path_bytes = path_array.array_data().as_char_array();

    let mut file_path_data = Vec::with_capacity(path_bytes.len());
    for value in path_bytes {
        file_path_data.push(value.get());
    }

    let file_path = String::from_utf16_lossy(&file_path_data);

    let mut files_ref = ALL_FILES.lock().unwrap();

    let path = path::PathBuf::from(&*file_path);
    if path.is_dir() {
        // Return -1 to signal that there was an error- Java code will throw the exception
        return Ok(Some(Value::Integer(-1)));
    }

    // FIXME this sometimes returns Err when the file has a different owner
    // even if it's actually writeable for us
    let Ok(created_file) = fs::File::create(path) else {
        // Return -1 to signal that there was an error- Java code will throw the exception
        return Ok(Some(Value::Integer(-1)));
    };

    files_ref.push(created_file);

    // +2 to account for stdin, stdout, and stderr descriptors
    Ok(Some(Value::Integer(files_ref.len() as i32 + 2)))
}

fn readable_descriptor_from_path(
    _context: Context,
    args: &[Value],
) -> Result<Option<Value>, Error> {
    let path_object = args[0].object().unwrap();

    let path_array = path_object.get_field(0).object().unwrap();
    let path_bytes = path_array.array_data().as_char_array();

    let mut file_path_data = Vec::with_capacity(path_bytes.len());
    for value in path_bytes {
        file_path_data.push(value.get());
    }

    let file_path = String::from_utf16_lossy(&file_path_data);

    let mut files_ref = ALL_FILES.lock().unwrap();

    let path = path::PathBuf::from(&*file_path);
    if path.is_dir() {
        // Return -1 to signal that there was an error- Java code will throw the exception
        return Ok(Some(Value::Integer(-1)));
    }

    let Ok(created_file) = fs::File::open(path) else {
        // Return -1 to signal that there was an error- Java code will throw the exception
        return Ok(Some(Value::Integer(-1)));
    };

    files_ref.push(created_file);

    // +2 to account for stdin, stdout, and stderr descriptors
    Ok(Some(Value::Integer(files_ref.len() as i32 + 2)))
}
