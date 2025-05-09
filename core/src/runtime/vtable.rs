use super::class::Class;

use crate::gc::{Gc, GcCtx, Trace};

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct VTable<T>(Gc<VTableData<T>>);

// Clone and Copy can't be #[derive]d here, see https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for VTable<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for VTable<T> {}

impl<T: Copy + Debug + Eq + Hash> VTable<T> {
    pub fn empty(gc_ctx: GcCtx) -> Self {
        Self(Gc::new(
            gc_ctx,
            VTableData {
                parent: None,
                class: None,
                mapping: HashMap::new(),
                first_unused: 0,
            },
        ))
    }

    pub fn from_parent_and_keys(
        gc_ctx: GcCtx,
        class: Option<Class>,
        parent: Option<VTable<T>>,
        keys: Vec<T>,
    ) -> Self {
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
                class,
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

    /// The class that created this vtable. This is entirely optional and
    /// only used for debugging.
    class: Option<Class>,

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
        self.class.trace();
        self.mapping.trace();
    }
}

pub struct OverridingVTable<T, E>(Gc<OverridingVTableData<T, E>>);

impl<T, E> Clone for OverridingVTable<T, E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, E> Copy for OverridingVTable<T, E> {}

impl<T: Copy + Debug + Eq + Hash, E: Copy + Debug> OverridingVTable<T, E> {
    pub fn empty(gc_ctx: GcCtx) -> Self {
        Self(Gc::new(
            gc_ctx,
            OverridingVTableData {
                class: None,
                mapping: HashMap::new(),
                elements: Box::new([]),
            },
        ))
    }

    pub fn from_parent_and_keys(
        gc_ctx: GcCtx,
        class: Option<Class>,
        parent: Option<OverridingVTable<T, E>>,
        data: Vec<(T, E)>,
    ) -> Self {
        let mut new_mapping = parent.map(|p| p.0.mapping.clone()).unwrap_or_default();
        let mut new_elements = parent
            .map(|p| p.0.elements.clone())
            .unwrap_or_default()
            .to_vec();

        for (key, element) in data {
            if let Some(idx) = new_mapping.get(&key) {
                // Override of function
                new_elements[*idx] = element;
            } else {
                new_mapping.insert(key, new_elements.len());
                new_elements.push(element);
            }
        }

        Self(Gc::new(
            gc_ctx,
            OverridingVTableData {
                class,
                mapping: new_mapping,
                elements: new_elements.into_boxed_slice(),
            },
        ))
    }

    pub fn lookup(self, key: T) -> Option<usize> {
        self.0.mapping.get(&key).copied()
    }

    pub fn get_element(self, index: usize) -> E {
        self.0.elements[index]
    }
}

impl<T: Trace, E: Trace> Trace for OverridingVTable<T, E> {
    fn trace(&self) {
        self.0.trace();
    }
}

struct OverridingVTableData<T, E> {
    /// The class that created this vtable. This is entirely optional and
    /// only used for debugging.
    class: Option<Class>,

    /// A mapping of T (a tuple (name, descriptor) ) to slot index.
    mapping: HashMap<T, usize>,

    /// The items on this VTable.
    elements: Box<[E]>,
}

impl<T: Trace, E: Trace> Trace for OverridingVTableData<T, E> {
    fn trace(&self) {
        self.class.trace();
        self.mapping.trace();
        self.elements.trace();
    }
}
