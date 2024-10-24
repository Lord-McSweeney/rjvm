use super::descriptor::Descriptor;

use crate::gc::{Gc, GcCtx, Trace};

use std::collections::HashMap;
use std::hash::Hash;

pub struct VTable<T>(Gc<VTableData<T>>);

// Clone and Copy can't be #[derive]d here, see https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for VTable<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for VTable<T> {}

impl<T: Copy + Eq + Hash> VTable<T> {
    pub fn empty(gc_ctx: GcCtx) -> Self {
        Self(Gc::new(
            gc_ctx,
            VTableData {
                parent: None,
                mapping: HashMap::new(),
                first_unused: 0,
            },
        ))
    }

    pub fn from_parent_and_keys(gc_ctx: GcCtx, parent: Option<VTable<T>>, keys: Vec<T>) -> Self {
        let mut first_unused = if let Some(parent) = parent {
            parent.first_unused()
        } else {
            // No parent, we can start with slot #0
            0
        };

        let mut mapping = HashMap::new();
        for key in keys {
            mapping.insert(key, first_unused);
            first_unused += 1;
        }

        Self(Gc::new(
            gc_ctx,
            VTableData {
                parent,
                mapping,
                first_unused,
            },
        ))
    }

    fn first_unused(self) -> usize {
        self.0.first_unused
    }

    pub fn lookup(self, key: T) -> Option<usize> {
        if let Some(idx) = self.0.mapping.get(&key) {
            Some(*idx)
        } else if let Some(parent) = self.0.parent {
            // Recursively lookup on parent
            parent.lookup(key)
        } else {
            // No parent and mapping didn't include key: lookup failed
            None
        }
    }
}

impl<T: Trace> Trace for VTable<T> {
    fn trace(&self) {
        self.0.trace();
    }
}

struct VTableData<T> {
    /// The parent vtable. Any lookup on a vtable that fails should defer to
    /// this parent recursively.
    parent: Option<VTable<T>>,

    /// A mapping of T (a tuple (name, descriptor) ) to slot index.
    mapping: HashMap<T, usize>,

    /// The first unused slot index for this VTable, taking into account
    /// those used by the parent vtable. This will be 0 for empty vtables.
    first_unused: usize,
}

impl<T> Trace for VTableData<T>
where
    T: Trace,
{
    fn trace(&self) {
        self.parent.trace();
        self.mapping.trace();
    }
}
