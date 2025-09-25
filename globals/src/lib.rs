pub mod native_impl;

pub const GLOBALS_BASE_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes-base.jar"));
pub const GLOBALS_DESKTOP_JAR: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/classes-desktop.jar"));
