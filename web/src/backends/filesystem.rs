use crate::output;
use crate::output_to_err;
use rjvm_core::FilesystemBackend;

pub struct WebFilesystemBackend {}

impl FilesystemBackend for WebFilesystemBackend {
    fn to_absolute_path(&self, _path: &str) -> String {
        unimplemented!()
    }

    fn to_canonical_path(&self, _path: &str) -> String {
        unimplemented!()
    }

    fn file_exists(&self, _path: &str) -> Result<bool, ()> {
        // No filesystem on web
        Ok(false)
    }

    fn write_by_descriptor(&self, descriptor: i32, data: &[u8]) {
        match descriptor {
            0 => {
                // Writing to stdin is a noop
            }
            1 => {
                // stdout
                output(&*String::from_utf8_lossy(data));
            }
            2 => {
                // stderr
                output_to_err(&*String::from_utf8_lossy(data));
            }
            _ => unimplemented!("writing to files"),
        }
    }
}
