use super::class::Class;
use super::object::Object;

use crate::classfile::error::Error as ClassFileError;
use crate::string::JvmString;

use std::fmt;

pub enum Error {
    Native(NativeError),
    Java(Object),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Native(native) => write!(f, "NativeError({:?})", native),
            Error::Java(object) => write!(f, "{}", object.class().dot_name()),
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

    ArrayStoreException,
}

impl From<ClassFileError> for Error {
    fn from(error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
