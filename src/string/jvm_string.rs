// A UTF-8 string used for the JVM. All strings from, for example, cpools are
// stored as UTF-8 strings. However, the java/lang/String class stores strings
// as UTF-16, so conversion is needed.

use std::ops::Deref;

use crate::gc::{Gc, GcCtx};

pub struct JvmString(Gc<String>);

impl JvmString {
    fn new(gc_ctx: &GcCtx, string: String) -> Self {
        Self(Gc::new(&gc_ctx, string))
    }
}

impl Deref for JvmString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
