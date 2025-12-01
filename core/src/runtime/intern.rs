use super::context::STRING_DATA_FIELD;
use super::object::Object;

use crate::gc::Trace;

use hashbrown::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
struct StringObject(Object);

impl StringObject {
    fn new(object: Object) -> Self {
        // Should we assert that this is of the String class here?

        StringObject(object)
    }
}

impl Trace for StringObject {
    fn trace(&self) {
        self.0.trace();
    }
}

impl PartialEq for StringObject {
    fn eq(&self, other: &Self) -> bool {
        // Compare the characters stored in the strings
        let these_chars = self.0.get_field(STRING_DATA_FIELD).object().unwrap();
        let these_chars = these_chars.array_data().as_char_array();

        let other_chars = other.0.get_field(STRING_DATA_FIELD).object().unwrap();
        let other_chars = other_chars.array_data().as_char_array();

        these_chars == other_chars
    }
}

impl Eq for StringObject {}

impl Hash for StringObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let chars = self.0.get_field(STRING_DATA_FIELD).object().unwrap();
        let chars = chars.array_data().as_char_array();

        // TODO :)
        chars.len().hash(state);
    }
}

// TODO: Make this a weak set
pub struct InternedStrings(HashSet<StringObject>);

impl InternedStrings {
    pub fn new() -> Self {
        InternedStrings(HashSet::new())
    }

    pub fn intern(&mut self, object: Object) -> Object {
        let object = StringObject::new(object);

        // TODO use `get_or_insert` once it's stabilized
        let value = self.0.get(&object);
        if let Some(value) = value {
            return value.0;
        } else {
            self.0.insert(object);
            return object.0;
        }
    }
}

impl Trace for InternedStrings {
    fn trace(&self) {
        self.0.trace();
    }
}
