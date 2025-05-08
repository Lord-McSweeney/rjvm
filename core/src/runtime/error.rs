use super::object::Object;

use crate::classfile::error::Error as ClassFileError;

use std::fmt;

pub enum Error {
    Native(NativeError),
    Java(Object),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Native(native) => write!(f, "NativeError({:?})", native),
            Error::Java(object) => {
                write!(f, "{}", object.class().dot_name())?;

                let stack_trace = object.get_field(1).object();
                if let Some(stack_trace) = stack_trace {
                    let chars = stack_trace.get_field(0).object().unwrap();
                    let chars = chars.get_array_data();
                    let chars = chars
                        .iter()
                        .map(|c| c.get().int() as u16)
                        .collect::<Vec<_>>();

                    let string = String::from_utf16_lossy(&chars);
                    write!(f, "{}", string)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum NativeError {
    ErrorClassNotThrowable,
    ClassNotInterface,
    MethodMustHaveCode,

    InvalidArrayType,
    InvalidBranchPosition,
    InvalidDescriptor,
    InvalidJar,

    ReadError,

    VTableLookupFailed,

    CodeFellOffMethod,
    VerifyCountWrong,
    VerifyTypeWrong,

    WrongReturnType,
}

impl From<ClassFileError> for Error {
    fn from(_error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
