use super::context::Context;
use super::context::{OBJECT_TO_STRING_METHOD, THROWABLE_CAUSE_FIELD, THROWABLE_STACK_TRACE_FIELD};
use super::object::Object;
use super::value::Value;

use crate::classfile::error::Error as ClassFileError;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

pub struct Error(pub Object);

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.display_infallible())
    }
}

impl Error {
    pub fn from_class_file_error(context: &Context, error: ClassFileError) -> Error {
        let message = match error {
            ClassFileError::ConstantPoolIndexOutOfBounds => "Out-of-bounds constant pool index",
            ClassFileError::ConstantPoolInvalidEntry => "Invalid constant pool entry",
            ClassFileError::ConstantPoolTypeMismatch => "Type mismatch for constant pool entry",
            ClassFileError::ConstantPoolVerifyError => "Constant pool failed verification",
            ClassFileError::EndOfFile => "Truncated class file",
            ClassFileError::ExpectedNonZero => "Illegal zero constant pool index",
            ClassFileError::InvalidMagic => "Invalid magic value",
            ClassFileError::InvalidString => "Illegal UTF8 string",
        };

        let chosen_function = match error {
            ClassFileError::ConstantPoolIndexOutOfBounds => Context::verify_error,
            ClassFileError::ConstantPoolInvalidEntry => Context::verify_error,
            ClassFileError::ConstantPoolTypeMismatch => Context::verify_error,
            ClassFileError::ConstantPoolVerifyError => Context::verify_error,
            ClassFileError::EndOfFile => Context::class_format_error,
            ClassFileError::ExpectedNonZero => Context::verify_error,
            ClassFileError::InvalidMagic => Context::class_format_error,
            ClassFileError::InvalidString => Context::class_format_error,
        };

        chosen_function(context, message)
    }

    pub fn display_infallible(&self) -> String {
        format!("{}", self.0.class().dot_name())
    }

    pub fn display(&self, context: &Context) -> String {
        let mut result_string = String::new();

        let error_object = self.0;

        // TODO we should just be calling `Throwable.printStackTrace(PrintStream)`
        // instead of duplicating all the logic

        // Call `toString` on the error object
        let to_string_method = error_object
            .class()
            .instance_method_vtable()
            .get_element(OBJECT_TO_STRING_METHOD);

        let args = &[Value::Object(Some(error_object))];
        let result = context.exec_method(to_string_method, args);
        let value = match result {
            Ok(value) => value.unwrap(),
            Err(e) => {
                return format!("(error while displaying error): {}", e.display_infallible());
            }
        };

        let string_obj = value.object();

        if let Some(string_obj) = string_obj {
            result_string.push_str(&Context::string_object_to_string(string_obj));
        } else {
            result_string.push_str("null");
        }

        result_string.push('\n');

        // Get cause now- the stack trace code can cause a GC, which would
        // result in the error object being collected, which we don't want to
        // happen before we retrieve the cause
        let cause = error_object.get_field(THROWABLE_CAUSE_FIELD).object();
        // If cause is equal to the error, there is no cause
        let cause_info = cause
            .filter(|cause| !cause.ptr_eq(error_object))
            .map(|cause| {
                let mut result = String::new();
                result.push_str("Caused by: ");

                let cause_error = Error(cause);
                result.push_str(&cause_error.display(context));

                result
            });

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

        if let Some(cause_info) = cause_info {
            result_string.push_str(&cause_info);
        }

        result_string
    }
}
