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
pub struct InternedStrings(HashSet<StringObject>);

impl InternedStrings {
    pub fn new() -> Self {
        InternedStrings(HashSet::new())
    }

    pub fn intern(&mut self, string_object: Object) -> Object {
        let new_object = StringObject::new(string_object);

        // TODO use `get_or_insert` once it's stabilized
        let existing_object = self.0.get(&new_object);
        if let Some(existing_object) = existing_object {
            existing_object.object
        } else {
            self.0.insert(new_object);
            new_object.object
        }
    }
}

impl Trace for InternedStrings {
    fn trace(&self) {
        self.0.trace();
    }
}
