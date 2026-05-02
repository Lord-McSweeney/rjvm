// Thin wrapper for reading JAR files

use super::read_zip::{ZipFile, ZipReadError};
use crate::gc::{Gc, GcCtx, Trace};

use alloc::string::String;
use alloc::vec::Vec;

/// A JAR file.
///
/// Currently this crate uses a custom zip decoder, so this struct only supports
/// deflate-compressed JAR files.
#[derive(Clone, Copy)]
pub struct Jar(Gc<JarData>);

impl Jar {
    /// Create a JAR file from the provided data.
    pub fn from_bytes(gc_ctx: GcCtx, bytes: Vec<u8>) -> Result<Self, ZipReadError> {
        let jar_file = ZipFile::new(bytes)?;

        Ok(Self(Gc::new(gc_ctx, JarData { jar_file })))
    }

    /// Checks whether the JAR contains a file with the given name.
    pub fn has_file(self, file_name: &String) -> bool {
        self.0.jar_file.has_file(file_name)
    }

    /// Reads a file with the given name from the JAR.
    ///
    /// This method makes no attempt to normalize the provided name as a path
    /// into the JAR.
    pub fn read_file(self, file_name: String) -> Result<Vec<u8>, ZipReadError> {
        self.0.jar_file.read_file(&file_name)
    }
}

struct JarData {
    jar_file: ZipFile,
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
