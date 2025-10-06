use super::context::Context;
use super::context::{OBJECT_TO_STRING_METHOD, THROWABLE_STACK_TRACE_FIELD};
use super::object::Object;
use super::value::Value;

use crate::classfile::error::Error as ClassFileError;

use std::fmt;

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
                let result = to_string_method.exec(context, args);
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

                // Now write the stack trace, if it exists. (TODO: Shouldn't it
                // always exist?)

                let stack_trace = error_object.get_field(THROWABLE_STACK_TRACE_FIELD).object();
                if let Some(stack_trace) = stack_trace {
                    result_string.push_str(&Context::string_object_to_string(stack_trace));
                }

                result_string
            }
        }
    }
}

#[derive(Debug)]
pub enum NativeError {
    ErrorClassNotThrowable,
    ClassNotInterface,
    MethodMustHaveCode,

    InvalidArrayType,
    InvalidBranchPosition,
    InvalidDescriptor,
    InvalidJar,

    ReadError,

    VTableLookupFailed,

    CodeFellOffMethod,
    VerifyCountWrong,
    VerifyTypeWrong,

    WrongReturnType,
}

impl From<ClassFileError> for Error {
    fn from(_error: ClassFileError) -> Self {
        Error::Native(NativeError::ReadError)
    }
}
