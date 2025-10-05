use super::array::Array;
use super::class::Class;
use super::context::Context;
use super::descriptor::{MethodDescriptor, ResolvedDescriptor};
use super::error::{Error, NativeError};
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};

use std::cell::Cell;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Object(Gc<ObjectData>);

impl Object {
    pub fn from_class(gc_ctx: GcCtx, class: Class) -> Self {
        let fields = class
            .instance_fields()
            .iter()
            .map(|f| Cell::new(f.value()))
            .collect::<Box<_>>();

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
        let class_class = context.builtins().java_lang_class;

        let fields = class_class
            .instance_fields()
            .iter()
            .map(|f| Cell::new(f.value()))
            .collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: class_class,
                data: FieldOrArrayData::Fields(fields),
            },
        ))
    }

    // Creates a new instance of java.lang.reflect.Constructor. The caller is
    // responsible for making it a valid `Constructor` object (see how
    // `Context::get_or_init_java_executable_for_method` does it).
    pub fn constructor_object(context: Context) -> Self {
        let constructor_class = context.builtins().java_lang_reflect_constructor;

        let fields = constructor_class
            .instance_fields()
            .iter()
            .map(|f| Cell::new(f.value()))
            .collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: constructor_class,
                data: FieldOrArrayData::Fields(fields),
            },
        ))
    }

    pub fn bool_array(context: Context, data: Box<[i8]>) -> Self {
        let class = context.builtins().array_bool;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::ByteArray(data)),
            },
        ))
    }

    pub fn byte_array(context: Context, data: Box<[i8]>) -> Self {
        let class = context.builtins().array_byte;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::ByteArray(data)),
            },
        ))
    }

    pub fn char_array(context: Context, data: Box<[u16]>) -> Self {
        let class = context.builtins().array_char;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::CharArray(data)),
            },
        ))
    }

    pub fn double_array(context: Context, data: Box<[f64]>) -> Self {
        let class = context.builtins().array_double;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::DoubleArray(data)),
            },
        ))
    }

    pub fn float_array(context: Context, data: Box<[f32]>) -> Self {
        let class = context.builtins().array_float;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::FloatArray(data)),
            },
        ))
    }

    pub fn int_array(context: Context, data: Box<[i32]>) -> Self {
        let class = context.builtins().array_int;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::IntArray(data)),
            },
        ))
    }

    pub fn long_array(context: Context, data: Box<[i64]>) -> Self {
        let class = context.builtins().array_long;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::LongArray(data)),
            },
        ))
    }

    pub fn short_array(context: Context, data: Box<[i16]>) -> Self {
        let class = context.builtins().array_short;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::ShortArray(data)),
            },
        ))
    }

    pub fn obj_array(context: Context, class: Class, data: Box<[Option<Object>]>) -> Self {
        // If this is an array of arrays, use an array type for its type instead of a class type
        let descriptor = if class.array_value_type().is_some() {
            ResolvedDescriptor::Array(class)
        } else {
            ResolvedDescriptor::Class(class)
        };

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: context.array_class_for(descriptor),
                data: FieldOrArrayData::Array(Array::ObjectArray(data)),
            },
        ))
    }

    pub fn ptr_eq(self, other: Self) -> bool {
        Gc::ptr_eq(self.0, other.0)
    }

    pub fn is_of_class(self, class: Class) -> bool {
        self.class().matches_class(class)
    }

    pub fn class(self) -> Class {
        self.0.class
    }

    pub fn get_field(self, field_idx: usize) -> Value {
        match &self.0.data {
            FieldOrArrayData::Fields(fields) => {
                let field = &fields[field_idx];
                field.get()
            }
            FieldOrArrayData::Array(_) => panic!("Cannot get field of array"),
        }
    }

    pub fn set_field(self, field_idx: usize, value: Value) {
        match &self.0.data {
            FieldOrArrayData::Fields(fields) => {
                let field = &fields[field_idx];
                field.set(value);
            }
            FieldOrArrayData::Array(_) => panic!("Cannot set field on array"),
        }
    }

    pub fn array_data(&self) -> &Array {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Expected an array"),
            FieldOrArrayData::Array(array) => array,
        }
    }

    pub fn array_length(self) -> usize {
        match &self.0.data {
            FieldOrArrayData::Fields(_) => panic!("Cannot get length of object"),
            FieldOrArrayData::Array(array) => array.len(),
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

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "[object {}]", self.class().name())
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

#[derive(Clone)]
enum FieldOrArrayData {
    Fields(Box<[Cell<Value>]>),
    Array(Array),
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
