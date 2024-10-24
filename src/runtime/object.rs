use super::class::Class;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};

#[derive(Clone, Copy)]
pub struct Object(Gc<ObjectData>);

impl Trace for Object {
    fn trace(&self) {
        self.0.trace();
    }
}

struct ObjectData {
    class: Class,

    slots: Box<[Value]>,
}

impl Trace for ObjectData {
    fn trace(&self) {
        self.class.trace();
        self.slots.trace();
    }
}
