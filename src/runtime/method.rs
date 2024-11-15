use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::interpreter::Interpreter;
use super::native_impl::NativeMethod;
use super::op::Op;
use super::value::Value;
use super::verify::verify_ops;

use crate::classfile::attribute::Attribute;
use crate::classfile::flags::MethodFlags;
use crate::classfile::method::Method as ClassFileMethod;
use crate::classfile::reader::{FileData, Reader};
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::{Cell, Ref, RefCell};
use std::fmt;

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

    // This should only be used for debugging
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

            MethodInfo::Native(native_method.expect("Native method lookup failed"))
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
        // Parse bytecode if it hasn't been already
        if matches!(
            &*self.0.method_info.borrow(),
            MethodInfo::BytecodeUnparsed(_)
        ) {
            self.parse_info(context)?;
        }

        // All checks are performed in the verifier

        let result = match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => {
                let frame_reference = context.frame_data.borrow();
                let mut interpreter = Interpreter::new(context, frame_reference, self, args)?;

                interpreter.interpret_ops(&bytecode_info.code, &bytecode_info.exceptions)?
            }
            MethodInfo::BytecodeUnparsed(_) => unreachable!(),
            MethodInfo::Native(native_method) => native_method(context, &args)?,
            MethodInfo::Empty => None,
        };

        Ok(result)
    }

    fn parse_info(self, context: Context) -> Result<(), Error> {
        let new_method_info = match &*self.0.method_info.borrow() {
            MethodInfo::BytecodeUnparsed(code_data) => {
                let cloned_data = code_data.clone();

                // Clone again...
                let bytecode_method_info =
                    BytecodeMethodInfo::from_code_data(context, self, self.class(), cloned_data)?;

                Some(MethodInfo::Bytecode(bytecode_method_info))
            }
            // None of the other method info types need (re-)parsing
            _ => None,
        };

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

enum MethodInfo {
    Bytecode(BytecodeMethodInfo),
    BytecodeUnparsed(Vec<u8>),
    Native(NativeMethod),
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
    pub fn from_code_data(
        context: Context,
        method: Method,
        class: Class,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        let mut reader = FileData::new(data);

        let return_type = method.descriptor().return_type();

        let class_file = class.class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let max_stack = reader.read_u16()?;
        let max_locals = reader.read_u16()?;

        let (code, offset_to_idx_map) =
            Op::read_ops(context, class, return_type, constant_pool, &mut reader)?;

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

                let throwable_class = context
                    .lookup_class(context.common.java_lang_throwable)
                    .expect("Throwable class should exist");
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
            context,
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
        })
    }
}

impl Trace for BytecodeMethodInfo {
    #[inline(always)]
    fn trace(&self) {
        self.code.trace();
        self.exceptions.trace();
    }
}
