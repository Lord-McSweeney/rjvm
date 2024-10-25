// A UTF-8 string used for the JVM. All strings from, for example, cpools are
// stored as UTF-8 strings. Note that the java/lang/String class stores strings
// as UTF-16.

use crate::gc::{Gc, GcCtx, Trace};

use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;

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
        // Precompute hash
        let mut hasher = DefaultHasher::new();
        string.hash(&mut hasher);
        let hash = hasher.finish();

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
