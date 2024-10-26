use super::descriptor::Descriptor;
use super::error::{Error, NativeError};
use super::object::Object;

use crate::gc::Trace;

pub enum ValueType {
    Integer,
    Long,
    Float,
    Double,
    Reference,
}

#[derive(Clone, Copy)]
pub enum Value {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Object>),
}

impl Value {
    pub fn type_check(self, descriptor: Descriptor) -> Result<Self, Error> {
        match self {
            Value::Integer(_) => {
                // TODO: ...these are probably supposed to result in wrapping, aren't they
                if !matches!(
                    descriptor,
                    Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer
                ) {
                    return Err(Error::Native(NativeError::WrongArgType));
                }
            }
            Value::Long(_) => {
                if !matches!(descriptor, Descriptor::Long) {
                    return Err(Error::Native(NativeError::WrongArgType));
                }
            }
            Value::Float(_) => {
                if !matches!(descriptor, Descriptor::Float) {
                    return Err(Error::Native(NativeError::WrongArgType));
                }
            }
            Value::Double(_) => {
                if !matches!(descriptor, Descriptor::Double) {
                    return Err(Error::Native(NativeError::WrongArgType));
                }
            }
            Value::Object(_) => {
                if !matches!(descriptor, Descriptor::Class(_) | Descriptor::Array(_)) {
                    return Err(Error::Native(NativeError::WrongArgType));
                }
            }
        }

        Ok(self)
    }
}

impl Trace for Value {
    fn trace(&self) {
        match self {
            Value::Object(object) => object.trace(),
            _ => {}
        }
    }
}
