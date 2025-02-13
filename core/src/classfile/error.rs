#[derive(Debug)]
pub enum Error {
    ConstantPoolTypeMismatch,
    EndOfFile,
    ExpectedNonZero,
    InvalidString,
    MagicMismatch,
}
