// Thin wrapper for reading JAR files

use super::read_zip::ZipFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::runtime::error::{Error, NativeError};
use crate::string::JvmString;

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;

#[derive(Clone, Copy)]
pub struct Jar(Gc<JarData>);

impl Jar {
    pub fn from_bytes(gc_ctx: GcCtx, bytes: Vec<u8>) -> Result<Self, Error> {
        let jar_file = ZipFile::new(bytes).map_err(|_| Error::Native(NativeError::InvalidJar))?;

        Ok(Self(Gc::new(
            gc_ctx,
            JarData {
                jar_file,
                cached_files: RefCell::new(HashMap::new()),
            },
        )))
    }

    pub fn has_class(self, class_name: JvmString) -> bool {
        let mut modified_name = class_name.to_string().clone();
        modified_name.push_str(".class");

        self.has_file(&modified_name)
    }

    pub fn read_class(self, class_name: JvmString) -> Result<Vec<u8>, Error> {
        let mut modified_name = class_name.to_string().clone();
        modified_name.push_str(".class");

        self.read_file(modified_name)
    }

    pub fn has_file(self, file_name: &String) -> bool {
        self.0.jar_file.has_file(file_name)
    }

    pub fn read_file(self, file_name: String) -> Result<Vec<u8>, Error> {
        let mut cached_files = self.0.cached_files.borrow_mut();
        match cached_files.entry(file_name.clone()) {
            Entry::Occupied(occupied) => Ok(occupied.get().clone()),
            Entry::Vacant(vacant) => {
                let result = self
                    .0
                    .jar_file
                    .read_file(&file_name)
                    .map_err(|_| Error::Native(NativeError::InvalidJar))?;

                vacant.insert(result.clone());

                Ok(result)
            }
        }
    }
}

struct JarData {
    jar_file: ZipFile,
    cached_files: RefCell<HashMap<String, Vec<u8>>>,
}

impl Trace for Jar {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for JarData {
    fn trace(&self) {
        self.cached_files.trace();
    }
}
