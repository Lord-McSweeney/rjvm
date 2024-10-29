use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::interpreter::Interpreter;
use super::native_impl::NativeMethod;
use super::op::Op;
use super::value::Value;

use crate::classfile::attribute::Attribute;
use crate::classfile::flags::MethodFlags;
use crate::classfile::method::Method as ClassFileMethod;
use crate::classfile::reader::{FileData, Reader};
use crate::gc::{Gc, GcCtx, Trace};
use crate::string::JvmString;

use std::cell::{Cell, RefCell};

#[derive(Clone, Copy)]
pub struct Method(Gc<MethodData>);

impl Trace for Method {
    fn trace(&self) {
        self.0.trace();
    }
}

struct MethodData {
    descriptor: MethodDescriptor,

    flags: MethodFlags,

    // This should only be used for debugging.
    name: Option<JvmString>,

    class: Option<Class>,

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
                class: Some(class),
                flags: method.flags(),
                name: Some(method.name()),
                method_info: RefCell::new(method_info),
            },
        )))
    }

    pub fn empty(gc_ctx: GcCtx, descriptor: MethodDescriptor, flags: MethodFlags) -> Self {
        Self(Gc::new(
            gc_ctx,
            MethodData {
                descriptor,
                class: None,
                flags,
                name: None,
                method_info: RefCell::new(MethodInfo::Empty),
            },
        ))
    }

    pub fn parse_info(self, context: Context) -> Result<(), Error> {
        let new_method_info = match &*self.0.method_info.borrow() {
            MethodInfo::BytecodeUnparsed(code_data) => {
                // Clone again...
                let bytecode_method_info = BytecodeMethodInfo::from_code_data(
                    context,
                    self.descriptor(),
                    self.class().unwrap(),
                    code_data.clone(),
                )?;

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

    pub fn exec(self, context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
        let descriptor = self.descriptor();
        let descriptor_types = descriptor.args();
        let return_type = descriptor.return_type();

        let maybe_receiver_included = if self.flags().contains(MethodFlags::STATIC) {
            0
        } else {
            1
        };

        // Typecheck args
        let mut args = args.to_vec();
        if args.len() != descriptor_types.len() + maybe_receiver_included {
            return Err(Error::Native(NativeError::WrongArgCount));
        }

        if !self.flags().contains(MethodFlags::STATIC) {
            if !matches!(args[0], Value::Object(_)) {
                panic!("Instance methods should have arg #0 be an object");
            }
        }

        for (arg, descriptor_type) in args
            .iter_mut()
            .skip(maybe_receiver_included)
            .zip(descriptor_types.iter())
        {
            *arg = arg.type_check(*descriptor_type)?;
        }

        let mut result = match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => {
                let mut interpreter = Interpreter::new(context, self, args);

                interpreter.interpret_ops(&bytecode_info.code)?
            }
            MethodInfo::BytecodeUnparsed(_) => unreachable!(),
            MethodInfo::Native(native_method) => native_method(context, &args)?,
            MethodInfo::Empty => None,
        };

        if let Some(some_result) = result {
            result = Some(some_result.type_check(return_type)?);
        } else {
            if !matches!(return_type, Descriptor::Void) {
                return Err(Error::Native(NativeError::WrongReturnType));
            }
        }

        Ok(result)
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

    pub fn class(self) -> Option<Class> {
        self.0.class
    }

    pub fn max_stack(self) -> usize {
        match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.max_stack as usize,
            _ => 0,
        }
    }

    pub fn max_locals(self) -> usize {
        match &*self.0.method_info.borrow() {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.max_locals as usize,
            _ => 0,
        }
    }
}

impl Trace for MethodData {
    fn trace(&self) {
        self.descriptor.trace();
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
    fn trace(&self) {
        match self {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.trace(),
            _ => {}
        }
    }
}

struct BytecodeMethodInfo {
    max_stack: u16,
    max_locals: u16,
    code: Vec<Op>,
    // TODO: Exceptions
}

impl BytecodeMethodInfo {
    pub fn from_code_data(
        context: Context,
        descriptor: MethodDescriptor,
        class: Class,
        data: Vec<u8>,
    ) -> Result<Self, Error> {
        let mut reader = FileData::new(data);

        let return_type = descriptor.return_type();

        let class_file = class.class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let max_stack = reader.read_u16()?;
        let max_locals = reader.read_u16()?;

        let code = Op::read_ops(context, class, return_type, constant_pool, &mut reader)?;

        Ok(Self {
            max_stack,
            max_locals,
            code,
        })
    }
}

impl Trace for BytecodeMethodInfo {
    fn trace(&self) {
        self.code.trace();
        // TODO: exceptions
    }
}
