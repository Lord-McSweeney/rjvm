use super::array::Array;
use super::class::Class;
use super::context::Context;
use super::descriptor::ResolvedDescriptor;
use super::error::Error;
use super::loader::ClassLoader;
use super::value::Value;

use crate::gc::{Gc, GcCtx, Trace};

use alloc::boxed::Box;
use core::cell::Cell;
use core::fmt;
use core::hash::{Hash, Hasher};

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

    // Creates a new instance of `java.lang.Class`. The caller is
    // responsible for making it a valid `Class` object (see how
    // `Class::get_or_init_object` does it).
    pub fn class_object(context: &Context) -> Self {
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
    // `Method::get_or_init_object` does it).
    pub fn constructor_object(context: &Context) -> Self {
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

    // Creates a new instance of java.lang.reflect.Method. The caller is
    // responsible for making it a valid `Method` object (see how
    // `Method::get_or_init_object` does it).
    pub fn method_object(context: &Context) -> Self {
        let method_class = context.builtins().java_lang_reflect_method;

        let fields = method_class
            .instance_fields()
            .iter()
            .map(|f| Cell::new(f.value()))
            .collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: method_class,
                data: FieldOrArrayData::Fields(fields),
            },
        ))
    }

    pub fn bool_array(context: &Context, data: Box<[i8]>) -> Self {
        let class = context.primitive_arrays().array_bool;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::ByteArray(data)),
            },
        ))
    }

    pub fn byte_array(context: &Context, data: Box<[i8]>) -> Self {
        let class = context.primitive_arrays().array_byte;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::ByteArray(data)),
            },
        ))
    }

    pub fn char_array(context: &Context, data: Box<[u16]>) -> Self {
        let class = context.primitive_arrays().array_char;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::CharArray(data)),
            },
        ))
    }

    pub fn double_array(context: &Context, data: Box<[f64]>) -> Self {
        let class = context.primitive_arrays().array_double;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::DoubleArray(data)),
            },
        ))
    }

    pub fn float_array(context: &Context, data: Box<[f32]>) -> Self {
        let class = context.primitive_arrays().array_float;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::FloatArray(data)),
            },
        ))
    }

    pub fn int_array(context: &Context, data: Box<[i32]>) -> Self {
        let class = context.primitive_arrays().array_int;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::IntArray(data)),
            },
        ))
    }

    pub fn long_array(context: &Context, data: Box<[i64]>) -> Self {
        let class = context.primitive_arrays().array_long;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::LongArray(data)),
            },
        ))
    }

    pub fn short_array(context: &Context, data: Box<[i16]>) -> Self {
        let class = context.primitive_arrays().array_short;

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class,
                data: FieldOrArrayData::Array(Array::ShortArray(data)),
            },
        ))
    }

    /// NOTE: `elem_class` is the class of each element, not of the whole array
    pub fn obj_array(context: &Context, elem_class: Class, data: Box<[Option<Object>]>) -> Self {
        // If this is an array of arrays, use an array type for its type instead of a class type
        let descriptor = if elem_class.array_value_type().is_some() {
            ResolvedDescriptor::Array(elem_class)
        } else {
            ResolvedDescriptor::Class(elem_class)
        };

        let data = data.into_iter().map(Cell::new).collect::<Box<_>>();

        Self(Gc::new(
            context.gc_ctx,
            ObjectData {
                class: ClassLoader::array_class_for(context, descriptor),
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

    pub fn as_ptr(self) -> *const () {
        Gc::as_ptr(self.0) as *const ()
    }
}

const _: () = assert!(core::mem::size_of::<Object>() <= 8);
const _: () = assert!(core::mem::size_of::<Option<Object>>() <= 8);

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

// "[Ljava/lang/Object;".clone()Ljava/lang/Object;
// "[I".clone()Ljava/lang/Object;
// "[J".clone()Ljava/lang/Object;
// "[Z".clone()Ljava/lang/Object;
// etc
pub fn array_clone_method(context: &Context, args: &[Value]) -> Result<Option<Value>, Error> {
    // Receiver should never be null
    let this_array = args[0].object().unwrap();

    let cloned_data = this_array.array_data().clone();

    let new_array = Object(Gc::new(
        context.gc_ctx,
        ObjectData {
            class: this_array.class(),
            data: FieldOrArrayData::Array(cloned_data),
        },
    ));

    Ok(Some(Value::Object(Some(new_array))))
}
