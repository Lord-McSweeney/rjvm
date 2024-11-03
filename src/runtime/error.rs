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
            Error::Java(object) => write!(f, "{}", object.class().name()),
        }
    }
}

#[derive(Debug)]
pub enum NativeError {
    ErrorClassNotThrowable,
    ClassNotInterface,

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
    NegativeArraySizeException,
}

impl From<ClassFileError> for Error {
    fn from(error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
