// A UTF-8 string used for the JVM. All strings from, for example, cpools are
// stored as UTF-8 strings. Note that the java/lang/String class stores strings
// as UTF-16.

use crate::gc::{Gc, GcCtx, Trace};

use alloc::string::String;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

const MAX_HASHED_STRING_LEN: usize = 4096;

#[derive(Clone, Copy, Debug)]
pub struct JvmString(Gc<JvmStringData>);

#[derive(Debug)]
struct JvmStringData {
    hash: u64,
    contents: String,
}

impl Trace for JvmStringData {
    fn trace(&self) {}
}

impl PartialEq for JvmString {
    fn eq(&self, other: &Self) -> bool {
        self.0.contents == other.0.contents
    }
}

impl Eq for JvmString {}

impl Hash for JvmString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash.hash(state);
    }
}

impl fmt::Display for JvmString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &*self.0.contents)
    }
}

impl JvmString {
    pub fn new(gc_ctx: GcCtx, string: String) -> Self {
        // Precompute hash. Anything better than just the length of the string
        // is probably enough. The GC allocation is probably always much more
        // expensive than any hashing we do here.
        let mut hash = string.len() as u64;
        if string.len() < MAX_HASHED_STRING_LEN {
            for (i, byte) in string.as_bytes().iter().enumerate() {
                hash += ((byte ^ 0xAA) as u64) << (i as u64);
            }
        }

        Self(Gc::new(
            gc_ctx,
            JvmStringData {
                hash,
                contents: string,
            },
        ))
    }

    pub fn to_string(&self) -> &String {
        &*self
    }
}

impl Deref for JvmString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0.contents
    }
}

impl Trace for JvmString {
    fn trace(&self) {
        self.0.trace();
    }
}
