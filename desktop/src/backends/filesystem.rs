use rjvm_core::FilesystemBackend;

use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path;

pub struct DesktopFilesystemBackend {}

impl FilesystemBackend for DesktopFilesystemBackend {
    fn to_absolute_path(&self, path: &str) -> String {
        let mut result = env::current_dir().unwrap_or_default();
        let path = path::PathBuf::from(&*path);

        result.push(path);

        let result_bytes = result.into_os_string().into_encoded_bytes();

        String::from_utf8_lossy(&result_bytes).to_string()
    }

    fn to_canonical_path(&self, path: &str) -> String {
        // This is very expensive but seems to exactly match Java, except (FIXME)
        // we should throw an IOException instead of the `unwrap_or_default`
        // TODO use correct file separator instead of assuming it must be '/'
        let first_regex = Regex::new(r"[^\/]{1,}\/\.\.").unwrap();
        let second_regex = Regex::new(r"\/{1,}").unwrap();

        path::absolute(&*path)
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
            .unwrap_or_default()
    }

    fn file_exists(&self, path: &str) -> Result<bool, ()> {
        fs::exists(path).map_err(|_| ())
    }

    fn write_by_descriptor(&self, descriptor: i32, data: &[u8]) {
        match descriptor {
            0 => {
                // Writing to stdin is a noop
            }
            1 => {
                // stdout
                io::stdout().write(data).unwrap();
            }
            2 => {
                // stderr
                io::stderr().write(data).unwrap();
            }
            _ => unimplemented!("writing to files"),
        }
    }
}
