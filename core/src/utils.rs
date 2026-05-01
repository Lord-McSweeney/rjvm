// Utility types

use core::ops::Deref;

/// A struct that implements `Sync` and holds an arbitary value.
pub(crate) struct Syncable<T> {
    value: T,
}

impl<T> Syncable<T> {
    /// SAFETY: The created `Syncable` must never be shared between threads.
    pub(crate) const unsafe fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Deref for Syncable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

unsafe impl<T> Sync for Syncable<T> {}
