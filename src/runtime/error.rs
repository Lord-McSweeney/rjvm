use crate::classfile::error::Error as ClassFileError;

#[derive(Debug)]
pub enum Error {
    Native(NativeError),
}

#[derive(Debug)]
pub enum NativeError {
    ClassNotFound,
    InvalidDescriptor,
    InvalidJar,
    ReadError,
    VTableLookupFailed,
    WrongArgCount,
    WrongArgType,
    WrongObjectClass,
    WrongReturnType,
    WrongValueType,

    NullPointerException,
}

impl From<ClassFileError> for Error {
    fn from(error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
