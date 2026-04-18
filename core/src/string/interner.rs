use super::jvm_string::{JvmString, hash_chars, hash_string};

use crate::gc::{GcCtx, Trace};

use alloc::string::String;
use hashbrown::HashTable;
use hashbrown::hash_table::Entry;

// Unfortunately hashbrown's HashSet doesn't provide enough control to implement
// an interner this way, so we have to do it ourselves.
pub struct JvmStringInterner(HashTable<JvmString>);

impl JvmStringInterner {
    pub fn new() -> Self {
        JvmStringInterner(HashTable::with_capacity(256))
    }

    pub fn get_or_alloc(&mut self, gc_ctx: GcCtx, string: String) -> JvmString {
        let hash = hash_string(&string);

        let entry = self.0.entry(
            // Hash
            hash,
            // Eq function
            |s| **s == string,
            // Hasher function
            |s| hash_string(s),
        );

        let entry = entry.or_insert_with(|| JvmString::new(gc_ctx, string));

        *entry.get()
    }

    pub fn get_or_alloc_bytes(&mut self, gc_ctx: GcCtx, bytes: &[u8]) -> Result<JvmString, ()> {
        let hash = hash_chars(bytes.len(), bytes.iter().map(|b| *b as u32));

        let entry = self.0.entry(
            // Hash
            hash,
            // Eq function
            |s| s.as_bytes() == bytes,
            // Hasher function
            |s| hash_string(s),
        );

        let occupied_entry = match entry {
            Entry::Occupied(occupied) => occupied,
            Entry::Vacant(vacant) => {
                let string = String::from_utf8(bytes.to_vec()).map_err(|_| ())?;
                let allocated = JvmString::new(gc_ctx, string);

                vacant.insert(allocated)
            }
        };

        Ok(*occupied_entry.get())
    }
}

impl Trace for JvmStringInterner {
    fn trace(&self) {
        for string in &self.0 {
            string.trace();
        }
    }
}
