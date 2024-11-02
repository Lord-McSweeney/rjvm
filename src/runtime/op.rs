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

#[derive(Clone, Debug)]
pub enum Op {
    AConstNull,
    IConst(i32),
    Ldc(ConstantPoolEntry),
    ILoad(usize),
    ALoad(usize),
    IaLoad,
    AaLoad,
    BaLoad,
    IStore(usize),
    AStore(usize),
    CaStore,
    Dup,
    IAdd,
    ISub,
    IDiv,
    IRem,
    INeg,
    IInc(usize, i32),
    I2C,
    IfEq(usize),
    IfNe(usize),
    IfLt(usize),
    IfGe(usize),
    IfLe(usize),
    IfICmpNe(usize),
    IfICmpGe(usize),
    IfICmpGt(usize),
    IfICmpLe(usize),
    Goto(usize),
    LookupSwitch(Box<[(i32, usize)]>, usize),
    IReturn,
    AReturn,
    Return,
    GetStatic(Class, usize),
    PutStatic(Class, usize),
    GetField(Class, usize),
    PutField(Class, usize),
    InvokeVirtual((JvmString, MethodDescriptor)),
    InvokeSpecial(Class, Method),
    InvokeStatic(Method),
    New(Class),
    NewArray(ArrayType),
    ArrayLength,
    AThrow,
    IfNonNull(usize),
}

#[derive(Clone, Copy, Debug)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}

impl Trace for Op {
    fn trace(&self) {
        match self {
            Op::AConstNull => {}
            Op::IConst(_) => {}
            Op::Ldc(entry) => {
                entry.trace();
            }
            Op::ILoad(_) => {}
            Op::ALoad(_) => {}
            Op::IaLoad => {}
            Op::AaLoad => {}
            Op::BaLoad => {}
            Op::IStore(_) => {}
            Op::AStore(_) => {}
            Op::CaStore => {}
            Op::Dup => {}
            Op::IAdd => {}
            Op::ISub => {}
            Op::IDiv => {}
            Op::IRem => {}
            Op::INeg => {}
            Op::IInc(_, _) => {}
            Op::I2C => {}
            Op::IfEq(_) => {}
            Op::IfNe(_) => {}
            Op::IfLt(_) => {}
            Op::IfGe(_) => {}
            Op::IfLe(_) => {}
            Op::IfICmpNe(_) => {}
            Op::IfICmpGe(_) => {}
            Op::IfICmpGt(_) => {}
            Op::IfICmpLe(_) => {}
            Op::Goto(_) => {}
            Op::LookupSwitch(_, _) => {}
            Op::IReturn => {}
            Op::AReturn => {}
            Op::Return => {}
            Op::GetStatic(class, _) => {
                class.trace();
            }
            Op::PutStatic(class, _) => {
                class.trace();
            }
            Op::GetField(class, _) => {
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
            Op::InvokeStatic(method) => {
                method.trace();
            }
            Op::New(class) => {
                class.trace();
            }
            Op::NewArray(_) => {}
            Op::ArrayLength => {}
            Op::AThrow => {}
            Op::IfNonNull(_) => {}
        }
    }
}

const A_CONST_NULL: u8 = 0x01;
const I_CONST_0: u8 = 0x03;
const I_CONST_1: u8 = 0x04;
const I_CONST_2: u8 = 0x05;
const I_CONST_3: u8 = 0x06;
const I_CONST_4: u8 = 0x07;
const I_CONST_5: u8 = 0x08;
const B_I_PUSH: u8 = 0x10;
const S_I_PUSH: u8 = 0x11;
const LDC: u8 = 0x12;
const I_LOAD: u8 = 0x15;
const A_LOAD: u8 = 0x19;
const I_LOAD_0: u8 = 0x1A;
const I_LOAD_1: u8 = 0x1B;
const I_LOAD_2: u8 = 0x1C;
const I_LOAD_3: u8 = 0x1D;
const A_LOAD_0: u8 = 0x2A;
const A_LOAD_1: u8 = 0x2B;
const A_LOAD_2: u8 = 0x2C;
const A_LOAD_3: u8 = 0x2D;
const IA_LOAD: u8 = 0x2E;
const AA_LOAD: u8 = 0x32;
const BA_LOAD: u8 = 0x33;
const I_STORE: u8 = 0x36;
const A_STORE: u8 = 0x3A;
const I_STORE_0: u8 = 0x3B;
const I_STORE_1: u8 = 0x3C;
const I_STORE_2: u8 = 0x3D;
const I_STORE_3: u8 = 0x3E;
const A_STORE_0: u8 = 0x4B;
const A_STORE_1: u8 = 0x4C;
const A_STORE_2: u8 = 0x4D;
const A_STORE_3: u8 = 0x4E;
const CA_STORE: u8 = 0x55;
const DUP: u8 = 0x59;
const I_ADD: u8 = 0x60;
const I_SUB: u8 = 0x64;
const I_DIV: u8 = 0x6C;
const I_REM: u8 = 0x70;
const I_NEG: u8 = 0x74;
const I_INC: u8 = 0x84;
const I2C: u8 = 0x92;
const IF_EQ: u8 = 0x99;
const IF_NE: u8 = 0x9A;
const IF_LT: u8 = 0x9B;
const IF_GE: u8 = 0x9C;
const IF_LE: u8 = 0x9E;
const IF_I_CMP_NE: u8 = 0xA0;
const IF_I_CMP_GE: u8 = 0xA2;
const IF_I_CMP_GT: u8 = 0xA3;
const IF_I_CMP_LE: u8 = 0xA4;
const GOTO: u8 = 0xA7;
const LOOKUP_SWITCH: u8 = 0xAB;
const I_RETURN: u8 = 0xAC;
const A_RETURN: u8 = 0xB0;
const RETURN: u8 = 0xB1;
const GET_STATIC: u8 = 0xB2;
const PUT_STATIC: u8 = 0xB3;
const GET_FIELD: u8 = 0xB4;
const PUT_FIELD: u8 = 0xB5;
const INVOKE_VIRTUAL: u8 = 0xB6;
const INVOKE_SPECIAL: u8 = 0xB7;
const INVOKE_STATIC: u8 = 0xB8;
const NEW: u8 = 0xBB;
const NEW_ARRAY: u8 = 0xBC;
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
    ) -> Result<(Vec<Op>, HashMap<usize, usize>), Error> {
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
                Op::IfEq(position)
                | Op::IfNe(position)
                | Op::IfLt(position)
                | Op::IfGe(position)
                | Op::IfLe(position)
                | Op::IfICmpNe(position)
                | Op::IfICmpGe(position)
                | Op::IfICmpGt(position)
                | Op::IfICmpLe(position)
                | Op::Goto(position)
                | Op::IfNonNull(position) => {
                    *position = *offset_to_idx_map
                        .get(position)
                        .ok_or(Error::Native(NativeError::InvalidBranchPosition))?;
                }
                Op::LookupSwitch(ref mut matches, default_offset) => {
                    *default_offset = *offset_to_idx_map
                        .get(default_offset)
                        .ok_or(Error::Native(NativeError::InvalidBranchPosition))?;

                    for (_, offset) in matches.iter_mut() {
                        *offset = *offset_to_idx_map
                            .get(offset)
                            .ok_or(Error::Native(NativeError::InvalidBranchPosition))?;
                    }
                }
                _ => {}
            }
        }

        Ok((code, offset_to_idx_map))
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
            I_CONST_0 => Ok(Op::IConst(0)),
            I_CONST_1 => Ok(Op::IConst(1)),
            I_CONST_2 => Ok(Op::IConst(2)),
            I_CONST_3 => Ok(Op::IConst(3)),
            I_CONST_4 => Ok(Op::IConst(4)),
            I_CONST_5 => Ok(Op::IConst(5)),
            B_I_PUSH => {
                let byte = data.read_u8()? as i32;

                Ok(Op::IConst(byte))
            }
            S_I_PUSH => {
                let byte = data.read_u16()? as i32;

                Ok(Op::IConst(byte))
            }
            LDC => {
                let constant_pool_idx = data.read_u8()?;
                let entry = constant_pool.entry(constant_pool_idx as u16)?;

                Ok(Op::Ldc(entry))
            }
            I_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::ILoad(local_idx as usize))
            }
            A_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::ALoad(local_idx as usize))
            }
            I_LOAD_0 => Ok(Op::ILoad(0)),
            I_LOAD_1 => Ok(Op::ILoad(1)),
            I_LOAD_2 => Ok(Op::ILoad(2)),
            I_LOAD_3 => Ok(Op::ILoad(3)),
            A_LOAD_0 => Ok(Op::ALoad(0)),
            A_LOAD_1 => Ok(Op::ALoad(1)),
            A_LOAD_2 => Ok(Op::ALoad(2)),
            A_LOAD_3 => Ok(Op::ALoad(3)),
            IA_LOAD => Ok(Op::IaLoad),
            AA_LOAD => Ok(Op::AaLoad),
            BA_LOAD => Ok(Op::BaLoad),
            I_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::IStore(local_idx as usize))
            }
            A_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::AStore(local_idx as usize))
            }
            I_STORE_0 => Ok(Op::IStore(0)),
            I_STORE_1 => Ok(Op::IStore(1)),
            I_STORE_2 => Ok(Op::IStore(2)),
            I_STORE_3 => Ok(Op::IStore(3)),
            A_STORE_0 => Ok(Op::AStore(0)),
            A_STORE_1 => Ok(Op::AStore(1)),
            A_STORE_2 => Ok(Op::AStore(2)),
            A_STORE_3 => Ok(Op::AStore(3)),
            CA_STORE => Ok(Op::CaStore),
            DUP => Ok(Op::Dup),
            I_ADD => Ok(Op::IAdd),
            I_SUB => Ok(Op::ISub),
            I_DIV => Ok(Op::IDiv),
            I_REM => Ok(Op::IRem),
            I_NEG => Ok(Op::INeg),
            I_INC => {
                let local_idx = data.read_u8()?;
                let constant = data.read_u8()? as i8;

                Ok(Op::IInc(local_idx as usize, constant as i32))
            }
            I2C => Ok(Op::I2C),
            IF_EQ => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfEq(((data_position as isize) + offset) as usize))
            }
            IF_NE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfNe(((data_position as isize) + offset) as usize))
            }
            IF_LT => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfLt(((data_position as isize) + offset) as usize))
            }
            IF_GE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfGe(((data_position as isize) + offset) as usize))
            }
            IF_LE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfLe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_NE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpNe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_GE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpGe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_GT => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpGt(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_LE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpLe(((data_position as isize) + offset) as usize))
            }
            GOTO => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::Goto(((data_position as isize) + offset) as usize))
            }
            LOOKUP_SWITCH => {
                let padding_bytes = (data_position + 1) % 4;
                if padding_bytes != 0 {
                    for _ in 0..(4 - padding_bytes) {
                        data.read_u8()?;
                    }
                }

                let default_offset = data.read_u32()? as i32 as isize;
                let default_offset = ((data_position as isize) + default_offset) as usize;

                let num_pairs = data.read_u32()?;
                let mut pairs = Vec::with_capacity(num_pairs as usize);
                for _ in 0..num_pairs {
                    let matching_value = data.read_u32()? as i32;

                    let offset = data.read_u32()? as i32 as isize;
                    let offset = ((data_position as isize) + offset) as usize;

                    pairs.push((matching_value, offset));
                }

                Ok(Op::LookupSwitch(pairs.into_boxed_slice(), default_offset))
            }
            I_RETURN => {
                if !matches!(
                    method_return_type,
                    Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Integer
                        | Descriptor::Short
                ) {
                    Err(Error::Native(NativeError::WrongReturnType))
                } else {
                    Ok(Op::IReturn)
                }
            }
            A_RETURN => {
                if !matches!(
                    method_return_type,
                    Descriptor::Class(_) | Descriptor::Array(_)
                ) {
                    Err(Error::Native(NativeError::WrongReturnType))
                } else {
                    Ok(Op::AReturn)
                }
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
            GET_FIELD => {
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

                Ok(Op::GetField(class, field_slot))
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
            INVOKE_STATIC => {
                let method_ref_idx = data.read_u16()?;
                let method_ref = constant_pool.get_method_ref(method_ref_idx)?;

                let (class_name, method_name, descriptor_name) = method_ref;

                let class = context.lookup_class(class_name)?;

                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let method_slot = class
                    .static_method_vtable()
                    .lookup((method_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                let method = class.static_methods()[method_slot];

                Ok(Op::InvokeStatic(method))
            }
            NEW => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let class = context.lookup_class(class_name)?;

                Ok(Op::New(class))
            }
            NEW_ARRAY => {
                let array_type = match data.read_u8()? {
                    4 => ArrayType::Boolean,
                    5 => ArrayType::Char,
                    6 => ArrayType::Float,
                    7 => ArrayType::Double,
                    8 => ArrayType::Byte,
                    9 => ArrayType::Short,
                    10 => ArrayType::Int,
                    11 => ArrayType::Long,
                    _ => return Err(Error::Native(NativeError::InvalidArrayType)),
                };

                Ok(Op::NewArray(array_type))
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

    pub fn can_throw_error(&self) -> bool {
        matches!(
            self,
            Op::IaLoad
                | Op::AaLoad
                | Op::BaLoad
                | Op::CaStore
                | Op::IDiv
                | Op::IRem
                | Op::GetField(_, _)
                | Op::PutField(_, _)
                | Op::InvokeVirtual(_)
                | Op::InvokeSpecial(_, _)
                | Op::InvokeStatic(_)
                | Op::NewArray(_)
                | Op::ArrayLength
                | Op::AThrow
        )
    }
}
