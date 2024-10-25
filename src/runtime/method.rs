use super::class::Class;
use super::context::Context;
use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};
use super::op::Op;
use super::value::Value;

use crate::classfile::attribute::Attribute;
use crate::classfile::flags::MethodFlags;
use crate::classfile::method::Method as ClassFileMethod;
use crate::classfile::reader::{FileData, Reader};
use crate::gc::{Gc, GcCtx, Trace};

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

    class: Cell<Option<Class>>,

    raw_code_data: Option<Vec<u8>>,
    method_info: RefCell<MethodInfo>,
}

impl Method {
    pub fn from_method(gc_ctx: GcCtx, method: &ClassFileMethod) -> Result<Self, Error> {
        let descriptor = MethodDescriptor::from_string(gc_ctx, method.descriptor())
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        let attributes = method.attributes();
        let mut raw_code_data = None;
        for attribute in attributes {
            if attribute.name().as_bytes() == b"Code" {
                // This performs a clone, but this code shouldn't be perf-sensitive
                raw_code_data = Some(attribute.data().to_vec());
            }
        }

        Ok(Self(Gc::new(
            gc_ctx,
            MethodData {
                descriptor,
                class: Cell::new(None),
                flags: method.flags(),
                raw_code_data,
                method_info: RefCell::new(MethodInfo::Empty),
            },
        )))
    }

    pub fn empty(gc_ctx: GcCtx, descriptor: MethodDescriptor, flags: MethodFlags) -> Self {
        Self(Gc::new(
            gc_ctx,
            MethodData {
                descriptor,
                class: Cell::new(None),
                flags,
                raw_code_data: None,
                method_info: RefCell::new(MethodInfo::Empty),
            },
        ))
    }

    pub fn exec(self, context: Context, args: &[Value]) -> Result<Option<Value>, Error> {
        // Typecheck args

        Ok(None)
    }

    pub fn flags(self) -> MethodFlags {
        self.0.flags
    }

    pub fn descriptor(self) -> MethodDescriptor {
        self.0.descriptor
    }

    pub fn set_class_and_parse_code(self, context: Context, class: Class) -> Result<(), Error> {
        // Ugh, cloned again...
        let bytecode_method_info = self
            .0
            .raw_code_data
            .clone()
            .map(|data| BytecodeMethodInfo::from_code_data(context, class, data))
            .transpose()?;

        if let Some(bytecode_method_info) = bytecode_method_info {
            *self.0.method_info.borrow_mut() = MethodInfo::Bytecode(bytecode_method_info);
        }

        Ok(())
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
    Empty,
}

impl Trace for MethodInfo {
    fn trace(&self) {
        match self {
            MethodInfo::Bytecode(bytecode_info) => bytecode_info.trace(),
            MethodInfo::Empty => {}
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
    pub fn from_code_data(context: Context, class: Class, data: Vec<u8>) -> Result<Self, Error> {
        let mut reader = FileData::new(data);

        let class_file = class.class_file().unwrap();
        let constant_pool = class_file.constant_pool();

        let max_stack = reader.read_u16()?;
        let max_locals = reader.read_u16()?;

        let code_length = reader.read_u32()? as usize;
        let code_start = reader.position();
        let mut code = Vec::with_capacity(code_length / 2);

        while reader.position() < code_start + code_length {
            code.push(Op::read_from(context, class, constant_pool, &mut reader)?);
        }

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
