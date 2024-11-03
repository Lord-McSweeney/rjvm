use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::field::Field;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub struct Object(Gc<ObjectData>);

impl Object {
    pub fn from_class(gc_ctx: GcCtx, class: Class) -> Self {
        let fields = class.instance_fields().to_vec().into_boxed_slice();

        Self(Gc::new(
            gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Fields(fields),
            },
        ))
    }

    pub fn byte_array(context: Context, chars: &[u8]) -> Self {
        let value_list = chars
            .iter()
            .map(|b| Cell::new(Value::Integer(*b as i32)))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_byte_desc)
                    .expect("Should lookup"),
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn char_array(context: Context, chars: &[u16]) -> Self {
        let value_list = chars
            .iter()
            .map(|b| Cell::new(Value::Integer(*b as i32)))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_char_desc)
                    .expect("Should lookup"),
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn int_array(context: Context, chars: &[i32]) -> Self {
        let value_list = chars
            .iter()
            .map(|b| Cell::new(Value::Integer(*b)))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_int_desc)
                    .expect("Should lookup"),
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn ptr_eq(self, other: Self) -> bool {
        Gc::ptr_eq(self.0, other.0)
    }

    pub fn is_of_class(self, class: Class) -> bool {
        self.class().matches_class(class)
    }

    pub fn is_array(self) -> bool {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => false,
            FieldOrArrayData::Array(_) => true,
        }
    }

    pub fn class(self) -> Class {
        self.0.class
    }

    pub fn get_field(self, field_idx: usize) -> Value {
        match &self.0.data {
            FieldOrArrayData::Fields(fields) => {
                let field = fields[field_idx];

                field.value()
            }
            FieldOrArrayData::Array(_) => panic!("Cannot get field of array"),
        }
    }

    pub fn set_field(self, field_idx: usize, value: Value) {
        match &self.0.data {
            FieldOrArrayData::Fields(fields) => {
                let field = fields[field_idx];
                field.set_value(value);
            }
            FieldOrArrayData::Array(_) => panic!("Cannot set field on array"),
        }
    }

    pub fn array_length(self) -> usize {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get length of object"),
            FieldOrArrayData::Array(data) => data.len(),
        }
    }

    pub fn get_byte_at_index(self, idx: usize) -> u8 {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get index of object"),
            FieldOrArrayData::Array(data) => {
                let value = data[idx].get();
                let Value::Integer(byte) = value else {
                    unreachable!();
                };

                byte as u8
            }
        }
    }

    pub fn get_char_at_index(self, idx: usize) -> u16 {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get index of object"),
            FieldOrArrayData::Array(data) => {
                let value = data[idx].get();
                let Value::Integer(character) = value else {
                    unreachable!();
                };

                character as u16
            }
        }
    }

    pub fn get_integer_at_index(self, idx: usize) -> i32 {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get index of object"),
            FieldOrArrayData::Array(data) => {
                let value = data[idx].get();
                let Value::Integer(integer) = value else {
                    unreachable!();
                };

                integer
            }
        }
    }

    pub fn set_char_at_index(self, idx: usize, value: u16) {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get index of object"),
            FieldOrArrayData::Array(data) => {
                data[idx].set(Value::Integer(value as i32));
            }
        }
    }

    pub fn get_object_at_index(self, idx: usize) -> Option<Object> {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get index of object"),
            FieldOrArrayData::Array(data) => {
                let value = data[idx].get();
                let Value::Object(obj) = value else {
                    unreachable!();
                };

                obj
            }
        }
    }

    pub fn get_array_data(&self) -> &[Cell<Value>] {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get array data of object"),
            FieldOrArrayData::Array(data) => &data,
        }
    }

    pub fn call_construct(
        self,
        context: Context,
        descriptor: MethodDescriptor,
        args: &[Value],
    ) -> Result<(), Error> {
        let init_name = context.common.init_name;

        let instance_method_vtable = self.0.class.instance_method_vtable();
        let instance_methods = self.0.class.instance_methods();

        let method_idx = instance_method_vtable
            .lookup((init_name, descriptor))
            .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

        instance_methods[method_idx].exec(context, args)?;

        Ok(())
    }
}

impl Trace for Object {
    fn trace(&self) {
        self.0.trace();
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
enum FieldOrArrayData {
    Fields(Box<[Field]>),
    Array(Box<[Cell<Value>]>),
}

impl Trace for FieldOrArrayData {
    fn trace(&self) {
        match self {
            FieldOrArrayData::Fields(data) => data.trace(),
            FieldOrArrayData::Array(data) => data.trace(),
        }
    }
}
