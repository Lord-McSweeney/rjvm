use super::object::Object;

use crate::gc::Trace;

use std::cell::Cell;

#[derive(Clone, Debug)]
pub enum Array {
    ByteArray(Box<[Cell<i8>]>),
    CharArray(Box<[Cell<u16>]>),
    DoubleArray(Box<[Cell<f64>]>),
    FloatArray(Box<[Cell<f32>]>),
    IntArray(Box<[Cell<i32>]>),
    LongArray(Box<[Cell<i64>]>),
    ShortArray(Box<[Cell<i16>]>),
    ObjectArray(Box<[Cell<Option<Object>>]>),
}

impl Array {
    pub fn as_byte_array(&self) -> &[Cell<i8>] {
        match self {
            Array::ByteArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_char_array(&self) -> &[Cell<u16>] {
        match self {
            Array::CharArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_double_array(&self) -> &[Cell<f64>] {
        match self {
            Array::DoubleArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_float_array(&self) -> &[Cell<f32>] {
        match self {
            Array::FloatArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_int_array(&self) -> &[Cell<i32>] {
        match self {
            Array::IntArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_long_array(&self) -> &[Cell<i64>] {
        match self {
            Array::LongArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_short_array(&self) -> &[Cell<i16>] {
        match self {
            Array::ShortArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn as_object_array(&self) -> &[Cell<Option<Object>>] {
        match self {
            Array::ObjectArray(arr) => &arr,
            _ => unreachable!("Wrong array type"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Array::ByteArray(arr) => arr.len(),
            Array::CharArray(arr) => arr.len(),
            Array::DoubleArray(arr) => arr.len(),
            Array::FloatArray(arr) => arr.len(),
            Array::IntArray(arr) => arr.len(),
            Array::LongArray(arr) => arr.len(),
            Array::ShortArray(arr) => arr.len(),
            Array::ObjectArray(arr) => arr.len(),
        }
    }
}

impl Trace for Array {
    fn trace(&self) {
        match self {
            Array::ObjectArray(arr) => arr.trace(),
            _ => {}
        }
    }
}
