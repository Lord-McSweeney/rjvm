// Implementations of some native methods such as `Object.getClass`- these
// aren't environment-dependent, so we can stuff them all in one crate.

#![no_std]

#[macro_use]
extern crate alloc;

pub(crate) mod hash_code;
pub(crate) mod impls_loader;
pub(crate) mod impls_math;
pub(crate) mod impls_misc;
pub(crate) mod impls_reflect;
pub(crate) mod impls_system;
pub mod native_impl;
pub(crate) mod reflect;

pub const GLOBALS_BASE_JAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes-base.jar"));
pub const GLOBALS_DESKTOP_JAR: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/classes-desktop.jar"));
