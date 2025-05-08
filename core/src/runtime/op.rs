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
    LConst(i8),
    DConst(f64),
    Ldc(ConstantPoolEntry),
    Ldc2(ConstantPoolEntry),
    ILoad(usize),
    LLoad(usize),
    DLoad(usize),
    ALoad(usize),
    IaLoad,
    LaLoad,
    AaLoad,
    BaLoad,
    CaLoad,
    IStore(usize),
    LStore(usize),
    DStore(usize),
    AStore(usize),
    IaStore,
    LaStore,
    AaStore,
    BaStore,
    CaStore,
    Pop,
    Dup,
    DupX1,
    Dup2,
    IAdd,
    LAdd,
    DAdd,
    ISub,
    LSub,
    DSub,
    IMul,
    DMul,
    IDiv,
    DDiv,
    IRem,
    INeg,
    IShl,
    LShl,
    IShr,
    LShr,
    LUshr,
    IAnd,
    LAnd,
    IOr,
    LOr,
    LXor,
    IInc(usize, i32),
    I2L,
    I2D,
    L2I,
    D2I,
    I2B,
    I2C,
    I2S,
    LCmp,
    IfEq(usize),
    IfNe(usize),
    IfLt(usize),
    IfGe(usize),
    IfGt(usize),
    IfLe(usize),
    IfICmpEq(usize),
    IfICmpNe(usize),
    IfICmpLt(usize),
    IfICmpGe(usize),
    IfICmpGt(usize),
    IfICmpLe(usize),
    IfACmpEq(usize),
    IfACmpNe(usize),
    Goto(usize),
    TableSwitch(i32, i32, Box<[usize]>, usize),
    LookupSwitch(Box<[(i32, usize)]>, usize),
    IReturn,
    AReturn,
    Return,
    GetStatic(Class, usize),
    PutStatic(Class, usize),
    GetField(Class, usize),
    PutField(Class, usize),
    InvokeVirtual(Class, usize),
    InvokeSpecial(Class, Method),
    InvokeStatic(Method),
    InvokeInterface(Class, (JvmString, MethodDescriptor)),
    New(Class),
    NewArray(ArrayType),
    ANewArray(Class),
    ArrayLength,
    AThrow,
    CheckCast(Class),
    InstanceOf(Class),
    MultiANewArray(Class, u8),
    IfNull(usize),
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
            Op::LConst(_) => {}
            Op::DConst(_) => {}
            Op::Ldc(entry) | Op::Ldc2(entry) => {
                entry.trace();
            }
            Op::ILoad(_) => {}
            Op::LLoad(_) => {}
            Op::DLoad(_) => {}
            Op::ALoad(_) => {}
            Op::IaLoad => {}
            Op::LaLoad => {}
            Op::AaLoad => {}
            Op::BaLoad => {}
            Op::CaLoad => {}
            Op::IStore(_) => {}
            Op::LStore(_) => {}
            Op::DStore(_) => {}
            Op::AStore(_) => {}
            Op::IaStore => {}
            Op::LaStore => {}
            Op::AaStore => {}
            Op::BaStore => {}
            Op::CaStore => {}
            Op::Pop => {}
            Op::Dup => {}
            Op::DupX1 => {}
            Op::Dup2 => {}
            Op::IAdd => {}
            Op::LAdd => {}
            Op::DAdd => {}
            Op::ISub => {}
            Op::LSub => {}
            Op::DSub => {}
            Op::IMul => {}
            Op::DMul => {}
            Op::IDiv => {}
            Op::DDiv => {}
            Op::IRem => {}
            Op::INeg => {}
            Op::IShl => {}
            Op::LShl => {}
            Op::IShr => {}
            Op::LShr => {}
            Op::LUshr => {}
            Op::IAnd => {}
            Op::LAnd => {}
            Op::IOr => {}
            Op::LOr => {}
            Op::LXor => {}
            Op::IInc(_, _) => {}
            Op::I2L => {}
            Op::I2D => {}
            Op::L2I => {}
            Op::D2I => {}
            Op::I2B => {}
            Op::I2C => {}
            Op::I2S => {}
            Op::LCmp => {}
            Op::IfEq(_) => {}
            Op::IfNe(_) => {}
            Op::IfLt(_) => {}
            Op::IfGe(_) => {}
            Op::IfGt(_) => {}
            Op::IfLe(_) => {}
            Op::IfICmpEq(_) => {}
            Op::IfICmpNe(_) => {}
            Op::IfICmpLt(_) => {}
            Op::IfICmpGe(_) => {}
            Op::IfICmpGt(_) => {}
            Op::IfICmpLe(_) => {}
            Op::IfACmpEq(_) => {}
            Op::IfACmpNe(_) => {}
            Op::Goto(_) => {}
            Op::TableSwitch(_, _, _, _) => {}
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
            Op::InvokeVirtual(class, _) => {
                class.trace();
            }
            Op::InvokeSpecial(class, method) => {
                class.trace();
                method.trace();
            }
            Op::InvokeStatic(method) => {
                method.trace();
            }
            Op::InvokeInterface(class, (method_name, method_descriptor)) => {
                class.trace();
                method_name.trace();
                method_descriptor.trace();
            }
            Op::New(class) => {
                class.trace();
            }
            Op::NewArray(_) => {}
            Op::ANewArray(class) => {
                class.trace();
            }
            Op::ArrayLength => {}
            Op::AThrow => {}
            Op::CheckCast(class) => {
                class.trace();
            }
            Op::InstanceOf(class) => {
                class.trace();
            }
            Op::MultiANewArray(class, _) => {
                class.trace();
            }
            Op::IfNull(_) => {}
            Op::IfNonNull(_) => {}
        }
    }
}

const A_CONST_NULL: u8 = 0x01;
const I_CONST_M1: u8 = 0x02;
const I_CONST_0: u8 = 0x03;
const I_CONST_1: u8 = 0x04;
const I_CONST_2: u8 = 0x05;
const I_CONST_3: u8 = 0x06;
const I_CONST_4: u8 = 0x07;
const I_CONST_5: u8 = 0x08;
const L_CONST_0: u8 = 0x09;
const L_CONST_1: u8 = 0x0A;
const D_CONST_0: u8 = 0x0E;
const D_CONST_1: u8 = 0x0F;
const B_I_PUSH: u8 = 0x10;
const S_I_PUSH: u8 = 0x11;
const LDC: u8 = 0x12;
const LDC_W: u8 = 0x13;
const LDC_2_W: u8 = 0x14;
const I_LOAD: u8 = 0x15;
const L_LOAD: u8 = 0x16;
const D_LOAD: u8 = 0x18;
const A_LOAD: u8 = 0x19;
const I_LOAD_0: u8 = 0x1A;
const I_LOAD_1: u8 = 0x1B;
const I_LOAD_2: u8 = 0x1C;
const I_LOAD_3: u8 = 0x1D;
const L_LOAD_0: u8 = 0x1E;
const L_LOAD_1: u8 = 0x1F;
const D_LOAD_0: u8 = 0x26;
const D_LOAD_1: u8 = 0x27;
const D_LOAD_2: u8 = 0x28;
const D_LOAD_3: u8 = 0x29;
const A_LOAD_0: u8 = 0x2A;
const A_LOAD_1: u8 = 0x2B;
const A_LOAD_2: u8 = 0x2C;
const A_LOAD_3: u8 = 0x2D;
const IA_LOAD: u8 = 0x2E;
const LA_LOAD: u8 = 0x2F;
const AA_LOAD: u8 = 0x32;
const BA_LOAD: u8 = 0x33;
const CA_LOAD: u8 = 0x34;
const I_STORE: u8 = 0x36;
const L_STORE: u8 = 0x37;
const D_STORE: u8 = 0x39;
const A_STORE: u8 = 0x3A;
const I_STORE_0: u8 = 0x3B;
const I_STORE_1: u8 = 0x3C;
const I_STORE_2: u8 = 0x3D;
const I_STORE_3: u8 = 0x3E;
const L_STORE_0: u8 = 0x3F;
const D_STORE_0: u8 = 0x47;
const D_STORE_1: u8 = 0x48;
const D_STORE_2: u8 = 0x49;
const D_STORE_3: u8 = 0x4A;
const A_STORE_0: u8 = 0x4B;
const A_STORE_1: u8 = 0x4C;
const A_STORE_2: u8 = 0x4D;
const A_STORE_3: u8 = 0x4E;
const IA_STORE: u8 = 0x4F;
const LA_STORE: u8 = 0x50;
const AA_STORE: u8 = 0x53;
const BA_STORE: u8 = 0x54;
const CA_STORE: u8 = 0x55;
const POP: u8 = 0x57;
const DUP: u8 = 0x59;
const DUP_X1: u8 = 0x5A;
const DUP_2: u8 = 0x5C;
const I_ADD: u8 = 0x60;
const L_ADD: u8 = 0x61;
const D_ADD: u8 = 0x63;
const I_SUB: u8 = 0x64;
const L_SUB: u8 = 0x65;
const D_SUB: u8 = 0x67;
const I_MUL: u8 = 0x68;
const D_MUL: u8 = 0x6B;
const I_DIV: u8 = 0x6C;
const D_DIV: u8 = 0x6F;
const I_REM: u8 = 0x70;
const I_NEG: u8 = 0x74;
const I_SHL: u8 = 0x78;
const L_SHL: u8 = 0x79;
const I_SHR: u8 = 0x7A;
const L_SHR: u8 = 0x7B;
const L_USHR: u8 = 0x7D;
const I_AND: u8 = 0x7E;
const L_AND: u8 = 0x7F;
const I_OR: u8 = 0x80;
const L_OR: u8 = 0x81;
const L_XOR: u8 = 0x83;
const I_INC: u8 = 0x84;
const I2L: u8 = 0x85;
const I2D: u8 = 0x87;
const L2I: u8 = 0x88;
const D2I: u8 = 0x8E;
const I2B: u8 = 0x91;
const I2C: u8 = 0x92;
const I2S: u8 = 0x93;
const L_CMP: u8 = 0x94;
const IF_EQ: u8 = 0x99;
const IF_NE: u8 = 0x9A;
const IF_LT: u8 = 0x9B;
const IF_GE: u8 = 0x9C;
const IF_GT: u8 = 0x9D;
const IF_LE: u8 = 0x9E;
const IF_I_CMP_EQ: u8 = 0x9F;
const IF_I_CMP_NE: u8 = 0xA0;
const IF_I_CMP_LT: u8 = 0xA1;
const IF_I_CMP_GE: u8 = 0xA2;
const IF_I_CMP_GT: u8 = 0xA3;
const IF_I_CMP_LE: u8 = 0xA4;
const IF_A_CMP_EQ: u8 = 0xA5;
const IF_A_CMP_NE: u8 = 0xA6;
const GOTO: u8 = 0xA7;
const TABLE_SWITCH: u8 = 0xAA;
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
const INVOKE_INTERFACE: u8 = 0xB9;
const NEW: u8 = 0xBB;
const NEW_ARRAY: u8 = 0xBC;
const A_NEW_ARRAY: u8 = 0xBD;
const ARRAY_LENGTH: u8 = 0xBE;
const A_THROW: u8 = 0xBF;
const CHECK_CAST: u8 = 0xC0;
const INSTANCE_OF: u8 = 0xC1;
const MULTI_A_NEW_ARRAY: u8 = 0xC5;
const IF_NULL: u8 = 0xC6;
const IF_NON_NULL: u8 = 0xC7;

impl Op {
    pub fn read_ops(
        context: Context,
        current_class: Class,
        method_return_type: Descriptor,
        constant_pool: &ConstantPool,
        data: &mut FileData,
    ) -> Result<(Vec<Op>, HashMap<usize, usize>, Vec<Class>), Error> {
        // TODO: Should current_class be None if this is a static method?

        let code_length = data.read_u32()? as usize;
        let code_start = data.position();
        let mut code = Vec::with_capacity(code_length / 2);

        let mut op_index = 0;
        let mut offset_to_idx_map = HashMap::new();

        let mut class_dependencies = Vec::new();

        while data.position() < code_start + code_length {
            offset_to_idx_map.insert(data.position() - code_start, op_index);

            code.push(Op::read_op(
                context,
                current_class,
                method_return_type,
                constant_pool,
                data,
                data.position() - code_start,
                &mut class_dependencies,
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
                | Op::IfGt(position)
                | Op::IfLe(position)
                | Op::IfICmpEq(position)
                | Op::IfICmpNe(position)
                | Op::IfICmpLt(position)
                | Op::IfICmpGe(position)
                | Op::IfICmpGt(position)
                | Op::IfICmpLe(position)
                | Op::IfACmpEq(position)
                | Op::IfACmpNe(position)
                | Op::Goto(position)
                | Op::IfNull(position)
                | Op::IfNonNull(position) => {
                    *position = *offset_to_idx_map
                        .get(position)
                        .ok_or(Error::Native(NativeError::InvalidBranchPosition))?;
                }
                Op::TableSwitch(_, _, ref mut matches, default_offset) => {
                    *default_offset = *offset_to_idx_map
                        .get(default_offset)
                        .ok_or(Error::Native(NativeError::InvalidBranchPosition))?;

                    for offset in matches.iter_mut() {
                        *offset = *offset_to_idx_map
                            .get(offset)
                            .ok_or(Error::Native(NativeError::InvalidBranchPosition))?;
                    }
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

        Ok((code, offset_to_idx_map, class_dependencies))
    }

    fn read_op(
        context: Context,
        current_class: Class,
        method_return_type: Descriptor,
        constant_pool: &ConstantPool,
        data: &mut FileData,
        data_position: usize,
        class_dependencies: &mut Vec<Class>,
    ) -> Result<Op, Error> {
        let opcode = data.read_u8()?;
        match opcode {
            A_CONST_NULL => Ok(Op::AConstNull),
            I_CONST_M1 => Ok(Op::IConst(-1)),
            I_CONST_0 => Ok(Op::IConst(0)),
            I_CONST_1 => Ok(Op::IConst(1)),
            I_CONST_2 => Ok(Op::IConst(2)),
            I_CONST_3 => Ok(Op::IConst(3)),
            I_CONST_4 => Ok(Op::IConst(4)),
            I_CONST_5 => Ok(Op::IConst(5)),
            L_CONST_0 => Ok(Op::LConst(0)),
            L_CONST_1 => Ok(Op::LConst(1)),
            D_CONST_0 => Ok(Op::DConst(0.0)),
            D_CONST_1 => Ok(Op::DConst(1.0)),
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
            LDC_W => {
                let constant_pool_idx = data.read_u16()?;
                let entry = constant_pool.entry(constant_pool_idx)?;

                Ok(Op::Ldc(entry))
            }
            LDC_2_W => {
                let constant_pool_idx = data.read_u16()?;
                let entry = constant_pool.entry(constant_pool_idx)?;

                Ok(Op::Ldc2(entry))
            }
            I_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::ILoad(local_idx as usize))
            }
            L_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::LLoad(local_idx as usize))
            }
            D_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::DLoad(local_idx as usize))
            }
            A_LOAD => {
                let local_idx = data.read_u8()?;

                Ok(Op::ALoad(local_idx as usize))
            }
            I_LOAD_0 => Ok(Op::ILoad(0)),
            I_LOAD_1 => Ok(Op::ILoad(1)),
            I_LOAD_2 => Ok(Op::ILoad(2)),
            I_LOAD_3 => Ok(Op::ILoad(3)),
            L_LOAD_0 => Ok(Op::LLoad(0)),
            L_LOAD_1 => Ok(Op::LLoad(1)),
            D_LOAD_0 => Ok(Op::DLoad(0)),
            D_LOAD_1 => Ok(Op::DLoad(1)),
            D_LOAD_2 => Ok(Op::DLoad(2)),
            D_LOAD_3 => Ok(Op::DLoad(3)),
            A_LOAD_0 => Ok(Op::ALoad(0)),
            A_LOAD_1 => Ok(Op::ALoad(1)),
            A_LOAD_2 => Ok(Op::ALoad(2)),
            A_LOAD_3 => Ok(Op::ALoad(3)),
            IA_LOAD => Ok(Op::IaLoad),
            LA_LOAD => Ok(Op::LaLoad),
            AA_LOAD => Ok(Op::AaLoad),
            BA_LOAD => Ok(Op::BaLoad),
            CA_LOAD => Ok(Op::CaLoad),
            I_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::IStore(local_idx as usize))
            }
            L_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::LStore(local_idx as usize))
            }
            D_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::DStore(local_idx as usize))
            }
            A_STORE => {
                let local_idx = data.read_u8()?;

                Ok(Op::AStore(local_idx as usize))
            }
            I_STORE_0 => Ok(Op::IStore(0)),
            I_STORE_1 => Ok(Op::IStore(1)),
            I_STORE_2 => Ok(Op::IStore(2)),
            I_STORE_3 => Ok(Op::IStore(3)),
            L_STORE_0 => Ok(Op::LStore(0)),
            D_STORE_0 => Ok(Op::DStore(0)),
            D_STORE_1 => Ok(Op::DStore(1)),
            D_STORE_2 => Ok(Op::DStore(2)),
            D_STORE_3 => Ok(Op::DStore(3)),
            A_STORE_0 => Ok(Op::AStore(0)),
            A_STORE_1 => Ok(Op::AStore(1)),
            A_STORE_2 => Ok(Op::AStore(2)),
            A_STORE_3 => Ok(Op::AStore(3)),
            IA_STORE => Ok(Op::IaStore),
            LA_STORE => Ok(Op::LaStore),
            AA_STORE => Ok(Op::AaStore),
            BA_STORE => Ok(Op::BaStore),
            CA_STORE => Ok(Op::CaStore),
            POP => Ok(Op::Pop),
            DUP => Ok(Op::Dup),
            DUP_X1 => Ok(Op::DupX1),
            DUP_2 => Ok(Op::Dup2),
            I_ADD => Ok(Op::IAdd),
            L_ADD => Ok(Op::LAdd),
            D_ADD => Ok(Op::DAdd),
            I_SUB => Ok(Op::ISub),
            L_SUB => Ok(Op::LSub),
            D_SUB => Ok(Op::DSub),
            I_MUL => Ok(Op::IMul),
            D_MUL => Ok(Op::DMul),
            I_DIV => Ok(Op::IDiv),
            D_DIV => Ok(Op::DDiv),
            I_REM => Ok(Op::IRem),
            I_NEG => Ok(Op::INeg),
            I_SHL => Ok(Op::IShl),
            L_SHL => Ok(Op::LShl),
            I_SHR => Ok(Op::IShr),
            L_SHR => Ok(Op::LShr),
            L_USHR => Ok(Op::LUshr),
            I_AND => Ok(Op::IAnd),
            L_AND => Ok(Op::LAnd),
            I_OR => Ok(Op::IOr),
            L_OR => Ok(Op::LOr),
            L_XOR => Ok(Op::LXor),
            I_INC => {
                let local_idx = data.read_u8()?;
                let constant = data.read_u8()? as i8;

                Ok(Op::IInc(local_idx as usize, constant as i32))
            }
            I2L => Ok(Op::I2L),
            I2D => Ok(Op::I2D),
            L2I => Ok(Op::L2I),
            D2I => Ok(Op::D2I),
            I2B => Ok(Op::I2B),
            I2C => Ok(Op::I2C),
            I2S => Ok(Op::I2S),
            L_CMP => Ok(Op::LCmp),
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
            IF_GT => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfGt(((data_position as isize) + offset) as usize))
            }
            IF_LE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfLe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_EQ => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpEq(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_NE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpNe(((data_position as isize) + offset) as usize))
            }
            IF_I_CMP_LT => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfICmpLt(((data_position as isize) + offset) as usize))
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
            IF_A_CMP_EQ => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfACmpEq(((data_position as isize) + offset) as usize))
            }
            IF_A_CMP_NE => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfACmpNe(((data_position as isize) + offset) as usize))
            }
            GOTO => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::Goto(((data_position as isize) + offset) as usize))
            }
            TABLE_SWITCH => {
                let padding_bytes = (data_position + 1) % 4;
                if padding_bytes != 0 {
                    for _ in 0..(4 - padding_bytes) {
                        data.read_u8()?;
                    }
                }

                let default_offset = data.read_u32()? as i32 as isize;
                let default_offset = ((data_position as isize) + default_offset) as usize;

                let low_int = data.read_u32()? as i32;
                let high_int = data.read_u32()? as i32;

                let num_offsets = (high_int - low_int) as usize + 1;
                let mut offsets = Vec::with_capacity(num_offsets);
                for _ in 0..num_offsets {
                    let offset = data.read_u32()? as i32 as isize;
                    let offset = ((data_position as isize) + offset) as usize;

                    offsets.push(offset);
                }

                Ok(Op::TableSwitch(
                    low_int,
                    high_int,
                    offsets.into_boxed_slice(),
                    default_offset,
                ))
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
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

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
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

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
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

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
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

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
                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let method_index = class
                    .instance_method_vtable()
                    .lookup((method_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                Ok(Op::InvokeVirtual(class, method_index))
            }
            INVOKE_SPECIAL => {
                let method_ref_idx = data.read_u16()?;
                let method_ref = constant_pool.get_method_ref(method_ref_idx)?;

                let (class_name, method_name, descriptor_name) = method_ref;

                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                // TODO implement rules around when `class` is an interface
                let real_class = if method_name.as_bytes() != b"<init>"
                    && !class.is_interface()
                    && current_class.has_super_class(class)
                {
                    current_class.super_class().unwrap()
                } else {
                    class
                };

                let method_vtable = real_class.instance_method_vtable();

                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let method_slot = method_vtable
                    .lookup((method_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                let method = method_vtable.get_element(method_slot);

                Ok(Op::InvokeSpecial(class, method))
            }
            INVOKE_STATIC => {
                let method_ref_idx = data.read_u16()?;
                let method_ref = constant_pool.get_method_ref(method_ref_idx)?;

                let (class_name, method_name, descriptor_name) = method_ref;

                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                let method_slot = class
                    .static_method_vtable()
                    .lookup((method_name, descriptor))
                    .ok_or(Error::Native(NativeError::VTableLookupFailed))?;

                let method = class.static_methods()[method_slot];

                Ok(Op::InvokeStatic(method))
            }
            INVOKE_INTERFACE => {
                let method_ref_idx = data.read_u16()?;
                let method_ref = constant_pool.get_interface_method_ref(method_ref_idx)?;

                let (class_name, method_name, descriptor_name) = method_ref;

                // Method is called based on class of object on stack
                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                let descriptor = MethodDescriptor::from_string(context.gc_ctx, descriptor_name)
                    .ok_or(Error::Native(NativeError::InvalidDescriptor))?;

                // According to the JVMS, this byte states the argument count
                // of the method (despite that also being defined in the
                // descriptor) for "historical" reasons.
                let _arg_count = data.read_u8()?;

                // This should always be zero.
                let _ = data.read_u8()?;

                Ok(Op::InvokeInterface(class, (method_name, descriptor)))
            }
            NEW => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

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
            A_NEW_ARRAY => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                Ok(Op::ANewArray(class))
            }
            ARRAY_LENGTH => Ok(Op::ArrayLength),
            A_THROW => Ok(Op::AThrow),
            CHECK_CAST => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                Ok(Op::CheckCast(class))
            }
            INSTANCE_OF => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let class = context.lookup_class(class_name)?;
                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                Ok(Op::InstanceOf(class))
            }
            MULTI_A_NEW_ARRAY => {
                let class_idx = data.read_u16()?;
                let class_name = constant_pool.get_class(class_idx)?;

                let mut class = context.lookup_class(class_name)?;

                // Get the innermost class of this class if we were given an array class.
                while let Some(resolved_descriptor) = class.array_value_type() {
                    if let Some(inner_class) = resolved_descriptor.class() {
                        class = inner_class;
                    } else {
                        return Err(Error::Native(NativeError::VerifyTypeWrong));
                    }
                }

                if !class_dependencies.contains(&class) {
                    class_dependencies.push(class);
                }

                let dim_count = data.read_u8()?;

                if dim_count == 0 {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }

                Ok(Op::MultiANewArray(class, dim_count))
            }
            IF_NULL => {
                let offset = data.read_u16()? as i16 as isize;

                Ok(Op::IfNull(((data_position as isize) + offset) as usize))
            }
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
                | Op::LaLoad
                | Op::AaLoad
                | Op::BaLoad
                | Op::CaLoad
                | Op::IaStore
                | Op::LaStore
                | Op::AaStore
                | Op::BaStore
                | Op::CaStore
                | Op::IDiv
                | Op::IRem
                | Op::GetField(_, _)
                | Op::PutField(_, _)
                | Op::InvokeVirtual(_, _)
                | Op::InvokeSpecial(_, _)
                | Op::InvokeStatic(_)
                | Op::InvokeInterface(_, _)
                | Op::NewArray(_)
                | Op::ANewArray(_)
                | Op::ArrayLength
                | Op::AThrow
                | Op::CheckCast(_)
        )
    }
}
