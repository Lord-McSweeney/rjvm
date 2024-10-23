pub(crate) mod gc;
pub(crate) mod trace_impl;

pub use gc::{Gc, GcCtx, Trace};
