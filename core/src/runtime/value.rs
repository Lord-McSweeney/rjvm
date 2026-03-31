use super::object::Object;

use crate::gc::Trace;

use core::fmt;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Value(ValueData);

#[derive(Clone, Copy)]
enum ValueData {
    Reference(Option<Object>),
    Primitive(u64),
}

impl Value {
    // Make Value from inner value
    #[allow(non_snake_case)]
    pub fn Integer(value: i32) -> Self {
        Self(ValueData::Primitive(value as u64))
    }

    #[allow(non_snake_case)]
    pub fn Long(value: i64) -> Self {
        Self(ValueData::Primitive(value as u64))
    }

    #[allow(non_snake_case)]
    pub fn Float(value: f32) -> Self {
        Self(ValueData::Primitive(f32::to_bits(value) as u64))
    }

    #[allow(non_snake_case)]
    pub fn Double(value: f64) -> Self {
        Self(ValueData::Primitive(f64::to_bits(value) as u64))
    }

    #[allow(non_snake_case)]
    pub fn Object(object: Option<Object>) -> Self {
        Self(ValueData::Reference(object))
    }

    // Get inner value from Value
    pub fn int(self) -> i32 {
        match self.0 {
            ValueData::Primitive(p) => p as i32,
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn long(self) -> i64 {
        match self.0 {
            ValueData::Primitive(p) => p as i64,
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn float(self) -> f32 {
        match self.0 {
            ValueData::Primitive(p) => f32::from_bits(p as u32),
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn double(self) -> f64 {
        match self.0 {
            ValueData::Primitive(p) => f64::from_bits(p),
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    pub fn object(self) -> Option<Object> {
        match self.0 {
            ValueData::Reference(object) => object,
            ValueData::Primitive(_) => panic!("Expected value to be object"),
        }
    }
}

const _: () = assert!(core::mem::size_of::<Value>() <= 16);

impl Trace for Value {
    #[inline(always)]
    fn trace(&self) {
        match self.0 {
            ValueData::Reference(object) => object.trace(),
            _ => {}
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.0 {
            ValueData::Primitive(p) => write!(f, "Value({})", p),
            ValueData::Reference(o) => write!(f, "Value({:?})", o),
        }
    }
}
