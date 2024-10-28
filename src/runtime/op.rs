use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor};
use super::error::{Error, NativeError};
use super::method::Method;

use crate::classfile::constant_pool::{ConstantPool, ConstantPoolEntry};
use crate::classfile::reader::{FileData, Reader};
use crate::gc::Trace;
use crate::string::JvmString;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Op {
    AConstNull,
    Ldc(ConstantPoolEntry),
    ILoad(usize),
    ALoad(usize),
    BaLoad,
    IStore(usize),
    Dup,
    IAdd,
    IInc(usize, i32),
    IfLt(usize),
    IfGe(usize),
    IfICmpGe(usize),
    IfICmpGt(usize),
    Goto(usize),
    Return,
    GetStatic(Class, usize),
    PutStatic(Class, usize),
    PutField(Class, usize),
    InvokeVirtual((JvmString, MethodDescriptor)),
    InvokeSpecial(Class, Method),
    New(Class),
    ArrayLength,
    AThrow,
    IfNonNull(usize),
}

impl Trace for Op {
    fn trace(&self) {
        match self {
            Op::AConstNull => {}
            Op::Ldc(entry) => {
                entry.trace();
            }
            Op::ILoad(_) => {}
            Op::ALoad(_) => {}
            Op::BaLoad => {}
            Op::IStore(_) => {}
            Op::Dup => {}
            Op::IAdd => {}
            Op::IInc(_, _) => {}
            Op::IfLt(_) => {}
            Op::IfGe(_) => {}
            Op::IfICmpGe(_) => {}
            Op::IfICmpGt(_) => {}
            Op::Goto(_) => {}
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
            Op::ArrayLength => {}
            Op::AThrow => {}
            Op::IfNonNull(_) => {}
        }
    }
}

const A_CONST_NULL: u8 = 0x01;
const LDC: u8 = 0x12;
const I_LOAD: u8 = 0x15;
const I_LOAD_2: u8 = 0x1C;
const I_LOAD_3: u8 = 0x1D;
const A_LOAD_0: u8 = 0x2A;
const A_LOAD_1: u8 = 0x2B;
const BA_LOAD: u8 = 0x33;
const I_STORE: u8 = 0x36;
const DUP: u8 = 0x59;
const I_ADD: u8 = 0x60;
const I_INC: u8 = 0x84;
const IF_LT: u8 = 0x9B;
const IF_GE: u8 = 0x9C;
const IF_I_CMP_GE: u8 = 0xA2;
const IF_I_CMP_GT: u8 = 0xA3;
const GOTO: u8 = 0xA7;
const RETURN: u8 = 0xB1;
const GET_STATIC: u8 = 0xB2;
const PUT_STATIC: u8 = 0xB3;
const PUT_FIELD: u8 = 0xB5;
const INVOKE_VIRTUAL: u8 = 0xB6;
const INVOKE_SPECIAL: u8 = 0xB7;
const NEW: u8 = 0xBB;
const ARRAY_LENGTH: u8 = 0xBE;
const A_THROW: u8 = 0xBF;
const IF_NON_NULL: u8 = 0xC7;

impl Op {
    pub fn read_ops(
        context: Context,
        current_class: Class,
        method_return_type: Descriptor,
        constant_pool: &ConstantPool,
        data: &mut FileData,
    ) -> Result<Vec<Op>, Error> {
        // TODO: Should current_class be None if this is a static method?

        let code_length = data.read_u32()? as usize;
        let code_start = data.position();
        let mut code = Vec::with_capacity(code_length / 2);

        let mut op_index = 0;
        let mut offset_to_idx_map = HashMap::new();

        while data.position() < code_start + code_length {
            offset_to_idx_map.insert(data.position() - code_start, op_index);

            code.push(Op::read_op(
                context,
                current_class,
                method_return_type,
                constant_pool,
                data,
                data.position() - code_start,
            )?);

            op_index += 1;
        }

        offset_to_idx_map.insert(data.position() - code_start, op_index);

        // Resolve branch ops' offsets
        for op in code.iter_mut() {
            match op {
                Op::IfNonNull(position)
                | Op::IfLt(position)
                | Op::IfGe(position)
                | Op::IfICmpGe(position)
                | Op::IfICmpGt(position)
                | Op::Goto(position) => {
                    *position = *offset_to_idx_map.get(position).ok_or(Error::Native(NativeError::InvalidBranchPosition))?;
                }
                _ => {}
            }
        }

        Ok(code)
    }

    fn read_op(
        context: Context,
        current_class: Class,
        method_return_type: Descriptor,
        constant_pool: &ConstantPool,
        data: &mut FileData,
        data_position: usize,
    ) -> Result<Op, Error> {
        let opcode = data.read_u8()?;
        match opcode {
            A_CONST_NULL => Ok(Op::AConstNull),
            LDC => {
                let constant_pool_idx = data.read_u8()?;
                let entry = constant_pool.entry(constant_pool_idx as u16)?;

                Ok(Op::Ldc(entry))
            }
            I_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::ILoad(local_idx as usize))
            }
            I_LOAD_2 => Ok(Op::ILoad(2)),
            I_LOAD_3 => Ok(Op::ILoad(3)),
            A_LOAD_0 => Ok(Op::ALoad(0)),
            A_LOAD_1 => Ok(Op::ALoad(1)),
            BA_LOAD => Ok(Op::BaLoad),
            I_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::IStore(local_idx as usize))
            }
            DUP => Ok(Op::Dup),
            I_ADD => Ok(Op::IAdd),
            I_INC => {
                let local_idx = data.read_u8()?;
                let constant = data.read_u8()? as i8;

                Ok(Op::IInc(local_idx as usize, constant as i32))
            }
            IF_LT => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfLt(((data_position as isize) + offset) as usize))
            }
            IF_GE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfGe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_GE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpGe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_GT => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpGt(((data_position as isize) + offset) as usize))
            }
            GOTO => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::Goto(((data_position as isize) + offset) as usize))
            }
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
            ARRAY_LENGTH => Ok(Op::ArrayLength),
            A_THROW => Ok(Op::AThrow),
            IF_NON_NULL => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfNonNull(((data_position as isize) + offset) as usize))
            }
            other => unimplemented!("Unimplemented opcode {}", other),
        }
    }
}
