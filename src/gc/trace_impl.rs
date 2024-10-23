use std::cell::Cell;

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

impl<T> Trace for Option<T>
where
    T: Trace,
{
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
    #[inline]
    fn trace(&self) {
        self.get().trace();
    }
}

impl<T> Trace for Vec<T>
where
    T: Trace,
{
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
    fn trace(&self) {
        self.as_ref().trace();
    }
}
