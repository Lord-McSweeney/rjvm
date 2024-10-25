use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::method::Method;

use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::reader::{FileData, Reader};
use crate::gc::Trace;
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub enum Op {
    ALoad(usize),
    GetStatic(Class, usize),
    InvokeSpecial(Class, Method),
}

impl Trace for Op {
    fn trace(&self) {
        match self {
            Op::ALoad(_) => {}
            Op::GetStatic(class, _) => {
                class.trace();
            }
            Op::InvokeSpecial(class, method) => {
                class.trace();
                method.trace();
            }
        }
    }
}

const A_LOAD_0: u8 = 0x2A;
const A_LOAD_1: u8 = 0x2B;
const GET_STATIC: u8 = 0xB2;
const INVOKE_SPECIAL: u8 = 0xB7;

impl Op {
    pub fn read_from(
        context: Context,
        current_class: Class,
        constant_pool: &ConstantPool,
        data: &mut FileData,
    ) -> Result<Self, Error> {
        // TODO: Should current_class be None if this is a static method?

        let opcode = data.read_u8()?;
        match opcode {
            A_LOAD_0 => Ok(Op::ALoad(0)),
            A_LOAD_1 => Ok(Op::ALoad(1)),
            GET_STATIC => {
                let field_ref_idx = data.read_u16()?;
                let field_ref = constant_pool.get_field_ref(field_ref_idx)?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = context.lookup_class(class_name)?;
                let descriptor = Descriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let field_slot = class
                    .static_field_vtable()
                    .lookup((field_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                Ok(Op::GetStatic(class, field_slot))
            }
            INVOKE_SPECIAL => {
                let method_ref_idx = data.read_u16()?;
                let method_ref = constant_pool.get_method_ref(method_ref_idx)?;

                let (class_name, method_name, descriptor_name) = method_ref;

                let class = context.lookup_class(class_name)?;

                let real_class = if method_name.as_bytes() != b"<init>"
                    && !class.is_interface()
                    && current_class.has_super_class(class)
                {
                    current_class.super_class().unwrap()
                } else {
                    class
                };

                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let method_slot = real_class
                    .instance_method_vtable()
                    .lookup((method_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                let method = class.instance_methods()[method_slot];

                Ok(Op::InvokeSpecial(class, method))
            }
            other => unimplemented!("Unimplemented opcode {}", other),
        }
    }
}
