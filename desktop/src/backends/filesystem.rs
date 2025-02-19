use rjvm_core::FilesystemBackend;

use regex::Regex;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path;

pub struct DesktopFilesystemBackend {
    files: RefCell<Vec<fs::File>>,
}

impl DesktopFilesystemBackend {
    pub fn new() -> Self {
        Self {
            files: RefCell::new(Vec::new()),
        }
    }
}

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

    fn write_by_descriptor(&self, descriptor: u32, data: &[u8]) {
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
            3.. => {
                // -3 to account for stdin, stdout, and stderr descriptors
                let mut file = &self.files.borrow()[descriptor as usize - 3];

                file.write(data).unwrap();
            }
        }
    }

    fn read_by_descriptor(&self, descriptor: u32, buffer: &mut [u8]) -> Result<(), ()> {
        match descriptor {
            0 => {
                io::stdin().read(buffer).unwrap();

                Ok(())
            }
            1 | 2 => {
                // Output streams never yield input
                loop {}
            }
            3.. => {
                // -3 to account for stdin, stdout, and stderr descriptors
                let mut file = &self.files.borrow()[descriptor as usize - 3];

                let bytes_read = file.read(buffer).unwrap();
                if bytes_read == 0 {
                    return Err(());
                }

                Ok(())
            }
        }
    }

    fn available_bytes(&self, descriptor: u32) -> u64 {
        match descriptor {
            0 | 1 | 2 => 0,
            3.. => {
                // -3 to account for stdin, stdout, and stderr descriptors
                let mut file = &self.files.borrow()[descriptor as usize - 3];

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
        }
    }

    fn writeable_descriptor_from_path(&self, path: &str) -> Result<u32, ()> {
        let mut files_ref = self.files.borrow_mut();

        let path = path::PathBuf::from(path);
        if path.is_dir() {
            return Err(());
        }

        // FIXME this sometimes returns Err when the file has a different owner
        // even if it's actually writeable for us
        let created_file = fs::File::create(path).map_err(|_| ())?;
        files_ref.push(created_file);

        // +2 to account for stdin, stdout, and stderr descriptors
        Ok(files_ref.len() as u32 + 2)
    }

    fn readable_descriptor_from_path(&self, path: &str) -> Result<u32, ()> {
        let mut files_ref = self.files.borrow_mut();

        let path = path::PathBuf::from(path);
        if path.is_dir() {
            return Err(());
        }

        let created_file = fs::File::open(path).map_err(|_| ())?;
        files_ref.push(created_file);

        // +2 to account for stdin, stdout, and stderr descriptors
        Ok(files_ref.len() as u32 + 2)
    }
}
