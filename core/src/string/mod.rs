pub(crate) mod interner;
pub(crate) mod jvm_string;

pub use interner::JvmStringInterner;
pub use jvm_string::JvmString;
pub(crate) use jvm_string::hash_chars;
