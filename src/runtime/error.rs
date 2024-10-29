use super::object::Object;

use crate::classfile::error::Error as ClassFileError;

#[derive(Debug)]
pub enum Error {
    Native(NativeError),
    Java(Object),
}

#[derive(Debug)]
pub enum NativeError {
    ClassNotFound,
    InvalidBranchPosition,
    InvalidDescriptor,
    InvalidJar,
    ReadError,
    VTableLookupFailed,
    WrongArgCount,
    WrongArgType,
    WrongObjectClass,
    WrongReturnType,
    WrongValueType,

    ArrayIndexOutOfBoundsException,
}

impl From<ClassFileError> for Error {
    fn from(error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
