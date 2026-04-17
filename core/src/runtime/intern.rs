use super::context::STRING_DATA_FIELD;
use super::object::Object;

use crate::gc::Trace;
use crate::string::hash_chars;

use core::hash::{Hash, Hasher};
use hashbrown::HashSet;

#[derive(Clone, Copy)]
struct StringObject {
    object: Object,
    hash: u32,
}

impl StringObject {
    fn new(object: Object) -> Self {
        // Should we assert that `object` is of the String class here?

        let chars = object.get_field(STRING_DATA_FIELD).object().unwrap();
        let chars = chars.array_data().as_char_array();

        let hash = hash_chars(chars.len(), chars.iter().map(|c| c.get() as u32));

        StringObject { object, hash }
    }
}

impl Trace for StringObject {
    fn trace(&self) {
        self.object.trace();
    }
}

impl PartialEq for StringObject {
    fn eq(&self, other: &Self) -> bool {
        // Compare the characters stored in the strings
        let these_chars = self.object.get_field(STRING_DATA_FIELD).object().unwrap();
        let these_chars = these_chars.array_data().as_char_array();

        let other_chars = other.object.get_field(STRING_DATA_FIELD).object().unwrap();
        let other_chars = other_chars.array_data().as_char_array();

        these_chars == other_chars
    }
}

impl Eq for StringObject {}

impl Hash for StringObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

// TODO: Make this a weak set
// NOTE: This is the set of interned Java `String` objects, interned using the
// `String.intern` Java method.
pub struct VmInternedStrings(HashSet<StringObject>);

impl VmInternedStrings {
    pub fn new() -> Self {
        VmInternedStrings(HashSet::new())
    }

    pub fn intern(&mut self, string_object: Object) -> Object {
        let new_object = StringObject::new(string_object);

        // If the string already exists in the set, return that. Otherwise,
        // insert this one in.
        let object = self.0.get_or_insert(new_object);

        object.object
    }
}

impl Trace for VmInternedStrings {
    fn trace(&self) {
        self.0.trace();
    }
}
