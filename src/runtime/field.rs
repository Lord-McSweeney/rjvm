use super::descriptor::Descriptor;
use super::error::{Error, NativeError};
use super::value::Value;

use crate::classfile::field::Field as ClassFileField;
use crate::gc::{Gc, GcCtx, Trace};

use std::cell::Cell;

// IMPORTANT NOTE: DON'T MAKE THIS Copy, WE NEED TO CREATE AN ACTUAL CLONE OF
// THE FIELD FOR OBJECT CREATION
#[derive(Clone, Debug)]
pub struct Field {
    descriptor: Descriptor,
    value: Cell<Value>,
}

impl Field {
    pub fn from_field(gc_ctx: GcCtx, field: &ClassFileField) -> Result<Self, Error> {
        let descriptor_name = field.descriptor();

        let descriptor = Descriptor::from_string(gc_ctx, descriptor_name)
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        let value = descriptor.default_value();

        Ok(Self {
            descriptor,
            value: Cell::new(value),
        })
    }

    pub fn descriptor(&self) -> Descriptor {
        self.descriptor
    }

    pub fn value(&self) -> Value {
        self.value.get()
    }

    pub fn set_value(&self, value: Value) {
        // TODO check that value is of descriptor type
        self.value.set(value);
    }
}

impl Trace for Field {
    fn trace(&self) {
        self.descriptor.trace();
        self.value.trace();
    }
}
