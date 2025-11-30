use std::cell::{Cell, OnceCell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};

use super::gc::Trace;

macro_rules! static_trace {
    ($type:ty) => {
        impl Trace for $type {
            #[inline]
            fn trace(&self) {}
        }
    };
}

static_trace!(u8);
static_trace!(u16);
static_trace!(u32);
static_trace!(u64);
static_trace!(usize);
static_trace!(i8);
static_trace!(i16);
static_trace!(i32);
static_trace!(i64);
static_trace!(isize);

static_trace!(f32);
static_trace!(f64);

static_trace!(bool);
static_trace!(char);

static_trace!(String);
static_trace!(());

impl<T> Trace for Option<T>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        if let Some(value) = self {
            value.trace();
        }
    }
}

impl<T> Trace for Cell<T>
where
    T: Trace,
    T: Copy,
{
    #[inline(always)]
    fn trace(&self) {
        self.get().trace();
    }
}

impl<T> Trace for Vec<T>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        for value in self {
            value.trace();
        }
    }
}

impl<T> Trace for VecDeque<T>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        for value in self {
            value.trace();
        }
    }
}

impl<T> Trace for Box<T>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        self.as_ref().trace();
    }
}

impl<T> Trace for Box<[T]>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        for value in self.as_ref() {
            value.trace();
        }
    }
}

impl<T> Trace for RefCell<T>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        self.borrow().trace();
    }
}

impl<T> Trace for OnceCell<T>
where
    T: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        if let Some(inner) = self.get() {
            inner.trace();
        }
    }
}

impl<K, V, S> Trace for HashMap<K, V, S>
where
    K: Trace,
    V: Trace,
{
    #[inline]
    fn trace(&self) {
        for (k, v) in self {
            k.trace();
            v.trace();
        }
    }
}

impl<T, S> Trace for HashSet<T, S>
where
    T: Trace,
{
    #[inline]
    fn trace(&self) {
        for item in self {
            item.trace();
        }
    }
}

impl<A, B> Trace for (A, B)
where
    A: Trace,
    B: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
        self.1.trace();
    }
}

impl<A, B, C> Trace for (A, B, C)
where
    A: Trace,
    B: Trace,
    C: Trace,
{
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
        self.1.trace();
        self.2.trace();
    }
}
