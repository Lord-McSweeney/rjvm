use super::context::Context;
use super::context::{
    STACK_TRACE_ELEMENT_DECL_CLASS_FIELD, STACK_TRACE_ELEMENT_FILE_FIELD,
    STACK_TRACE_ELEMENT_IS_NATIVE_FIELD, STACK_TRACE_ELEMENT_LINE_FIELD,
    STACK_TRACE_ELEMENT_METHOD_FIELD,
};
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
    pub fn get_entries(&self, skip_count: usize) -> Vec<Method> {
        let mut result = Vec::with_capacity(self.entries.len());

        // If we are currently removing initializer frames, this will be `Some`
        let mut last_entry_class = None;

        for entry in self.entries.iter().rev().skip(skip_count) {
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
            let created_element = CallStack::stack_trace_element_from_method(context, *entry);

            result.push(created_element);
        }

        result
    }

    /// Creates a new `java.lang.StackTraceElement` from a given call stack entry.
    fn stack_trace_element_from_method(context: &Context, entry: Method) -> Object {
        let element_class = context.builtins().java_lang_stack_trace_element;
        let instance = element_class.new_instance(context.gc_ctx);

        // Set class and method name
        instance.set_field(
            STACK_TRACE_ELEMENT_DECL_CLASS_FIELD,
            Value::Object(Some(context.str_to_string(&*entry.class().name()))),
        );
        instance.set_field(
            STACK_TRACE_ELEMENT_METHOD_FIELD,
            Value::Object(Some(context.str_to_string(&*entry.name()))),
        );

        // TODO set these three fields properly

        instance.set_field(STACK_TRACE_ELEMENT_FILE_FIELD, Value::Object(None));
        instance.set_field(STACK_TRACE_ELEMENT_LINE_FIELD, Value::Integer(0));
        instance.set_field(STACK_TRACE_ELEMENT_IS_NATIVE_FIELD, Value::Integer(0));

        instance
    }
}

impl Trace for CallStack {
    fn trace(&self) {
        self.entries.trace();
    }
}
