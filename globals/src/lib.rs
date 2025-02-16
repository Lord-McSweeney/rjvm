pub mod native_impl;

pub const GLOBALS_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.jar"));
