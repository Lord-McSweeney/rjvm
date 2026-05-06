// Utility types

use alloc::boxed::Box;
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

/// A space-efficient runtime-sized bit set.
///
/// If more than 64 bits are stored, this struct will allocate to hold the bits.
/// This struct does not support storing more than `usize::MAX - 1` bits.
pub(crate) struct CompactBitSet {
    size: usize,
    data: CompactBitSetData,
}

impl CompactBitSet {
    pub(crate) fn empty() -> Self {
        Self {
            size: 0,
            data: CompactBitSetData::Small(0),
        }
    }

    pub(crate) fn from_iter<I>(iter: I, size: usize) -> Self
    where
        I: Iterator<Item = bool>,
    {
        let mut data = if size > 64 {
            let data = (0..size).map(|_| 0).collect::<Box<[_]>>();
            CompactBitSetData::Large(data)
        } else {
            CompactBitSetData::Small(0)
        };

        for (i, bit) in iter.enumerate() {
            if bit {
                data.set(i);
            }
        }

        Self { size, data }
    }

    pub(crate) fn get(&self, index: usize) -> bool {
        if index >= self.size {
            panic!("Out of bounds of bitset");
        }

        self.data.get(index)
    }
}

// Bits are stored as
/*
|  Byte 1      |  Byte 2             |
7 6 5 4 3 2 1 0 15 14 13 12 11 10 9 8 etc
*/
enum CompactBitSetData {
    Small(u64),
    Large(Box<[u64]>),
}

impl CompactBitSetData {
    const BITS_PER_ELEMENT: usize = u64::BITS as usize;

    /// Get a specific bit in this bitset. This will not do a size check.
    fn get(&self, index: usize) -> bool {
        // All bits in the index that are bit 6 and above (64 32 16 8 2 4 1)
        let entry_index = index / Self::BITS_PER_ELEMENT;
        // The low 6 bits in the index
        let bit_index = index & (Self::BITS_PER_ELEMENT - 1);

        match self {
            CompactBitSetData::Small(entry) => ((entry >> bit_index) & 1) == 1,
            CompactBitSetData::Large(entries) => {
                let entry = entries[entry_index];
                ((entry >> bit_index) & 1) == 1
            }
        }
    }

    /// Set a specific bit in this bitset. This does no bounds checking.
    fn set(&mut self, index: usize) {
        // All bits in the index that are bit 6 and above (64 32 16 8 2 4 1)
        let entry_index = index / Self::BITS_PER_ELEMENT;
        // The low 6 bits in the index
        let bit_index = index & (Self::BITS_PER_ELEMENT - 1);

        match self {
            CompactBitSetData::Small(entry) => {
                *entry |= 1 << bit_index;
            }
            CompactBitSetData::Large(entries) => {
                let entry = &mut entries[entry_index];
                *entry |= 1 << bit_index;
            }
        }
    }
}
