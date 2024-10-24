use super::object::Object;

use crate::gc::Trace;

#[derive(Clone, Copy)]
pub enum Value {
    Integer(i32),
    Object(Option<Object>),
}

impl Trace for Value {
    fn trace(&self) {
        match self {
            Value::Object(object) => object.trace(),
            _ => {}
        }
    }
}
