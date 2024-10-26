use super::descriptor::Descriptor;
use super::error::{Error, NativeError};
use super::value::Value;

use crate::classfile::field::Field as ClassFileField;
use crate::gc::{Gc, GcCtx, Trace};

use std::cell::Cell;

#[derive(Clone, Copy)]
pub struct Field(Gc<FieldData>);

struct FieldData {
    descriptor: Descriptor,
    value: Cell<Value>,
}

impl Field {
    pub fn from_field(gc_ctx: GcCtx, field: &ClassFileField) -> Result<Self, Error> {
        let descriptor_name = field.descriptor();

        let descriptor = Descriptor::from_string(gc_ctx, descriptor_name)
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        let value = descriptor.default_value();

        Ok(Self(Gc::new(
            gc_ctx,
            FieldData {
                descriptor,
                value: Cell::new(value),
            },
        )))
    }

    pub fn descriptor(self) -> Descriptor {
        self.0.descriptor
    }

    pub fn value(self) -> Value {
        self.0.value.get()
    }
}

impl Trace for Field {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for FieldData {
    fn trace(&self) {
        self.descriptor.trace();
        self.value.trace();
    }
}
