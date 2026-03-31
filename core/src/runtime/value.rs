use super::object::Object;

use crate::gc::Trace;

use core::fmt;

/// Represents a Java value. This can be an object or a primitive (int, long,
/// float, or double).
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Value(ValueData);

#[derive(Clone, Copy)]
enum ValueData {
    Reference(Option<Object>),
    Primitive(u64),
}

impl Value {
    /// Returns a `Value` representing the given `i32` (Java `int`).
    #[allow(non_snake_case)]
    pub fn Integer(value: i32) -> Self {
        Self(ValueData::Primitive(value as u64))
    }

    /// Returns a `Value` representing the given `i64` (Java `long`).
    #[allow(non_snake_case)]
    pub fn Long(value: i64) -> Self {
        Self(ValueData::Primitive(value as u64))
    }

    /// Returns a `Value` representing the given `f32` (Java `float`).
    #[allow(non_snake_case)]
    pub fn Float(value: f32) -> Self {
        Self(ValueData::Primitive(f32::to_bits(value) as u64))
    }

    /// Returns a `Value` representing the given `f64` (Java `double`).
    #[allow(non_snake_case)]
    pub fn Double(value: f64) -> Self {
        Self(ValueData::Primitive(f64::to_bits(value) as u64))
    }

    /// Returns a `Value` representing the given [`Object`].
    ///
    /// Passing `None` to this method will result in it returning a `Value`
    /// representing Java `null`.
    #[allow(non_snake_case)]
    pub fn Object(object: Option<Object>) -> Self {
        Self(ValueData::Reference(object))
    }

    /// If this `Value` represents a `int`, returns the `i32` contained in it.
    ///
    /// This method's behavior is unspecified if the `Value` does not represent
    /// an `int`. It may panic.
    pub fn int(self) -> i32 {
        match self.0 {
            ValueData::Primitive(p) => p as i32,
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    /// If this `Value` represents a `long`, returns the `i64` contained in it.
    ///
    /// This method's behavior is unspecified if the `Value` does not represent
    /// a `long`. It may panic.
    pub fn long(self) -> i64 {
        match self.0 {
            ValueData::Primitive(p) => p as i64,
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    /// If this `Value` represents a `float`, returns the `f32` contained in it.
    ///
    /// This method's behavior is unspecified if the `Value` does not represent
    /// a `float`. It may panic.
    pub fn float(self) -> f32 {
        match self.0 {
            ValueData::Primitive(p) => f32::from_bits(p as u32),
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    /// If this `Value` represents a `double`, returns the `f64` contained in it.
    ///
    /// This method's behavior is unspecified if the `Value` does not represent
    /// a `double`. It may panic.
    pub fn double(self) -> f64 {
        match self.0 {
            ValueData::Primitive(p) => f64::from_bits(p),
            ValueData::Reference(_) => panic!("Expected value to be primitive"),
        }
    }

    /// If this `Value` represents an object, returns the [`Object`] contained
    /// in it.
    ///
    /// Returns `None` if this value represents `null`.
    ///
    /// This method's behavior is unspecified if the `Value` represents a
    /// primitive value rather than an `Object`. It may panic.
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
