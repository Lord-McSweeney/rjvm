use crate::gc::{Gc, GcCtx, Trace};
use crate::runtime::error::{Error, NativeError};
use crate::string::JvmString;

use std::cell::RefCell;
use std::io::{Cursor, Read};
use zip::ZipArchive;

#[derive(Clone, Copy)]
pub struct Jar(Gc<JarData>);

impl Jar {
    pub fn from_bytes(gc_ctx: GcCtx, bytes: Vec<u8>) -> Result<Self, Error> {
        let file_data = Cursor::new(bytes);
        let zip_file =
            ZipArchive::new(file_data).map_err(|_| Error::Native(NativeError::InvalidJar))?;

        Ok(Self(Gc::new(
            gc_ctx,
            JarData {
                zip_file: RefCell::new(zip_file),
            },
        )))
    }

    pub fn has_class(self, class_name: JvmString) -> bool {
        let mut modified_name = class_name.to_string().clone();
        modified_name.push_str(".class");

        self.has_file(modified_name)
    }

    pub fn read_class(self, class_name: JvmString) -> Result<Vec<u8>, Error> {
        let mut modified_name = class_name.to_string().clone();
        modified_name.push_str(".class");

        self.read_file(modified_name)
    }

    pub fn has_file(self, file_name: String) -> bool {
        let zip_file = self.0.zip_file.borrow();

        zip_file.index_for_name(&file_name).is_some()
    }

    pub fn read_file(self, file_name: String) -> Result<Vec<u8>, Error> {
        let mut zip_file = self.0.zip_file.borrow_mut();

        let result = zip_file
            .by_name(&file_name)
            .map_err(|_| Error::Native(NativeError::InvalidJar))?;

        Ok(result
            .bytes()
            .map(|b| b.expect("Byte should be Ok"))
            .collect::<Vec<_>>())
    }
}

struct JarData {
    zip_file: RefCell<ZipArchive<Cursor<Vec<u8>>>>,
}

impl Trace for Jar {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for JarData {
    fn trace(&self) {}
}
