// Thin wrapper for reading JAR files

use super::read_zip::ZipFile;
use crate::gc::{Gc, GcCtx, Trace};
use crate::reader::ReadError;

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;

/// A JAR file.
///
/// Currently this crate uses a custom zip decoder, so this struct only supports
/// deflate-compressed JAR files.
#[derive(Clone, Copy)]
pub struct Jar(Gc<JarData>);

impl Jar {
    /// Create a JAR file from the provided data.
    pub fn from_bytes(gc_ctx: GcCtx, bytes: Vec<u8>) -> Result<Self, ReadError> {
        let jar_file = ZipFile::new(bytes)?;

        Ok(Self(Gc::new(
            gc_ctx,
            JarData {
                jar_file,
                cached_files: RefCell::new(HashMap::new()),
            },
        )))
    }

    /// Checks whether the JAR contains a file with the given name.
    pub fn has_file(self, file_name: &String) -> bool {
        self.0.jar_file.has_file(file_name)
    }

    /// Reads a file with the given name from the JAR.
    ///
    /// This method makes no attempt to normalize the provided name as a path
    /// into the JAR.
    ///
    /// This method will cache read files.
    pub fn read_file(self, file_name: String) -> Result<Vec<u8>, ()> {
        let mut cached_files = self.0.cached_files.borrow_mut();
        match cached_files.entry(file_name.clone()) {
            Entry::Occupied(occupied) => Ok(occupied.get().clone()),
            Entry::Vacant(vacant) => {
                let result = self.0.jar_file.read_file(&file_name)?;

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
        // This doesn't store any Gc pointers
    }
}
