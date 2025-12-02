pub(crate) mod hash_code;
pub mod native_impl;
pub(crate) mod reflect;

pub const GLOBALS_BASE_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes-base.jar"));
pub const GLOBALS_DESKTOP_JAR: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/classes-desktop.jar"));
