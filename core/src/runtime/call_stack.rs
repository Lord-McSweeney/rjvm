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

    pub fn push_call(&mut self, entry: Method) {
        self.entries.push(entry);
    }

    pub fn pop_call(&mut self) {
        self.entries.pop();
    }

    pub fn display(&self) -> String {
        let mut result = String::with_capacity(self.entries.len() * 20);
        for entry in self.entries.iter().rev() {
            result.push_str(&format!(
                "\n    at {}.{}()",
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
