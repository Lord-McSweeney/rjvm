use super::method::Method;

use crate::gc::Trace;

pub struct CallStack {
    entries: Vec<Method>,
}

impl CallStack {
    pub fn empty() -> Self {
        CallStack {
            entries: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn push_call(&mut self, entry: Method) {
        self.entries.push(entry);
    }

    pub fn pop_call(&mut self) {
        self.entries.pop();
    }

    // This needs to do some hacky stuff to remove the error initializer frames
    // to make the call stack look correct
    pub fn display(&self) -> String {
        let mut result = String::with_capacity(self.entries.len() * 20);

        // If we are currently removing initializer frames, this will be `Some`
        let mut last_entry_class = None;

        // Skip the first two entries because they are
        // `Throwable.internalFillInStackTrace` and `Throwable.fillInStackTrace`
        for entry in self.entries.iter().rev().skip(2) {
            if *entry.class().name() == "java/lang/Throwable" {
                if *entry.name() == "<init>" {
                    last_entry_class = Some(entry.class());
                    continue;
                }
            }

            if let Some(this_last_entry_class) = last_entry_class {
                if entry.class().super_class() == Some(this_last_entry_class) {
                    if *entry.name() == "<init>" {
                        // Initializer of a subclass of `last_entry_class`,
                        // remove this frame too
                        last_entry_class = Some(entry.class());
                        continue;
                    }
                }
            }

            result.push_str(&format!(
                "    at {}.{}()\n",
                entry.class().dot_name(),
                entry.name()
            ));
        }

        result
    }
}

impl Trace for CallStack {
    fn trace(&self) {
        self.entries.trace();
    }
}
