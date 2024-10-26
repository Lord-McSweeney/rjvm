use super::class::Class;
use super::context::Context;
use super::field::Field;
use super::value::{Value, ValueType};

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub struct Object(Gc<ObjectData>);

impl Object {
    pub fn char_array(context: Context, bytes: &[u16]) -> Self {
        let value_list = bytes
            .iter()
            .map(|b| Value::Integer(*b as i32))
            .collect::<Vec<_>>();

        let char_array_class_name = JvmString::new(context.gc_ctx, "[C".to_string());

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(char_array_class_name)
                    .expect("Should lookup"),
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }
}

impl Trace for Object {
    fn trace(&self) {
        self.0.trace();
    }
}

struct ObjectData {
    class: Class,

    data: FieldOrArrayData,
}

impl Trace for ObjectData {
    fn trace(&self) {
        self.class.trace();
        self.data.trace();
    }
}

enum FieldOrArrayData {
    Fields(Box<[Field]>),
    Array(Box<[Value]>),
}

impl Trace for FieldOrArrayData {
    fn trace(&self) {
        match self {
            FieldOrArrayData::Fields(data) => data.trace(),
            FieldOrArrayData::Array(data) => data.trace(),
        }
    }
}
