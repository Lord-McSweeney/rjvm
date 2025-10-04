use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::interpreter::Interpreter;
use super::op::Op;
use super::value::Value;
use super::verify::verify_ops;

use crate::classfile::flags::MethodFlags;
use crate::classfile::method::Method as ClassFileMethod;
use crate::classfile::reader::{FileData, Reader};
use crate::gc::{Gc, Trace};
use crate::string::JvmString;

use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Method(Gc<MethodData>);

impl Trace for Method {
    #[inline(always)]
    fn trace(&self) {
        self.0.trace();
    }
}

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Method")
            .field("name", &self.name())
            .finish()
    }
}

struct MethodData {
    descriptor: MethodDescriptor,

    flags: MethodFlags,

    name: JvmString,

    class: Class,

    method_info: RefCell<MethodInfo>,
}

impl Method {
    pub fn from_method(
        context: Context,
        method: &ClassFileMethod,
        class: Class,
    ) -> Result<Self, Error> {
        let descriptor = MethodDescriptor::from_string(context.gc_ctx, method.descriptor())
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        let attributes = method.attributes();
        let mut raw_code_data = None;
        for attribute in attributes {
            if attribute.name().as_bytes() == b"Code" {
                // This performs a clone, but this code shouldn't be perf-sensitive
                raw_code_data = Some(attribute.data().to_vec());
            }
        }

        let method_info = if let Some(raw_code_data) = raw_code_data {
            MethodInfo::BytecodeUnparsed(raw_code_data)
        } else if method.flags().contains(MethodFlags::NATIVE) {
            let native_method = context.get_native_method(class.name(), method.name(), descriptor);

            if let Some(native_method) = native_method {
                MethodInfo::Native(native_method)
            } else {
                // We don't want to panic right away, as the method might not
                // end up getting called at all
                MethodInfo::NativeNotFound
            }
        } else {
            MethodInfo::Empty
        };

        Ok(Self(Gc::new(
            context.gc_ctx,
            MethodData {
                descriptor,
                class,
                flags: method.flags(),
                name: method.name(),
                method_info: RefCell::new(method_info),
            },
        )))
    }

    pub fn exec(self, context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
        // Run everything in a closure so we can handle the call stack more easily
        let closure = || -> Result<Option<Value>, Error> {
            // Parse bytecode if it hasn't been already
            if matches!(
                &*self.0.method_info.borrow(),
                MethodInfo::BytecodeUnparsed(_)
            ) {
                self.parse_info(context)?;
            }

            // All checks are performed in the verifier

            match &*self.0.method_info.borrow() {
                MethodInfo::Bytecode(bytecode_info) => {
                    for class in &bytecode_info.class_dependencies {
                        class.run_clinit(context)?;
                    }

                    let frame_reference = &context.frame_data;
                    let mut interpreter = Interpreter::new(context, frame_reference, self, args)?;

                    interpreter.interpret_ops(&bytecode_info.code, &bytecode_info.exceptions)
                }
                MethodInfo::BytecodeUnparsed(_) => unreachable!(),
                MethodInfo::Native(native_method) => native_method(context, &args),
                MethodInfo::NativeNotFound => {
                    panic!(
                        "associated native method for {}.{} not found",
                        self.class().name(),
                        self.name()
                    );
                }
                MethodInfo::Empty => panic!("cannot call method without body"),
            }
        };

        context.push_call(self);
        let result = closure();
        context.pop_call();
        result
    }

    fn parse_info(self, context: Context) -> Result<(), Error> {
        let borrow = self.0.method_info.borrow();

        let new_method_info = match &*borrow {
            MethodInfo::BytecodeUnparsed(code_data) => {
                let bytecode_method_info =
                    BytecodeMethodInfo::from_code_data(context, self, &*code_data)?;

                Some(MethodInfo::Bytecode(bytecode_method_info))
            }
            // None of the other method info types need (re-)parsing
            _ => None,
        };

        drop(borrow);

        if let Some(new_method_info) = new_method_info {
            *self.0.method_info.borrow_mut() = new_method_info;
        }

        Ok(())
    }

    pub fn descriptor(self) -> MethodDescriptor {
        self.0.descriptor
    }

    pub fn arg_count(self) -> usize {
        self.descriptor().args().len()
    }

    pub fn physical_arg_count(self) -> usize {
        self.descriptor().physical_arg_count()
    }

    pub fn flags(self) -> MethodFlags {
        self.0.flags
    }

    pub fn name(self) -> JvmString {
        self.0.name
    }

    pub fn class(self) -> Class {
        self.0.class
    }

    pub fn max_stack(self) -> usize {
        match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.max_stack as usize,
            _ => panic!("Should not call max_stack for non-Bytecode method info"),
        }
    }

    pub fn max_locals(self) -> usize {
        match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.max_locals as usize,
            _ => panic!("Should not call max_locals for non-Bytecode method info"),
        }
    }
}

impl Trace for MethodData {
    #[inline(always)]
    fn trace(&self) {
        self.descriptor.trace();
        self.name.trace();
        self.class.trace();
        self.method_info.trace();
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        Gc::as_ptr(self.0) == Gc::as_ptr(other.0)
    }
}

impl Eq for Method {}

impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Gc::as_ptr(self.0).hash(state);
    }
}

enum MethodInfo {
    Bytecode(BytecodeMethodInfo),
    BytecodeUnparsed(Vec<u8>),
    Native(NativeMethod),
    NativeNotFound,
    Empty,
}

impl Trace for MethodInfo {
    #[inline(always)]
    fn trace(&self) {
        match self {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.trace(),
            MethodInfo::Native(native_method) => native_method.trace(),
            _ => {}
        }
    }
}

struct BytecodeMethodInfo {
    max_stack: u16,
    max_locals: u16,
    code: Vec<Op>,
    exceptions: Vec<Exception>,

    class_dependencies: Vec<Class>,
}

pub struct Exception {
    // Inclusive
    pub start: usize,

    // Exclusive
    pub end: usize,

    pub target: usize,

    pub catch_class: Option<Class>,
}

impl Trace for Exception {
    #[inline(always)]
    fn trace(&self) {
        self.catch_class.trace();
    }
}

impl BytecodeMethodInfo {
    pub fn from_code_data(context: Context, method: Method, data: &[u8]) -> Result<Self, Error> {
        let mut reader = FileData::new(data);

        let class_file = method.class().class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let max_stack = reader.read_u16()?;
        let max_locals = reader.read_u16()?;

        let (code, offset_to_idx_map, class_dependencies) =
            Op::read_ops(context, method, constant_pool, &mut reader)?;

        let exception_count = reader.read_u16()?;
        let mut exceptions = Vec::with_capacity(exception_count as usize);
        for _ in 0..exception_count {
            let start_offset = reader.read_u16()? as usize;
            let end_offset = reader.read_u16()? as usize;
            let target_offset = reader.read_u16()? as usize;

            let start = offset_to_idx_map
                .get(&start_offset)
                .copied()
                .ok_or(Error::Native(NativeError::ErrorClassNotThrowable))?;
            let end = offset_to_idx_map
                .get(&end_offset)
                .copied()
                .ok_or(Error::Native(NativeError::ErrorClassNotThrowable))?;
            let target = offset_to_idx_map
                .get(&target_offset)
                .copied()
                .ok_or(Error::Native(NativeError::ErrorClassNotThrowable))?;

            let class_idx = reader.read_u16()?;
            let class = if class_idx != 0 {
                let class_name = constant_pool.get_class(class_idx)?;
                let class = context.lookup_class(class_name)?;

                let throwable_class = context.builtins().java_lang_throwable;
                if !class.matches_class(throwable_class) {
                    return Err(Error::Native(NativeError::ErrorClassNotThrowable));
                }

                Some(class)
            } else {
                None
            };

            exceptions.push(Exception {
                start,
                end,
                target,
                catch_class: class,
            });
        }

        verify_ops(
            method,
            max_stack as usize,
            max_locals as usize,
            &code,
            &exceptions,
        )?;

        Ok(Self {
            max_stack,
            max_locals,
            code,
            exceptions,
            class_dependencies,
        })
    }
}

impl Trace for BytecodeMethodInfo {
    #[inline(always)]
    fn trace(&self) {
        self.code.trace();
        self.exceptions.trace();
        self.class_dependencies.trace();
    }
}

pub type NativeMethod = for<'a> fn(Context, &[Value]) -> Result<Option<Value>, Error>;

impl Trace for NativeMethod {
    fn trace(&self) {}
}
