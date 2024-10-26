use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::method::Method;

use crate::classfile::constant_pool::{ConstantPool, ConstantPoolEntry};
use crate::classfile::reader::{FileData, Reader};
use crate::gc::Trace;
use crate::string::JvmString;

#[derive(Clone, Copy)]
pub enum Op {
    AConstNull,
    Ldc(ConstantPoolEntry),
    ALoad(usize),
    Dup,
    Return,
    GetStatic(Class, usize),
    PutStatic(Class, usize),
    PutField(Class, usize),
    InvokeVirtual((JvmString, MethodDescriptor)),
    InvokeSpecial(Class, Method),
    New(Class),
}

impl Trace for Op {
    fn trace(&self) {
        match self {
            Op::AConstNull => {}
            Op::Ldc(entry) => {
                entry.trace();
            }
            Op::ALoad(_) => {}
            Op::Dup => {}
            Op::Return => {}
            Op::GetStatic(class, _) => {
                class.trace();
            }
            Op::PutStatic(class, _) => {
                class.trace();
            }
            Op::PutField(class, _) => {
                class.trace();
            }
            Op::InvokeVirtual((method_name, method_descriptor)) => {
                method_name.trace();
                method_descriptor.trace();
            }
            Op::InvokeSpecial(class, method) => {
                class.trace();
                method.trace();
            }
            Op::New(class) => {
                class.trace();
            }
        }
    }
}

const A_CONST_NULL: u8 = 0x01;
const LDC: u8 = 0x12;
const A_LOAD_0: u8 = 0x2A;
const A_LOAD_1: u8 = 0x2B;
const DUP: u8 = 0x59;
const RETURN: u8 = 0xB1;
const GET_STATIC: u8 = 0xB2;
const PUT_STATIC: u8 = 0xB3;
const PUT_FIELD: u8 = 0xB5;
const INVOKE_VIRTUAL: u8 = 0xB6;
const INVOKE_SPECIAL: u8 = 0xB7;
const NEW: u8 = 0xBB;

impl Op {
    pub fn read_from(
        context: Context,
        current_class: Class,
        method_return_type: Descriptor,
        constant_pool: &ConstantPool,
        data: &mut FileData,
    ) -> Result<Self, Error> {
        // TODO: Should current_class be None if this is a static method?

        let opcode = data.read_u8()?;
        match opcode {
            A_CONST_NULL => Ok(Op::AConstNull),
            LDC => {
                let constant_pool_idx = data.read_u8()?;
                let entry = constant_pool.entry(constant_pool_idx as u16)?;

                Ok(Op::Ldc(entry))
            }
            A_LOAD_0 => Ok(Op::ALoad(0)),
            A_LOAD_1 => Ok(Op::ALoad(1)),
            DUP => Ok(Op::Dup),
            RETURN => {
                if !matches!(method_return_type, Descriptor::Void) {
                    Err(Error::Native(NativeError::WrongReturnType))
                } else {
                    Ok(Op::Return)
                }
            }
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
            PUT_STATIC => {
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

                Ok(Op::PutStatic(class, field_slot))
            }
            PUT_FIELD => {
                let field_ref_idx = data.read_u16()?;
                let field_ref = constant_pool.get_field_ref(field_ref_idx)?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = context.lookup_class(class_name)?;
                let descriptor = Descriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let field_slot = class
                    .instance_field_vtable()
                    .lookup((field_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                Ok(Op::PutField(class, field_slot))
            }
            INVOKE_VIRTUAL => {
                let method_ref_idx = data.read_u16()?;
                let method_ref = constant_pool.get_method_ref(method_ref_idx)?;

                let (class_name, method_name, descriptor_name) = method_ref;

                // Method is called based on class of object on stack
                let _class = context.lookup_class(class_name)?;
                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                Ok(Op::InvokeVirtual((method_name, descriptor)))
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
            NEW => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let class = context.lookup_class(class_name)?;

                Ok(Op::New(class))
            }
            other => unimplemented!("Unimplemented opcode {}", other),
        }
    }
}
