use super::class::Class;
use super::context::Context;
use super::descriptor::{MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::field::Field;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};

use std::cell::Cell;
use std::hash::{Hash, Hasher};

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

    // Creates a new instance of java.lang.Class. The caller is responsible for
    // making it a valid `Class` object (see how
    // `Context::get_or_init_java_class_for_class` does it).
    pub fn class_object(context: Context) -> Self {
        let class_class = context
            .lookup_class(context.common.java_lang_class)
            .expect("Class class should exist");

        let fields = class_class.instance_fields().to_vec().into_boxed_slice();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: class_class,
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
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn bool_array(context: Context, bools: &[bool]) -> Self {
        let value_list = bools
            .iter()
            .map(|b| Cell::new(Value::Integer((*b).into())))
            .collect::<Vec<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context
                    .lookup_class(context.common.array_bool_desc)
                    .expect("Should lookup"),
                data: FieldOrArrayData::Array(value_list.into_boxed_slice()),
            },
        ))
    }

    pub fn obj_array(context: Context, class: Class, objs: &[Option<Object>]) -> Self {
        let value_list = objs
            .iter()
            .map(|b| Cell::new(Value::Object(*b)))
            .collect::<Vec<_>>();

        // If this is an array of arrays, use an array type for its type instead of a class type
        let descriptor = if class.array_value_type().is_some() {
            ResolvedDescriptor::Array(class)
        } else {
            ResolvedDescriptor::Class(class)
        };

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context.array_class_for(descriptor),
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

    /// Create a clone of this object
    pub fn create_clone(self, gc_ctx: GcCtx) -> Object {
        let cloned_data = self.0.data.clone();

        Self(Gc::new(
            gc_ctx,
            ObjectData {
                class: self.0.class,
                data: cloned_data,
            },
        ))
    }

    pub fn call_construct(
        self,
        context: Context,
        descriptor: MethodDescriptor,
        args: &[Value],
    ) -> Result<(), Error> {
        let init_name = context.common.init_name;

        let instance_method_vtable = self.0.class.instance_method_vtable();

        let method_idx = instance_method_vtable
            .lookup((init_name, descriptor))
            .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

        let method = instance_method_vtable.get_element(method_idx);

        method.exec(context, args)?;

        Ok(())
    }

    pub fn as_ptr(self) -> *const () {
        Gc::as_ptr(self.0) as *const ()
    }
}

impl Trace for Object {
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        Gc::as_ptr(self.0) == Gc::as_ptr(other.0)
    }
}

impl Eq for Object {}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Gc::as_ptr(self.0).hash(state);
    }
}

#[derive(Debug)]
struct ObjectData {
    class: Class,

    data: FieldOrArrayData,
}

impl Trace for ObjectData {
    #[inline(always)]
    fn trace(&self) {
        self.class.trace();
        self.data.trace();
    }
}

#[derive(Clone, Debug)]
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
