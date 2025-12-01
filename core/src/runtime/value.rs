use super::object::Object;

use crate::gc::Trace;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Reference(Option<Object>),
    Primitive(u64),
}

impl Value {
    // Make Value from inner value
    #[allow(non_snake_case)]
    pub fn Integer(value: i32) -> Self {
        Self::Primitive(value as u64)
    }

    #[allow(non_snake_case)]
    pub fn Long(value: i64) -> Self {
        Self::Primitive(value as u64)
    }

    #[allow(non_snake_case)]
    pub fn Float(value: f32) -> Self {
        Self::Primitive(f32::to_bits(value) as u64)
    }

    #[allow(non_snake_case)]
    pub fn Double(value: f64) -> Self {
        Self::Primitive(f64::to_bits(value) as u64)
    }

    #[allow(non_snake_case)]
    pub fn Object(object: Option<Object>) -> Self {
        Self::Reference(object)
    }

    // Get inner value from Value
    pub fn int(self) -> i32 {
        match self {
            Value::Primitive(p) => p as i32,
            Value::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn long(self) -> i64 {
        match self {
            Value::Primitive(p) => p as i64,
            Value::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn float(self) -> f32 {
        match self {
            Value::Primitive(p) => f32::from_bits(p as u32),
            Value::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn double(self) -> f64 {
        match self {
            Value::Primitive(p) => f64::from_bits(p),
            Value::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn object(self) -> Option<Object> {
        match self {
            Value::Reference(object) => object,
            Value::Primitive(_) => panic!("Expected value to be object"),
        }
    }
}

const _: () = assert!(core::mem::size_of::<Value>() <= 16);

impl Trace for Value {
    #[inline(always)]
    fn trace(&self) {
        match self {
            Value::Reference(object) => object.trace(),
            _ => {}
        }
    }
}
