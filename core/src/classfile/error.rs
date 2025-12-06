use crate::reader::ReadError;

#[derive(Debug)]
pub enum Error {
    ConstantPoolInvalidEntry,
    ConstantPoolTypeMismatch,
    ConstantPoolVerifyError,
    EndOfFile,
    ExpectedNonZero,
    InvalidMagic,
    InvalidString,
}

impl From<ReadError> for Error {
    fn from(value: ReadError) -> Self {
        match value {
            ReadError::EndOfFile => Self::EndOfFile,
            ReadError::InvalidMagic => Self::InvalidMagic,
            ReadError::InvalidString => Self::InvalidString,
        }
    }
}
