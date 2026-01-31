use super::context::Context;
use super::context::{OBJECT_TO_STRING_METHOD, THROWABLE_STACK_TRACE_FIELD};
use super::object::Object;
use super::value::Value;

use crate::classfile::error::Error as ClassFileError;
use crate::reader::ReadError;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

pub enum Error {
    Native(NativeError),
    Java(Object),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.display_infallible())
    }
}

impl Error {
    pub fn display_infallible(&self) -> String {
        match self {
            Error::Native(native) => format!("NativeError({:?})", native),
            Error::Java(object) => format!("{}", object.class().dot_name()),
        }
    }

    pub fn display(&self, context: &Context) -> String {
        match self {
            Error::Native(native) => format!("NativeError({:?})", native),
            Error::Java(error_object) => {
                let mut result_string = String::new();

                // Call `toString` on the error object
                let to_string_method = error_object
                    .class()
                    .instance_method_vtable()
                    .get_element(OBJECT_TO_STRING_METHOD);

                let args = &[Value::Object(Some(*error_object))];
                let result = context.exec_method(to_string_method, args);
                let value = match result {
                    Ok(value) => value.unwrap(),
                    Err(e) => {
                        return format!(
                            "(error while displaying error): {}",
                            e.display_infallible()
                        );
                    }
                };

                let string_obj = value.object();

                if let Some(string_obj) = string_obj {
                    result_string.push_str(&Context::string_object_to_string(string_obj));
                } else {
                    result_string.push_str("null");
                }

                result_string.push('\n');

                // Now write the stack trace, if it exists. (TODO: Shouldn't it
                // always exist?)
                // This is a little complicated because we need to interact with
                // Java code a lot.

                let stack_trace = error_object.get_field(THROWABLE_STACK_TRACE_FIELD).object();
                if let Some(stack_trace) = stack_trace {
                    let mut result = Vec::new();

                    let stack_trace = stack_trace.array_data().as_object_array();

                    // Now we have the array of stack trace elements.
                    for stack_trace_element in stack_trace {
                        let stack_trace_element = stack_trace_element.get().unwrap();

                        let to_string_method = stack_trace_element
                            .class()
                            .instance_method_vtable()
                            .get_element(OBJECT_TO_STRING_METHOD);

                        let args = &[Value::Object(Some(stack_trace_element))];
                        // We know exactly what `StackTraceElement.toString` does
                        let stringified = context
                            .exec_method(to_string_method, args)
                            .unwrap()
                            .unwrap()
                            .object()
                            .unwrap();
                        let stringified = Context::unwrap_string(stringified);

                        // "\tat "
                        result.push(b'\t' as u16);
                        result.push(b'a' as u16);
                        result.push(b't' as u16);
                        result.push(b' ' as u16);

                        result.extend_from_slice(&stringified);

                        result.push(b'\n' as u16);
                    }
                    result_string.push_str(&String::from_utf16_lossy(&result));
                }

                result_string
            }
        }
    }
}

#[derive(Debug)]
pub enum NativeError {
    InvalidJar,

    ReadError,
}

impl From<ClassFileError> for Error {
    fn from(_error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}

impl From<ReadError> for Error {
    fn from(_error: ReadError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
