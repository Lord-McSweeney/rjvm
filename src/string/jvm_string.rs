// A UTF-8 string used for the JVM. All strings from, for example, cpools are
// stored as UTF-8 strings. However, the java/lang/String class stores strings
// as UTF-16, so conversion is needed.

use std::fmt;
use std::ops::Deref;

use crate::gc::{Gc, GcCtx};

#[derive(Clone, Copy, Debug)]
pub struct JvmString(Gc<String>);

impl fmt::Display for JvmString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.0)
    }
}

impl JvmString {
    pub fn new(gc_ctx: &GcCtx, string: String) -> Self {
        Self(Gc::new(gc_ctx, string))
    }
}

impl Deref for JvmString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
