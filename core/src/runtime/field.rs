use super::class::Class;
use super::context::Context;
use super::descriptor::Descriptor;
use super::error::Error;
use super::value::Value;

use crate::classfile::class::ClassFile;
use crate::classfile::constant_pool::ConstantPoolEntry;
use crate::classfile::field::Field as ClassFileField;
use crate::classfile::flags::FieldFlags;
use crate::gc::{Gc, Trace};
use crate::reader::{FileData, Reader};
use crate::string::JvmString;

use alloc::vec::Vec;
use core::cell::Cell;

// IMPORTANT NOTE: DON'T MAKE THIS Copy, WE NEED TO CREATE AN ACTUAL CLONE OF
// THE FIELD FOR OBJECT CREATION
#[derive(Clone, Debug)]
pub struct Field {
    descriptor: Descriptor,
    flags: FieldFlags,
    name: JvmString,
    value: Cell<Value>,
}

impl Field {
    pub fn from_field(
        context: &Context,
        class_file: ClassFile,
        field: &ClassFileField,
    ) -> Result<Self, Error> {
        let descriptor = Descriptor::from_string(context, field.descriptor())?;

        let constant_value = field_constant_value(context, class_file, field)?;

        let value = if let Some(constant_value) = constant_value {
            constant_value
        } else {
            descriptor.default_value()
        };

        Ok(Self {
            descriptor,
            flags: field.flags(),
            name: field.name(),
            value: Cell::new(value),
        })
    }

    pub fn descriptor(&self) -> Descriptor {
        self.descriptor
    }

    pub fn flags(&self) -> FieldFlags {
        self.flags
    }

    pub fn name(self) -> JvmString {
        self.name
    }

    pub fn value(&self) -> Value {
        self.value.get()
    }

    pub fn set_value(&self, value: Value) {
        // Verifier checks that value is of correct type
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
    flags: FieldFlags,
    name: JvmString,
    defining_class: Class,
    value: Cell<Value>,
}

impl FieldRef {
    pub fn from_field(
        context: &Context,
        defining_class: Class,
        field: &ClassFileField,
    ) -> Result<Self, Error> {
        let class_file = defining_class.class_file().unwrap();

        let descriptor = Descriptor::from_string(context, field.descriptor())?;

        let constant_value = field_constant_value(context, class_file, field)?;

        let value = if let Some(constant_value) = constant_value {
            constant_value
        } else {
            descriptor.default_value()
        };

        Ok(Self(Gc::new(
            context.gc_ctx,
            FieldRefData {
                descriptor,
                flags: field.flags(),
                name: field.name(),
                defining_class,
                value: Cell::new(value),
            },
        )))
    }

    pub fn descriptor(self) -> Descriptor {
        self.0.descriptor
    }

    pub fn flags(&self) -> FieldFlags {
        self.0.flags
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn defining_class(self) -> Class {
        self.0.defining_class
    }

    pub fn value(self) -> Value {
        self.0.value.get()
    }

    pub fn set_value(self, value: Value) {
        // Verifier checks that value is of correct type
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
        self.defining_class.trace();
        self.value.trace();
    }
}

fn field_constant_value(
    context: &Context,
    class_file: ClassFile,
    field: &ClassFileField,
) -> Result<Option<Value>, Error> {
    for attribute in field.attributes() {
        if attribute.name().as_bytes() == b"ConstantValue" {
            let mut data = FileData::new(attribute.data());
            let cpool_index = data.read_u16_be()?;
            let constant_pool = class_file.constant_pool();
            let cpool_entry = constant_pool.entry(cpool_index)?;

            // TODO validate that this matches the descriptor
            let cpool_value = match cpool_entry {
                ConstantPoolEntry::String { string_idx } => {
                    let string = constant_pool
                        .get_utf8(string_idx)
                        .expect("Should refer to valid entry");

                    let string_chars = string.encode_utf16().collect::<Vec<_>>();

                    Value::Object(Some(context.create_string(&string_chars)))
                }
                ConstantPoolEntry::Integer { value } => Value::Integer(value),
                ConstantPoolEntry::Float { value } => Value::Float(value),
                ConstantPoolEntry::Double { value } => Value::Double(value),
                ConstantPoolEntry::Long { value } => Value::Long(value),
                _ => {
                    // TODO implement proper error handling here
                    panic!("ConstantValue was of unexpected constant pool entry type")
                }
            };

            return Ok(Some(cpool_value));
        }
    }

    Ok(None)
}
