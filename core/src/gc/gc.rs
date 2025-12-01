// This implements an extremely primitive mark-and-sweep garbage collector.
//
// Limitations:
//   - It is fairly unsafe
//
// Advantages:
//   - Collection can be called anytime and will collect all unreachable Gcs;
//     it is the caller's responsibility to call it properly
//   - Almost zero-cost pointers

use alloc::boxed::Box;
use core::cell::Cell;
use core::hash::{Hash, Hasher};
use core::mem::drop;
use core::ops::Deref;
use core::ptr::NonNull;

#[derive(Clone, Copy)]
enum CollectionStatus {
    NotMarked,
    Marked,
}

// NOTE the `#[repr(C, align(16))]` is necessary to allow erasing and unerasing
// to work correctly
#[repr(C, align(16))]
struct GcBox<T> {
    /// The collection status of this Gc.
    status: Cell<CollectionStatus>,

    /// The type-erased version of the previous Gc in the linked list of Gcs.
    prev: Cell<Option<Gc<()>>>,

    /// The type-erased version of the next Gc in the linked list of Gcs.
    next: Cell<Option<Gc<()>>>,

    /// The function to call to drop this Gc when it is collected.
    drop: unsafe fn(Gc<()>),

    /// The actual value held by this GcBox
    value: T,
}

pub struct Gc<T> {
    ptr: NonNull<GcBox<T>>,
}

// Clone and Copy can't be #[derive]d here, see https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T> Copy for Gc<T> {}

impl<T> core::fmt::Debug for Gc<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe { write!(f, "{:?}", self.ptr.as_ref().value) }
    }
}

impl<T> PartialEq for Gc<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.ptr.as_ref().value == other.ptr.as_ref().value }
    }
}

impl<T> Hash for Gc<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.ptr.as_ref().value.hash(state);
        }
    }
}

impl<T> Eq for Gc<T> where T: PartialEq {}

fn leaked_non_null<T>(value: T) -> NonNull<T> {
    let pointer = Box::into_raw(Box::new(value));

    NonNull::new(pointer).expect("Box::into_raw shouldn't return null")
}

impl<T> Gc<T> {
    pub fn new(gc_ctx: GcCtx, value: T) -> Self {
        // This is the previous "first" Gc (that is not the real first one)
        let previous_next = unsafe { gc_ctx.first_gc.ptr.as_ref().next.get() };

        let structure = GcBox {
            status: Cell::new(CollectionStatus::NotMarked),
            prev: Cell::new(Some(gc_ctx.first_gc)),
            // It's now the next Gc of this Gc.
            next: Cell::new(previous_next),
            drop: |gc| {
                let unerased = gc.unerased::<T>();

                let created_box = unsafe { Box::from_raw(unerased.ptr.as_ptr()) };
                drop(created_box);
            },
            value,
        };

        let created_gc = Self {
            ptr: leaked_non_null(structure),
        };
        let erased_created_gc = created_gc.erased();

        unsafe {
            // The "first" of the real first Gc is now this Gc.
            gc_ctx
                .first_gc
                .ptr
                .as_ref()
                .next
                .set(Some(erased_created_gc));

            // The "previous" of the previous "first" Gc is now this Gc.
            if let Some(previous_next) = previous_next {
                previous_next.ptr.as_ref().prev.set(Some(erased_created_gc));
            }
        }

        created_gc
    }

    pub fn ptr_eq(this: Self, other: Self) -> bool {
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }

    pub fn as_ptr(this: Self) -> *const T {
        let box_ptr = this.ptr.as_ptr();
        // TODO is there a way to do this without the `unsafe`?
        unsafe { &raw const (*box_ptr).value }
    }

    // Mark this GC without calling Trace on its inner value.
    pub fn trace_self(&self) {
        unsafe {
            let gc_box = self.ptr.as_ref();

            gc_box.status.set(CollectionStatus::Marked);
        }
    }

    fn erased(&self) -> Gc<()> {
        let ptr = self.ptr.as_ptr() as *mut GcBox<()>;

        Gc {
            ptr: NonNull::new(ptr).expect("NonNull holds non-null pointer"),
        }
    }
}

impl Gc<()> {
    pub fn new_empty() -> Self {
        let structure = GcBox {
            status: Cell::new(CollectionStatus::NotMarked),
            prev: Cell::new(None),
            next: Cell::new(None),
            drop: |_| {
                // Should be dropped manually
            },
            value: (),
        };

        Self {
            ptr: leaked_non_null(structure),
        }
    }

    fn unerased<T>(&self) -> Gc<T> {
        let ptr = self.ptr.as_ptr() as *mut GcBox<T>;

        Gc {
            ptr: NonNull::new(ptr).expect("NonNull holds non-null pointer"),
        }
    }
}

impl<T> Deref for Gc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

#[derive(Clone, Copy)]
pub struct GcCtx {
    first_gc: Gc<()>,
}

impl GcCtx {
    pub fn new() -> Self {
        Self {
            first_gc: Gc::new_empty(),
        }
    }

    // This method is safe to call as long as the following invariants are upheld:
    //   - When `trace` is called on `root`, every `Gc` registered under this `GcCtx`
    //     also has `trace` called on it
    //   - There are no `Gc`s registered under this `GcCtx` that are reachable in any
    //     way other than accessing them through `root`
    pub unsafe fn collect<T>(self, root: &T)
    where
        T: Trace,
    {
        unsafe {
            let mut current = Some(self.first_gc);
            while let Some(gc) = current {
                let gc_box = gc.ptr.as_ref();

                gc_box.status.set(CollectionStatus::NotMarked);

                current = gc_box.next.get();
            }

            root.trace();

            let mut current = Some(self.first_gc);
            while let Some(gc) = current {
                let gc_box = gc.ptr.as_ref();

                let status = gc_box.status.get();
                let prev = gc_box.prev.get();
                let next = gc_box.next.get();

                if gc.ptr.as_ptr() as usize != self.first_gc.ptr.as_ptr() as usize {
                    if matches!(status, CollectionStatus::NotMarked) {
                        // Remove it from the linked list.
                        if let Some(prev) = prev {
                            prev.ptr.as_ref().next.set(next);
                        }

                        if let Some(next) = next {
                            next.ptr.as_ref().prev.set(prev);
                        }

                        // Drop it.
                        (gc_box.drop)(gc);
                    }
                }

                current = next;
            }
        }
    }

    pub unsafe fn drop(self) {
        // The inner allocation, the empty tuple, is a ZST and doesn't
        // actually allocate; we don't need to dealloc it.
        let created_box = unsafe { Box::from_raw(self.first_gc.ptr.as_ptr()) };

        drop(created_box);
    }
}

pub trait Trace {
    fn trace(&self);
}

impl<T> Trace for Gc<T>
where
    T: Trace,
{
    fn trace(&self) {
        let gc_box = unsafe { self.ptr.as_ref() };

        // If this GC is already marked, don't trace its contents again
        if matches!(gc_box.status.get(), CollectionStatus::NotMarked) {
            gc_box.status.set(CollectionStatus::Marked);
            gc_box.value.trace();
        }
    }
}
