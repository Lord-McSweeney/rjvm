use super::object::Object;

use crate::gc::Trace;

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Object>),
}

impl Value {
    pub fn is_wide(self) -> bool {
        match self {
            Value::Integer(_) | Value::Float(_) | Value::Object(_) => false,
            Value::Long(_) | Value::Double(_) => true,
        }
    }

    pub fn int(self) -> i32 {
        match self {
            Value::Integer(int) => int,
            _ => panic!("Expected value to be integer"),
        }
    }

    pub fn long(self) -> i64 {
        match self {
            Value::Long(long) => long,
            _ => panic!("Expected value to be long"),
        }
    }

    pub fn float(self) -> f32 {
        match self {
            Value::Float(float) => float,
            _ => panic!("Expected value to be float"),
        }
    }

    pub fn double(self) -> f64 {
        match self {
            Value::Double(double) => double,
            _ => panic!("Expected value to be double"),
        }
    }

    pub fn object(self) -> Option<Object> {
        match self {
            Value::Object(object) => object,
            _ => panic!("Expected value to be object"),
        }
    }
}

impl Trace for Value {
    #[inline(always)]
    fn trace(&self) {
        match self {
            Value::Object(object) => object.trace(),
            _ => {}
        }
    }
}
