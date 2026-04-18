use crate::gc::{Gc, GcCtx, Trace};

use alloc::string::String;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

/// A UTF-8 string, used for representing strings in the JVM.
///
/// All strings from, for example, constant pools, are stored as UTF-8 strings.
/// Note that the `java.lang.String` class stores strings as UTF-16.
///
/// This type is `Copy`. It simply stores a [`String`] behind a [`Gc`] pointer.
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
    /// Allocate a new `JvmString` object.
    pub fn new(gc_ctx: GcCtx, string: String) -> Self {
        let hash = hash_string(&string);

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

#[inline]
pub fn hash_chars(length: usize, chars: impl core::iter::Iterator<Item = u32>) -> u64 {
    let mut hash = length as u64;
    for character in chars {
        hash = hash * 11 + character as u64;
    }

    hash
}

pub fn hash_string(string: &str) -> u64 {
    hash_chars(string.len(), string.as_bytes().iter().map(|b| *b as u32))
}
