// Stub for reading JAR files- this impl panics when attempting to create a
// `Jar`. This is used when the "jar" feature is disabled
#![allow(dead_code)]

use crate::gc::{Gc, GcCtx, Trace};
use crate::runtime::error::Error;
use crate::string::JvmString;

use alloc::vec::Vec;

#[derive(Clone, Copy)]
pub struct Jar(Gc<()>);

impl Jar {
    pub fn from_bytes(_gc_ctx: GcCtx, _bytes: Vec<u8>) -> Result<Self, Error> {
        panic!("Jar support not compiled in");
    }

    pub fn has_class(self, _class_name: JvmString) -> bool {
        unreachable!()
    }

    pub fn read_class(self, _class_name: JvmString) -> Result<Vec<u8>, Error> {
        unreachable!()
    }

    pub fn has_file(self, _file_name: &str) -> bool {
        unreachable!()
    }

    pub fn read_file(self, _file_name: &str) -> Result<Vec<u8>, Error> {
        unreachable!()
    }
}

impl Trace for Jar {
    fn trace(&self) {
        self.0.trace();
    }
}
