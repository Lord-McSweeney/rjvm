#[derive(Debug)]
pub enum Error {
    ConstantPoolInvalidEntry,
    ConstantPoolTypeMismatch,
    ConstantPoolVerifyError,
    EndOfFile,
    ExpectedNonZero,
    InvalidString,
    MagicMismatch,
}
