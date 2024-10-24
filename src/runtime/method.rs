use super::descriptor::MethodDescriptor;
use super::error::{Error, NativeError};

use crate::classfile::attribute::Attribute;
use crate::classfile::method::Method as ClassFileMethod;
use crate::gc::{Gc, GcCtx};

#[derive(Clone, Copy)]
pub struct Method(Gc<MethodData>);

struct MethodData {
    descriptor: MethodDescriptor,
    method_info: MethodInfo,
}

impl Method {
    pub fn from_method(gc_ctx: GcCtx, method: &ClassFileMethod) -> Result<Self, Error> {
        let descriptor = MethodDescriptor::from_string(gc_ctx, method.descriptor())
            .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

        Ok(Self(Gc::new(
            gc_ctx,
            MethodData {
                descriptor,
                method_info: MethodInfo::Bytecode(BytecodeMethodInfo {
                    code_data: CodeData {},
                }),
            },
        )))
    }

    pub fn descriptor(&self) -> MethodDescriptor {
        self.0.descriptor
    }
}

enum MethodInfo {
    Bytecode(BytecodeMethodInfo),
}

struct BytecodeMethodInfo {
    code_data: CodeData,
}

struct CodeData {}
