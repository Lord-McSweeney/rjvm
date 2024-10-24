use super::object::Object;

use crate::gc::Trace;

#[derive(Clone, Copy)]
pub enum Value {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
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
