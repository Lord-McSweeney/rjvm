use super::class::Class;
use super::context::Context;
use super::descriptor::{MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::field::Field;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};

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
                native_data: NativeData::None,
                data: FieldOrArrayData::Fields(fields),
            },
        ))
    }

    // Creates a new instance of java.lang.Class referencing the given class.
    pub fn class_object(context: Context, class: Class) -> Self {
        let class_class = context
            .lookup_class(context.common.java_lang_class)
            .expect("Class class should exist");

        let fields = class_class.instance_fields().to_vec().into_boxed_slice();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: class_class,
                native_data: NativeData::Class(class),
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
                native_data: NativeData::None,
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
                native_data: NativeData::None,
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn int_array(context: Context, ints: &[i32]) -> Self {
        let value_list = ints
            .iter()
            .map(|b| Cell::new(Value::Integer(*b)))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_int_desc)
                    .expect("Should lookup"),
                native_data: NativeData::None,
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn long_array(context: Context, longs: &[i64]) -> Self {
        let value_list = longs
            .iter()
            .map(|b| Cell::new(Value::Long(*b)))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_long_desc)
                    .expect("Should lookup"),
                native_data: NativeData::None,
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn obj_array(context: Context, class: Class, objs: &[Option<Object>]) -> Self {
        let value_list = objs
            .iter()
            .map(|b| Cell::new(Value::Object(*b)))
            .collect::<Vec<_>>();

        let descriptor = ResolvedDescriptor::Class(class);

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: Class::for_array(context, descriptor),
                native_data: NativeData::None,
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

    pub fn implements_interface(self, interface: Class) -> bool {
        self.class().implements_interface(interface)
    }

    pub fn class(self) -> Class {
        self.0.class
    }

    pub fn get_field(self, field_idx: usize) -> Value {
        match &self.0.data {
            FieldOrArrayData::Fields(fields) => {
                let field = &fields[field_idx];

                field.value()
            }
            FieldOrArrayData::Array(_) => panic!("Cannot get field of array"),
        }
    }

    pub fn set_field(self, field_idx: usize, value: Value) {
        match &self.0.data {
            FieldOrArrayData::Fields(fields) => {
                let field = &fields[field_idx];
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

    pub fn get_long_at_index(self, idx: usize) -> i64 {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get index of object"),
            FieldOrArrayData::Array(data) => {
                let value = data[idx].get();
                let Value::Long(integer) = value else {
                    unreachable!();
                };

                integer
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

    pub fn set_byte_at_index(self, idx: usize, value: u8) {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot set index of object"),
            FieldOrArrayData::Array(data) => {
                data[idx].set(Value::Integer(value as i32));
            }
        }
    }

    pub fn set_char_at_index(self, idx: usize, value: u16) {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot set index of object"),
            FieldOrArrayData::Array(data) => {
                data[idx].set(Value::Integer(value as i32));
            }
        }
    }

    pub fn set_integer_at_index(self, idx: usize, value: i32) {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot set index of object"),
            FieldOrArrayData::Array(data) => {
                data[idx].set(Value::Integer(value));
            }
        }
    }

    pub fn set_long_at_index(self, idx: usize, value: i64) {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot set index of object"),
            FieldOrArrayData::Array(data) => {
                data[idx].set(Value::Long(value));
            }
        }
    }

    pub fn set_object_at_index(self, idx: usize, value: Option<Object>) {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot set index of object"),
            FieldOrArrayData::Array(data) => {
                data[idx].set(Value::Object(value));
            }
        }
    }

    pub fn get_array_data(&self) -> &[Cell<Value>] {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get array data of object"),
            FieldOrArrayData::Array(data) => data,
        }
    }

    pub fn get_stored_class(&self) -> Class {
        match self.0.native_data {
            NativeData::Class(class) => class,
            NativeData::None => unreachable!(),
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
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
    }
}

#[derive(Debug)]
struct ObjectData {
    class: Class,

    native_data: NativeData,

    data: FieldOrArrayData,
}

impl Trace for ObjectData {
    #[inline(always)]
    fn trace(&self) {
        self.class.trace();
        self.native_data.trace();
        self.data.trace();
    }
}

#[derive(Debug)]
enum FieldOrArrayData {
    Fields(Box<[Field]>),
    Array(Box<[Cell<Value>]>),
}

impl Trace for FieldOrArrayData {
    #[inline]
    fn trace(&self) {
        match self {
            FieldOrArrayData::Fields(data) => data.trace(),
            FieldOrArrayData::Array(data) => data.trace(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum NativeData {
    None,
    Class(Class),
}

impl Trace for NativeData {
    #[inline(always)]
    fn trace(&self) {
        match self {
            NativeData::None => {}
            NativeData::Class(class) => {
                class.trace();
            }
        }
    }
}
