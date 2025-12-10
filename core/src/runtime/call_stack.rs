use super::context::Context;
use super::context::STACK_TRACE_ELEMENT_CREATE_METHOD;
use super::method::Method;
use super::object::Object;
use super::value::Value;

use crate::gc::Trace;

use alloc::vec::Vec;

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
    pub fn get_entries(&self) -> Vec<Method> {
        let mut result = Vec::with_capacity(self.entries.len() - 2);

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

            result.push(*entry);
        }

        result
    }

    pub fn display(context: &Context, entries: &[Method]) -> Vec<Object> {
        let mut result = Vec::with_capacity(entries.len());

        // Skip the first two entries because they are
        // `Throwable.internalFillInStackTrace` and `Throwable.fillInStackTrace`
        for entry in entries {
            let method_object = entry.get_or_init_object(context);

            let element_class = context.builtins().java_lang_stack_trace_element;
            let args = &[Value::Object(Some(method_object))];

            let create_method = element_class.static_methods()[STACK_TRACE_ELEMENT_CREATE_METHOD];

            let created_element = create_method
                .exec(context, args)
                .unwrap()
                .unwrap()
                .object()
                .unwrap();

            result.push(created_element);
        }

        result
    }
}

impl Trace for CallStack {
    fn trace(&self) {
        self.entries.trace();
    }
}
