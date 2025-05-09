use super::descriptor::Descriptor;
use super::error::{Error, NativeError};
use super::value::Value;

use crate::classfile::field::Field as ClassFileField;
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::Cell;

// IMPORTANT NOTE: DON'T MAKE THIS Copy, WE NEED TO CREATE AN ACTUAL CLONE OF
// THE FIELD FOR OBJECT CREATION
#[derive(Clone, Debug)]
pub struct Field {
    descriptor: Descriptor,
    name: JvmString,
    value: Cell<Value>,
}

impl Field {
    pub fn from_field(gc_ctx: GcCtx, field: &ClassFileField) -> Result<Self, Error> {
        let descriptor_name = field.descriptor();

        let descriptor = Descriptor::from_string(gc_ctx, descriptor_name)
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        let name = field.name();

        let value = descriptor.default_value();

        Ok(Self {
            descriptor,
            name,
            value: Cell::new(value),
        })
    }

    pub fn descriptor(&self) -> Descriptor {
        self.descriptor
    }

    pub fn name(self) -> JvmString {
        self.name
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
        self.name.trace();
        self.value.trace();
    }
}

// This is intentionally Copy, so that a subclass can simply hold references
// to its superclass's static fields.
#[derive(Clone, Copy, Debug)]
pub struct FieldRef(Gc<FieldRefData>);

#[derive(Clone, Debug)]
struct FieldRefData {
    descriptor: Descriptor,
    name: JvmString,
    value: Cell<Value>,
}

impl FieldRef {
    pub fn from_field(gc_ctx: GcCtx, field: &ClassFileField) -> Result<Self, Error> {
        let descriptor_name = field.descriptor();

        let descriptor = Descriptor::from_string(gc_ctx, descriptor_name)
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        let name = field.name();

        let value = descriptor.default_value();

        Ok(Self(Gc::new(
            gc_ctx,
            FieldRefData {
                descriptor,
                name,
                value: Cell::new(value),
            },
        )))
    }

    pub fn descriptor(self) -> Descriptor {
        self.0.descriptor
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn value(self) -> Value {
        self.0.value.get()
    }

    pub fn set_value(self, value: Value) {
        // TODO check that value is of descriptor type
        self.0.value.set(value);
    }
}

impl Trace for FieldRef {
    fn trace(&self) {
        self.0.trace();
    }
}

impl Trace for FieldRefData {
    fn trace(&self) {
        self.descriptor.trace();
        self.name.trace();
        self.value.trace();
    }
}
