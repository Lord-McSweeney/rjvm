use super::class::Class;
use super::context::Context;
use super::descriptor::{Descriptor, MethodDescriptor, ResolvedDescriptor};
use super::error::Error;
use super::method::Method;
use super::read_macros::{read_u8, read_u16_be, read_u32_be};

use crate::classfile::constant_pool::{ConstantPool, ConstantPoolEntry};
use crate::classfile::flags::FieldFlags;
use crate::gc::{Gc, Trace};
use crate::reader::{FileData, Reader};
use crate::string::JvmString;

use alloc::boxed::Box;
use alloc::vec::Vec;
use hashbrown::HashMap;

#[derive(Clone, Debug)]
pub enum Op {
    Nop,

    // Push constant
    AConstNull,
    IConst(i32),
    LConst(i8),
    FConst(f32),
    DConst(f64),
    Ldc(Gc<ConstantPoolEntry>),
    LoadLong(i64),
    LoadDouble(f64),

    // Load local
    ILoad(usize),
    LLoad(usize),
    FLoad(usize),
    DLoad(usize),
    ALoad(usize),

    // Array get
    IaLoad,
    LaLoad,
    FaLoad,
    DaLoad,
    AaLoad,
    BaLoad,
    CaLoad,
    SaLoad,

    // Store local
    IStore(usize),
    LStore(usize),
    FStore(usize),
    DStore(usize),
    AStore(usize),

    // Array set
    IaStore,
    LaStore,
    FaStore,
    DaStore,
    AaStore,
    BaStore,
    CaStore,
    SaStore,

    // Stack
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X2,
    Swap,

    // Arithmetic
    IAdd,
    LAdd,
    FAdd,
    DAdd,
    ISub,
    LSub,
    FSub,
    DSub,
    IMul,
    LMul,
    FMul,
    DMul,
    IDiv,
    LDiv,
    FDiv,
    DDiv,
    IRem,
    LRem,
    FRem,
    DRem,
    INeg,
    LNeg,
    FNeg,
    DNeg,
    IShl,
    LShl,
    IShr,
    LShr,
    IUshr,
    LUshr,
    IAnd,
    LAnd,
    IOr,
    LOr,
    IXor,
    LXor,

    IInc(usize, i32),

    // Conversion
    I2L,
    I2F,
    I2D,
    L2I,
    L2F,
    L2D,
    F2I,
    F2L,
    F2D,
    D2I,
    D2L,
    D2F,
    I2B,
    I2C,
    I2S,

    // Comparison
    LCmp,
    FCmpL,
    FCmpG,
    DCmpL,
    DCmpG,

    // Branching
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
    Jsr(usize),
    Ret(usize),
    TableSwitch(Box<TableSwitchInfo>),
    LookupSwitch(Box<LookupSwitchInfo>),

    // Return
    IReturn,
    LReturn,
    FReturn,
    DReturn,
    AReturn,
    Return,

    // Field get/set
    GetStatic(Class, u32),
    PutStatic(Class, u32),
    GetField(Class, u32),
    PutField(Class, u32),

    // Wide field get/set
    // It's slightly faster to separate these than to check wideness at runtime
    GetStaticWide(Class, u32),
    PutStaticWide(Class, u32),
    GetFieldWide(Class, u32),
    PutFieldWide(Class, u32),

    // Method invocation
    InvokeVirtual(Class, u32, u8),
    InvokeVirtualWide(Class, u32, u8),
    InvokeSpecial(Method),
    InvokeStatic(Method),
    InvokeInterface(Box<InvokeInterfaceInfo>),

    // Memory allocation
    New(Class),
    NewArray(ArrayType),
    ANewArray(Class),

    // Misc
    ArrayLength,
    AThrow,
    CheckCast(Class),
    InstanceOf(Class),
    MonitorEnter,
    MonitorExit,
    MultiANewArray(Box<MultiANewArrayInfo>),
    IfNull(usize),
    IfNonNull(usize),

    // Custom ops
    Clinit(Class),
    GcCheck,
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
            Op::Nop => {}
            Op::AConstNull => {}
            Op::IConst(_) => {}
            Op::LConst(_) => {}
            Op::FConst(_) => {}
            Op::DConst(_) => {}
            Op::Ldc(entry) => {
                entry.trace();
            }
            Op::LoadLong(_) => {}
            Op::LoadDouble(_) => {}
            Op::ILoad(_) => {}
            Op::LLoad(_) => {}
            Op::FLoad(_) => {}
            Op::DLoad(_) => {}
            Op::ALoad(_) => {}
            Op::IaLoad => {}
            Op::LaLoad => {}
            Op::FaLoad => {}
            Op::DaLoad => {}
            Op::AaLoad => {}
            Op::BaLoad => {}
            Op::CaLoad => {}
            Op::SaLoad => {}
            Op::IStore(_) => {}
            Op::LStore(_) => {}
            Op::FStore(_) => {}
            Op::DStore(_) => {}
            Op::AStore(_) => {}
            Op::IaStore => {}
            Op::LaStore => {}
            Op::FaStore => {}
            Op::DaStore => {}
            Op::AaStore => {}
            Op::BaStore => {}
            Op::CaStore => {}
            Op::SaStore => {}
            Op::Pop => {}
            Op::Pop2 => {}
            Op::Dup => {}
            Op::DupX1 => {}
            Op::DupX2 => {}
            Op::Dup2 => {}
            Op::Dup2X2 => {}
            Op::Swap => {}
            Op::IAdd => {}
            Op::LAdd => {}
            Op::FAdd => {}
            Op::DAdd => {}
            Op::ISub => {}
            Op::LSub => {}
            Op::FSub => {}
            Op::DSub => {}
            Op::IMul => {}
            Op::LMul => {}
            Op::FMul => {}
            Op::DMul => {}
            Op::IDiv => {}
            Op::LDiv => {}
            Op::FDiv => {}
            Op::DDiv => {}
            Op::IRem => {}
            Op::LRem => {}
            Op::FRem => {}
            Op::DRem => {}
            Op::INeg => {}
            Op::LNeg => {}
            Op::FNeg => {}
            Op::DNeg => {}
            Op::IShl => {}
            Op::LShl => {}
            Op::IShr => {}
            Op::LShr => {}
            Op::IUshr => {}
            Op::LUshr => {}
            Op::IAnd => {}
            Op::LAnd => {}
            Op::IOr => {}
            Op::LOr => {}
            Op::IXor => {}
            Op::LXor => {}
            Op::IInc(_, _) => {}
            Op::I2L => {}
            Op::I2F => {}
            Op::I2D => {}
            Op::L2I => {}
            Op::L2F => {}
            Op::L2D => {}
            Op::F2I => {}
            Op::F2L => {}
            Op::F2D => {}
            Op::D2I => {}
            Op::D2L => {}
            Op::D2F => {}
            Op::I2B => {}
            Op::I2C => {}
            Op::I2S => {}
            Op::LCmp => {}
            Op::FCmpL => {}
            Op::FCmpG => {}
            Op::DCmpL => {}
            Op::DCmpG => {}
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
            Op::Jsr(_) => {}
            Op::Ret(_) => {}
            Op::TableSwitch(_) => {}
            Op::LookupSwitch(_) => {}
            Op::IReturn => {}
            Op::LReturn => {}
            Op::FReturn => {}
            Op::DReturn => {}
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
            Op::GetStaticWide(class, _) => {
                class.trace();
            }
            Op::PutStaticWide(class, _) => {
                class.trace();
            }
            Op::GetFieldWide(class, _) => {
                class.trace();
            }
            Op::PutFieldWide(class, _) => {
                class.trace();
            }
            Op::InvokeVirtual(class, _, _) => {
                class.trace();
            }
            Op::InvokeVirtualWide(class, _, _) => {
                class.trace();
            }
            Op::InvokeSpecial(method) => {
                method.trace();
            }
            Op::InvokeStatic(method) => {
                method.trace();
            }
            Op::InvokeInterface(invoke_interface) => {
                invoke_interface.class.trace();
                invoke_interface.name.trace();
                invoke_interface.descriptor.trace();
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
            Op::MonitorEnter => {}
            Op::MonitorExit => {}
            Op::MultiANewArray(multi_a_new_array) => {
                multi_a_new_array.class.trace();
            }
            Op::IfNull(_) => {}
            Op::IfNonNull(_) => {}

            Op::Clinit(class) => {
                class.trace();
            }
            Op::GcCheck => {}
        }
    }
}

const NOP: u8 = 0x00;
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
const F_CONST_0: u8 = 0x0B;
const F_CONST_1: u8 = 0x0C;
const F_CONST_2: u8 = 0x0D;
const D_CONST_0: u8 = 0x0E;
const D_CONST_1: u8 = 0x0F;
const B_I_PUSH: u8 = 0x10;
const S_I_PUSH: u8 = 0x11;
const LDC: u8 = 0x12;
const LDC_W: u8 = 0x13;
const LDC_2_W: u8 = 0x14;
const I_LOAD: u8 = 0x15;
const L_LOAD: u8 = 0x16;
const F_LOAD: u8 = 0x17;
const D_LOAD: u8 = 0x18;
const A_LOAD: u8 = 0x19;
const I_LOAD_0: u8 = 0x1A;
const I_LOAD_1: u8 = 0x1B;
const I_LOAD_2: u8 = 0x1C;
const I_LOAD_3: u8 = 0x1D;
const L_LOAD_0: u8 = 0x1E;
const L_LOAD_1: u8 = 0x1F;
const L_LOAD_2: u8 = 0x20;
const L_LOAD_3: u8 = 0x21;
const F_LOAD_0: u8 = 0x22;
const F_LOAD_1: u8 = 0x23;
const F_LOAD_2: u8 = 0x24;
const F_LOAD_3: u8 = 0x25;
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
const FA_LOAD: u8 = 0x30;
const DA_LOAD: u8 = 0x31;
const AA_LOAD: u8 = 0x32;
const BA_LOAD: u8 = 0x33;
const CA_LOAD: u8 = 0x34;
const SA_LOAD: u8 = 0x35;
const I_STORE: u8 = 0x36;
const L_STORE: u8 = 0x37;
const F_STORE: u8 = 0x38;
const D_STORE: u8 = 0x39;
const A_STORE: u8 = 0x3A;
const I_STORE_0: u8 = 0x3B;
const I_STORE_1: u8 = 0x3C;
const I_STORE_2: u8 = 0x3D;
const I_STORE_3: u8 = 0x3E;
const L_STORE_0: u8 = 0x3F;
const L_STORE_1: u8 = 0x40;
const L_STORE_2: u8 = 0x41;
const L_STORE_3: u8 = 0x42;
const F_STORE_0: u8 = 0x43;
const F_STORE_1: u8 = 0x44;
const F_STORE_2: u8 = 0x45;
const F_STORE_3: u8 = 0x46;
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
const FA_STORE: u8 = 0x51;
const DA_STORE: u8 = 0x52;
const AA_STORE: u8 = 0x53;
const BA_STORE: u8 = 0x54;
const CA_STORE: u8 = 0x55;
const SA_STORE: u8 = 0x56;
const POP: u8 = 0x57;
const POP_2: u8 = 0x58;
const DUP: u8 = 0x59;
const DUP_X1: u8 = 0x5A;
const DUP_X2: u8 = 0x5B;
const DUP_2: u8 = 0x5C;
const DUP_2_X2: u8 = 0x5E;
const SWAP: u8 = 0x5F;
const I_ADD: u8 = 0x60;
const L_ADD: u8 = 0x61;
const F_ADD: u8 = 0x62;
const D_ADD: u8 = 0x63;
const I_SUB: u8 = 0x64;
const L_SUB: u8 = 0x65;
const F_SUB: u8 = 0x66;
const D_SUB: u8 = 0x67;
const I_MUL: u8 = 0x68;
const L_MUL: u8 = 0x69;
const F_MUL: u8 = 0x6A;
const D_MUL: u8 = 0x6B;
const I_DIV: u8 = 0x6C;
const L_DIV: u8 = 0x6D;
const F_DIV: u8 = 0x6E;
const D_DIV: u8 = 0x6F;
const I_REM: u8 = 0x70;
const L_REM: u8 = 0x71;
const F_REM: u8 = 0x72;
const D_REM: u8 = 0x73;
const I_NEG: u8 = 0x74;
const L_NEG: u8 = 0x75;
const F_NEG: u8 = 0x76;
const D_NEG: u8 = 0x77;
const I_SHL: u8 = 0x78;
const L_SHL: u8 = 0x79;
const I_SHR: u8 = 0x7A;
const L_SHR: u8 = 0x7B;
const I_USHR: u8 = 0x7C;
const L_USHR: u8 = 0x7D;
const I_AND: u8 = 0x7E;
const L_AND: u8 = 0x7F;
const I_OR: u8 = 0x80;
const L_OR: u8 = 0x81;
const I_XOR: u8 = 0x82;
const L_XOR: u8 = 0x83;
const I_INC: u8 = 0x84;
const I2L: u8 = 0x85;
const I2F: u8 = 0x86;
const I2D: u8 = 0x87;
const L2I: u8 = 0x88;
const L2F: u8 = 0x89;
const L2D: u8 = 0x8A;
const F2I: u8 = 0x8B;
const F2L: u8 = 0x8C;
const F2D: u8 = 0x8D;
const D2I: u8 = 0x8E;
const D2L: u8 = 0x8F;
const D2F: u8 = 0x90;
const I2B: u8 = 0x91;
const I2C: u8 = 0x92;
const I2S: u8 = 0x93;
const L_CMP: u8 = 0x94;
const F_CMP_L: u8 = 0x95;
const F_CMP_G: u8 = 0x96;
const D_CMP_L: u8 = 0x97;
const D_CMP_G: u8 = 0x98;
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
const JSR: u8 = 0xA8;
const RET: u8 = 0xA9;
const TABLE_SWITCH: u8 = 0xAA;
const LOOKUP_SWITCH: u8 = 0xAB;
const I_RETURN: u8 = 0xAC;
const L_RETURN: u8 = 0xAD;
const F_RETURN: u8 = 0xAE;
const D_RETURN: u8 = 0xAF;
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
const MONITOR_ENTER: u8 = 0xC2;
const MONITOR_EXIT: u8 = 0xC3;
const MULTI_A_NEW_ARRAY: u8 = 0xC5;
const IF_NULL: u8 = 0xC6;
const IF_NON_NULL: u8 = 0xC7;
const GOTO_W: u8 = 0xC8;
const JSR_W: u8 = 0xC9;

impl Op {
    pub fn read_ops(
        context: &Context,
        method: Method,
        constant_pool: &ConstantPool,
        data: &mut FileData<'_>,
    ) -> Result<(Box<[Op]>, HashMap<usize, usize>), Error> {
        let code_length = read_u32_be!(context, data) as usize;
        let code_start = data.position();
        let mut code = Vec::with_capacity(code_length);

        let mut offset_to_idx_map = HashMap::with_capacity(code_length);
        offset_to_idx_map.insert(0, 0);

        // TODO: We should perform a Gc check in more places, probably ideally
        // at all allocation points
        code.push(Op::GcCheck);

        while data.position() < code_start + code_length {
            let ops = Op::read_op(
                context,
                method,
                constant_pool,
                data,
                data.position() - code_start,
            )?;

            // It is sometimes useful to split a single JVM op into multiple
            // sub-ops (microops?). So we support emitting two ops instead of
            // one for each Java op.
            code.push(ops.0);
            if let Some(second_op) = ops.1 {
                code.push(second_op);
            }

            offset_to_idx_map.insert(data.position() - code_start, code.len());
        }

        let mut code = code.into_boxed_slice();

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
                | Op::Jsr(position)
                | Op::IfNull(position)
                | Op::IfNonNull(position) => {
                    *position = *offset_to_idx_map
                        .get(position)
                        .ok_or_else(|| context.verify_error("Invalid branch target"))?;
                }
                Op::TableSwitch(table_switch) => {
                    table_switch.default_offset = *offset_to_idx_map
                        .get(&table_switch.default_offset)
                        .ok_or_else(|| {
                            context.verify_error("Invalid tableswitch default target")
                        })?;

                    for offset in table_switch.matches.iter_mut() {
                        *offset = *offset_to_idx_map.get(offset).ok_or_else(|| {
                            context.verify_error("Invalid tableswitch case target")
                        })?;
                    }
                }
                Op::LookupSwitch(lookup_switch) => {
                    lookup_switch.default_offset = *offset_to_idx_map
                        .get(&lookup_switch.default_offset)
                        .ok_or_else(|| {
                            context.verify_error("Invalid lookupswitch branch target")
                        })?;

                    for (_, offset) in lookup_switch.matches.iter_mut() {
                        *offset = *offset_to_idx_map.get(offset).ok_or_else(|| {
                            context.verify_error("Invalid lookupswitch case target")
                        })?;
                    }
                }
                _ => {}
            }
        }

        Ok((code, offset_to_idx_map))
    }

    fn read_op(
        context: &Context,
        method: Method,
        constant_pool: &ConstantPool,
        data: &mut FileData<'_>,
        data_position: usize,
    ) -> Result<(Op, Option<Op>), Error> {
        let loader = method.class_loader();

        let opcode = read_u8!(context, data);
        let result = match opcode {
            NOP => Op::Nop,
            A_CONST_NULL => Op::AConstNull,
            I_CONST_M1 => Op::IConst(-1),
            I_CONST_0 => Op::IConst(0),
            I_CONST_1 => Op::IConst(1),
            I_CONST_2 => Op::IConst(2),
            I_CONST_3 => Op::IConst(3),
            I_CONST_4 => Op::IConst(4),
            I_CONST_5 => Op::IConst(5),
            L_CONST_0 => Op::LConst(0),
            L_CONST_1 => Op::LConst(1),
            F_CONST_0 => Op::FConst(0.0),
            F_CONST_1 => Op::FConst(1.0),
            F_CONST_2 => Op::FConst(2.0),
            D_CONST_0 => Op::DConst(0.0),
            D_CONST_1 => Op::DConst(1.0),
            B_I_PUSH => {
                let byte = read_u8!(context, data) as i8 as i32;

                Op::IConst(byte)
            }
            S_I_PUSH => {
                let byte = read_u16_be!(context, data) as i16 as i32;

                Op::IConst(byte)
            }
            LDC => {
                let constant_pool_idx = read_u8!(context, data);
                let entry = constant_pool
                    .entry(constant_pool_idx as u16)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                Op::Ldc(Gc::new(context.gc_ctx, entry))
            }
            LDC_W => {
                let constant_pool_idx = read_u16_be!(context, data);
                let entry = constant_pool
                    .entry(constant_pool_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                Op::Ldc(Gc::new(context.gc_ctx, entry))
            }
            LDC_2_W => {
                let constant_pool_idx = read_u16_be!(context, data);
                let entry = constant_pool
                    .entry(constant_pool_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                match entry {
                    ConstantPoolEntry::Long { value } => Op::LoadLong(value),
                    ConstantPoolEntry::Double { value } => Op::LoadDouble(value),
                    // TODO error handling
                    _ => panic!("Ldc2 only works on Double and Long"),
                }
            }
            I_LOAD => {
                let local_idx = read_u8!(context, data);

                Op::ILoad(local_idx as usize)
            }
            L_LOAD => {
                let local_idx = read_u8!(context, data);

                Op::LLoad(local_idx as usize)
            }
            F_LOAD => {
                let local_idx = read_u8!(context, data);

                Op::FLoad(local_idx as usize)
            }
            D_LOAD => {
                let local_idx = read_u8!(context, data);

                Op::DLoad(local_idx as usize)
            }
            A_LOAD => {
                let local_idx = read_u8!(context, data);

                Op::ALoad(local_idx as usize)
            }
            I_LOAD_0 => Op::ILoad(0),
            I_LOAD_1 => Op::ILoad(1),
            I_LOAD_2 => Op::ILoad(2),
            I_LOAD_3 => Op::ILoad(3),
            L_LOAD_0 => Op::LLoad(0),
            L_LOAD_1 => Op::LLoad(1),
            L_LOAD_2 => Op::LLoad(2),
            L_LOAD_3 => Op::LLoad(3),
            F_LOAD_0 => Op::FLoad(0),
            F_LOAD_1 => Op::FLoad(1),
            F_LOAD_2 => Op::FLoad(2),
            F_LOAD_3 => Op::FLoad(3),
            D_LOAD_0 => Op::DLoad(0),
            D_LOAD_1 => Op::DLoad(1),
            D_LOAD_2 => Op::DLoad(2),
            D_LOAD_3 => Op::DLoad(3),
            A_LOAD_0 => Op::ALoad(0),
            A_LOAD_1 => Op::ALoad(1),
            A_LOAD_2 => Op::ALoad(2),
            A_LOAD_3 => Op::ALoad(3),
            IA_LOAD => Op::IaLoad,
            LA_LOAD => Op::LaLoad,
            FA_LOAD => Op::FaLoad,
            DA_LOAD => Op::DaLoad,
            AA_LOAD => Op::AaLoad,
            BA_LOAD => Op::BaLoad,
            CA_LOAD => Op::CaLoad,
            SA_LOAD => Op::SaLoad,
            I_STORE => {
                let local_idx = read_u8!(context, data);

                Op::IStore(local_idx as usize)
            }
            L_STORE => {
                let local_idx = read_u8!(context, data);

                Op::LStore(local_idx as usize)
            }
            F_STORE => {
                let local_idx = read_u8!(context, data);

                Op::FStore(local_idx as usize)
            }
            D_STORE => {
                let local_idx = read_u8!(context, data);

                Op::DStore(local_idx as usize)
            }
            A_STORE => {
                let local_idx = read_u8!(context, data);

                Op::AStore(local_idx as usize)
            }
            I_STORE_0 => Op::IStore(0),
            I_STORE_1 => Op::IStore(1),
            I_STORE_2 => Op::IStore(2),
            I_STORE_3 => Op::IStore(3),
            L_STORE_0 => Op::LStore(0),
            L_STORE_1 => Op::LStore(1),
            L_STORE_2 => Op::LStore(2),
            L_STORE_3 => Op::LStore(3),
            F_STORE_0 => Op::FStore(0),
            F_STORE_1 => Op::FStore(1),
            F_STORE_2 => Op::FStore(2),
            F_STORE_3 => Op::FStore(3),
            D_STORE_0 => Op::DStore(0),
            D_STORE_1 => Op::DStore(1),
            D_STORE_2 => Op::DStore(2),
            D_STORE_3 => Op::DStore(3),
            A_STORE_0 => Op::AStore(0),
            A_STORE_1 => Op::AStore(1),
            A_STORE_2 => Op::AStore(2),
            A_STORE_3 => Op::AStore(3),
            IA_STORE => Op::IaStore,
            LA_STORE => Op::LaStore,
            FA_STORE => Op::FaStore,
            DA_STORE => Op::DaStore,
            AA_STORE => Op::AaStore,
            BA_STORE => Op::BaStore,
            CA_STORE => Op::CaStore,
            SA_STORE => Op::SaStore,
            POP => Op::Pop,
            POP_2 => Op::Pop2,
            DUP => Op::Dup,
            DUP_X1 => Op::DupX1,
            DUP_X2 => Op::DupX2,
            DUP_2 => Op::Dup2,
            DUP_2_X2 => Op::Dup2X2,
            SWAP => Op::Swap,
            I_ADD => Op::IAdd,
            L_ADD => Op::LAdd,
            F_ADD => Op::FAdd,
            D_ADD => Op::DAdd,
            I_SUB => Op::ISub,
            L_SUB => Op::LSub,
            F_SUB => Op::FSub,
            D_SUB => Op::DSub,
            I_MUL => Op::IMul,
            L_MUL => Op::LMul,
            F_MUL => Op::FMul,
            D_MUL => Op::DMul,
            I_DIV => Op::IDiv,
            L_DIV => Op::LDiv,
            F_DIV => Op::FDiv,
            D_DIV => Op::DDiv,
            I_REM => Op::IRem,
            L_REM => Op::LRem,
            F_REM => Op::FRem,
            D_REM => Op::DRem,
            I_NEG => Op::INeg,
            L_NEG => Op::LNeg,
            F_NEG => Op::FNeg,
            D_NEG => Op::DNeg,
            I_SHL => Op::IShl,
            L_SHL => Op::LShl,
            I_SHR => Op::IShr,
            L_SHR => Op::LShr,
            I_USHR => Op::IUshr,
            L_USHR => Op::LUshr,
            I_AND => Op::IAnd,
            L_AND => Op::LAnd,
            I_OR => Op::IOr,
            L_OR => Op::LOr,
            I_XOR => Op::IXor,
            L_XOR => Op::LXor,
            I_INC => {
                let local_idx = read_u8!(context, data);
                let constant = read_u8!(context, data) as i8;

                Op::IInc(local_idx as usize, constant as i32)
            }
            I2L => Op::I2L,
            I2F => Op::I2F,
            I2D => Op::I2D,
            L2I => Op::L2I,
            L2F => Op::L2F,
            L2D => Op::L2D,
            F2I => Op::F2I,
            F2L => Op::F2L,
            F2D => Op::F2D,
            D2I => Op::D2I,
            D2L => Op::D2L,
            D2F => Op::D2F,
            I2B => Op::I2B,
            I2C => Op::I2C,
            I2S => Op::I2S,
            L_CMP => Op::LCmp,
            F_CMP_L => Op::FCmpL,
            F_CMP_G => Op::FCmpG,
            D_CMP_L => Op::DCmpL,
            D_CMP_G => Op::DCmpG,
            IF_EQ => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfEq(((data_position as isize) + offset) as usize)
            }
            IF_NE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfNe(((data_position as isize) + offset) as usize)
            }
            IF_LT => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfLt(((data_position as isize) + offset) as usize)
            }
            IF_GE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfGe(((data_position as isize) + offset) as usize)
            }
            IF_GT => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfGt(((data_position as isize) + offset) as usize)
            }
            IF_LE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfLe(((data_position as isize) + offset) as usize)
            }
            IF_I_CMP_EQ => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfICmpEq(((data_position as isize) + offset) as usize)
            }
            IF_I_CMP_NE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfICmpNe(((data_position as isize) + offset) as usize)
            }
            IF_I_CMP_LT => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfICmpLt(((data_position as isize) + offset) as usize)
            }
            IF_I_CMP_GE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfICmpGe(((data_position as isize) + offset) as usize)
            }
            IF_I_CMP_GT => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfICmpGt(((data_position as isize) + offset) as usize)
            }
            IF_I_CMP_LE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfICmpLe(((data_position as isize) + offset) as usize)
            }
            IF_A_CMP_EQ => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfACmpEq(((data_position as isize) + offset) as usize)
            }
            IF_A_CMP_NE => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfACmpNe(((data_position as isize) + offset) as usize)
            }
            GOTO => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::Goto(((data_position as isize) + offset) as usize)
            }
            JSR => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::Jsr(((data_position as isize) + offset) as usize)
            }
            RET => {
                let local_idx = read_u8!(context, data);

                Op::Ret(local_idx as usize)
            }
            TABLE_SWITCH => {
                let padding_bytes = (data_position + 1) % 4;
                if padding_bytes != 0 {
                    for _ in 0..(4 - padding_bytes) {
                        read_u8!(context, data);
                    }
                }

                let default_offset = read_u32_be!(context, data) as i32 as isize;
                let default_offset = ((data_position as isize) + default_offset) as usize;

                let low_int = read_u32_be!(context, data) as i32;
                let high_int = read_u32_be!(context, data) as i32;

                let num_offsets = (high_int - low_int) as usize + 1;
                let mut offsets = Vec::with_capacity(num_offsets);
                for _ in 0..num_offsets {
                    let offset = read_u32_be!(context, data) as i32 as isize;
                    let offset = ((data_position as isize) + offset) as usize;

                    offsets.push(offset);
                }

                let table_switch = TableSwitchInfo {
                    low_int,
                    matches: offsets.into_boxed_slice(),
                    default_offset,
                };

                Op::TableSwitch(Box::new(table_switch))
            }
            LOOKUP_SWITCH => {
                let padding_bytes = (data_position + 1) % 4;
                if padding_bytes != 0 {
                    for _ in 0..(4 - padding_bytes) {
                        read_u8!(context, data);
                    }
                }

                let default_offset = read_u32_be!(context, data) as i32 as isize;
                let default_offset = ((data_position as isize) + default_offset) as usize;

                let num_pairs = read_u32_be!(context, data);
                let mut pairs = Vec::with_capacity(num_pairs as usize);
                for _ in 0..num_pairs {
                    let matching_value = read_u32_be!(context, data) as i32;

                    let offset = read_u32_be!(context, data) as i32 as isize;
                    let offset = ((data_position as isize) + offset) as usize;

                    pairs.push((matching_value, offset));
                }

                let lookup_switch = LookupSwitchInfo {
                    matches: pairs.into_boxed_slice(),
                    default_offset,
                };

                Op::LookupSwitch(Box::new(lookup_switch))
            }
            I_RETURN => {
                let return_type = method.descriptor().return_type();

                if !matches!(
                    return_type,
                    Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Integer
                        | Descriptor::Short
                ) {
                    return Err(context.verify_error("ireturn: Bad return type"));
                } else {
                    Op::IReturn
                }
            }
            L_RETURN => {
                let return_type = method.descriptor().return_type();

                if !matches!(return_type, Descriptor::Long) {
                    return Err(context.verify_error("lreturn: Bad return type"));
                } else {
                    Op::LReturn
                }
            }
            F_RETURN => {
                let return_type = method.descriptor().return_type();

                if !matches!(return_type, Descriptor::Float) {
                    return Err(context.verify_error("freturn: Bad return type"));
                } else {
                    Op::FReturn
                }
            }
            D_RETURN => {
                let return_type = method.descriptor().return_type();

                if !matches!(return_type, Descriptor::Double) {
                    return Err(context.verify_error("dreturn: Bad return type"));
                } else {
                    Op::DReturn
                }
            }
            A_RETURN => {
                let return_type = method.descriptor().return_type();

                if !matches!(return_type, Descriptor::Class(_) | Descriptor::Array(_)) {
                    return Err(context.verify_error("areturn: Bad return type"));
                } else {
                    Op::AReturn
                }
            }
            RETURN => {
                let return_type = method.descriptor().return_type();

                if !matches!(return_type, Descriptor::Void) {
                    return Err(context.verify_error("return: Bad return type"));
                } else {
                    Op::Return
                }
            }
            GET_STATIC => {
                let field_ref_idx = read_u16_be!(context, data);
                let field_ref = constant_pool
                    .get_field_ref(field_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = loader.lookup_class(context, class_name)?;

                let descriptor = Descriptor::from_string(context, descriptor_name)?;

                let field_slot = class
                    .static_field_vtable()
                    .lookup((field_name, descriptor))
                    .ok_or_else(|| context.no_such_field_error())?;

                let field = class.get_static_field(field_slot);

                // TODO "package-private" and "protected" access control
                let flags = field.flags();
                if flags.contains(FieldFlags::PRIVATE) && method.class() != class {
                    return Err(context.illegal_access_error());
                }

                let defining_class = field.defining_class();

                let field_op = if descriptor.is_wide() {
                    Op::GetStaticWide(class, field_slot)
                } else {
                    Op::GetStatic(class, field_slot)
                };

                return Ok((Op::Clinit(defining_class), Some(field_op)));
            }
            PUT_STATIC => {
                let field_ref_idx = read_u16_be!(context, data);
                let field_ref = constant_pool
                    .get_field_ref(field_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = loader.lookup_class(context, class_name)?;

                let descriptor = Descriptor::from_string(context, descriptor_name)?;

                let field_slot = class
                    .static_field_vtable()
                    .lookup((field_name, descriptor))
                    .ok_or_else(|| context.no_such_field_error())?;

                let field = class.get_static_field(field_slot);

                // TODO "package-private" and "protected" access control
                let flags = field.flags();
                if flags.contains(FieldFlags::PRIVATE) && method.class() != class {
                    return Err(context.illegal_access_error());
                }

                let defining_class = field.defining_class();

                let field_op = if descriptor.is_wide() {
                    Op::PutStaticWide(class, field_slot)
                } else {
                    Op::PutStatic(class, field_slot)
                };

                return Ok((Op::Clinit(defining_class), Some(field_op)));
            }
            GET_FIELD => {
                let field_ref_idx = read_u16_be!(context, data);
                let field_ref = constant_pool
                    .get_field_ref(field_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = loader.lookup_class(context, class_name)?;

                let descriptor = Descriptor::from_string(context, descriptor_name)?;

                let field_slot = class
                    .instance_field_vtable()
                    .lookup((field_name, descriptor))
                    .ok_or_else(|| context.no_such_field_error())?;

                // TODO "package-private" and "protected" access control
                let flags = class.get_instance_field(field_slot).flags();
                if flags.contains(FieldFlags::PRIVATE) && method.class() != class {
                    return Err(context.illegal_access_error());
                }

                if descriptor.is_wide() {
                    Op::GetFieldWide(class, field_slot)
                } else {
                    Op::GetField(class, field_slot)
                }
            }
            PUT_FIELD => {
                let field_ref_idx = read_u16_be!(context, data);
                let field_ref = constant_pool
                    .get_field_ref(field_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, field_name, descriptor_name) = field_ref;

                let class = loader.lookup_class(context, class_name)?;

                let descriptor = Descriptor::from_string(context, descriptor_name)?;

                let field_slot = class
                    .instance_field_vtable()
                    .lookup((field_name, descriptor))
                    .ok_or_else(|| context.no_such_field_error())?;

                // TODO "package-private" and "protected" access control
                let flags = class.get_instance_field(field_slot).flags();
                if flags.contains(FieldFlags::PRIVATE) && method.class() != class {
                    return Err(context.illegal_access_error());
                }

                if descriptor.is_wide() {
                    Op::PutFieldWide(class, field_slot)
                } else {
                    Op::PutField(class, field_slot)
                }
            }
            INVOKE_VIRTUAL => {
                let method_ref_idx = read_u16_be!(context, data);
                let method_ref = constant_pool
                    .get_method_ref(method_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, method_name, descriptor_name) = method_ref;

                // Method is called based on class of object on stack
                let class = loader.lookup_class(context, class_name)?;

                let descriptor = MethodDescriptor::from_string(context, descriptor_name)?;

                let method_index = class
                    .instance_method_vtable()
                    .lookup((method_name, descriptor))
                    .ok_or_else(|| {
                        let message = format!("{}.{}()", class_name, method_name);
                        context.no_such_method_error(&message)
                    })?;

                // TODO access control?

                let physical_arg_count = descriptor.physical_arg_count();

                if descriptor.return_type().is_wide() {
                    Op::InvokeVirtualWide(class, method_index, physical_arg_count)
                } else {
                    Op::InvokeVirtual(class, method_index, physical_arg_count)
                }
            }
            INVOKE_SPECIAL => {
                let method_ref_idx = read_u16_be!(context, data);
                let method_ref = constant_pool
                    .get_any_method_ref(method_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, method_name, descriptor_name) = method_ref;

                let class = loader.lookup_class(context, class_name)?;

                let current_class = method.class();

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

                let descriptor = MethodDescriptor::from_string(context, descriptor_name)?;

                let method_slot =
                    method_vtable
                        .lookup((method_name, descriptor))
                        .ok_or_else(|| {
                            let message = format!("{}.{}()", class_name, method_name);
                            context.no_such_method_error(&message)
                        })?;

                let method = method_vtable.get_element(method_slot);

                // TODO access control?

                Op::InvokeSpecial(method)
            }
            INVOKE_STATIC => {
                let method_ref_idx = read_u16_be!(context, data);
                let method_ref = constant_pool
                    .get_any_method_ref(method_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, method_name, descriptor_name) = method_ref;

                let class = loader.lookup_class(context, class_name)?;

                let descriptor = MethodDescriptor::from_string(context, descriptor_name)?;

                let method_slot = class
                    .static_method_vtable()
                    .lookup((method_name, descriptor))
                    .ok_or_else(|| {
                        let message = format!("{}.{}()", class_name, method_name);
                        context.no_such_method_error(&message)
                    })?;

                let method = class.get_static_method(method_slot);

                // TODO access control?

                Op::InvokeStatic(method)
            }
            INVOKE_INTERFACE => {
                let method_ref_idx = read_u16_be!(context, data);
                let method_ref = constant_pool
                    .get_interface_method_ref(method_ref_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let (class_name, method_name, descriptor_name) = method_ref;

                // Method is called based on class of object on stack
                let class = loader.lookup_class(context, class_name)?;

                let descriptor = MethodDescriptor::from_string(context, descriptor_name)?;

                // According to the JVMS, this byte states the argument count
                // of the method (despite that also being defined in the
                // descriptor) for "historical" reasons.
                let _arg_count = read_u8!(context, data);

                // This should always be zero.
                let _ = read_u8!(context, data);

                // TODO access control?

                let invoke_interface = InvokeInterfaceInfo {
                    class,
                    name: method_name,
                    descriptor,
                };

                Op::InvokeInterface(Box::new(invoke_interface))
            }
            NEW => {
                let class_idx = read_u16_be!(context, data);
                let class_name = constant_pool
                    .get_class(class_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let class = loader.lookup_class(context, class_name)?;

                return Ok((Op::Clinit(class), Some(Op::New(class))));
            }
            NEW_ARRAY => {
                let array_type = match read_u8!(context, data) {
                    4 => ArrayType::Boolean,
                    5 => ArrayType::Char,
                    6 => ArrayType::Float,
                    7 => ArrayType::Double,
                    8 => ArrayType::Byte,
                    9 => ArrayType::Short,
                    10 => ArrayType::Int,
                    11 => ArrayType::Long,
                    _ => return Err(context.verify_error("Invalid array type")),
                };

                Op::NewArray(array_type)
            }
            A_NEW_ARRAY => {
                let class_idx = read_u16_be!(context, data);
                let class_name = constant_pool
                    .get_class(class_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let class = loader.lookup_class(context, class_name)?;

                Op::ANewArray(class)
            }
            ARRAY_LENGTH => Op::ArrayLength,
            A_THROW => Op::AThrow,
            CHECK_CAST => {
                let class_idx = read_u16_be!(context, data);
                let class_name = constant_pool
                    .get_class(class_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let class = loader.lookup_class(context, class_name)?;

                Op::CheckCast(class)
            }
            INSTANCE_OF => {
                let class_idx = read_u16_be!(context, data);
                let class_name = constant_pool
                    .get_class(class_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let class = loader.lookup_class(context, class_name)?;

                Op::InstanceOf(class)
            }
            MONITOR_ENTER => Op::MonitorEnter,
            MONITOR_EXIT => Op::MonitorExit,
            MULTI_A_NEW_ARRAY => {
                let class_idx = read_u16_be!(context, data);
                let class_name = constant_pool
                    .get_class(class_idx)
                    .map_err(|e| Error::from_class_file_error(context, e))?;

                let dim_count = read_u8!(context, data);

                if dim_count == 0 {
                    return Err(context.verify_error("multianewarray: dim_count must be > 0"));
                }

                // TODO this probably should go through a different path than
                // `Descriptor::from_string`, as that function is meant for
                // field descriptors (maybe a "lookup class" function)?
                let descriptor = Descriptor::from_string(context, class_name)?;
                let mut resolved_descriptor =
                    ResolvedDescriptor::from_descriptor(context, loader, descriptor)?;

                for _ in 0..dim_count {
                    resolved_descriptor =
                        match resolved_descriptor {
                            ResolvedDescriptor::Array(array_class) => {
                                array_class.array_value_type().unwrap()
                            }
                            _ => return Err(context.verify_error(
                                "multianewarray: class must be a dim_count-dimensional array type",
                            )),
                        }
                }

                let multi_a_new_array = MultiANewArrayInfo {
                    class: resolved_descriptor,
                    dimensions: dim_count,
                };

                Op::MultiANewArray(Box::new(multi_a_new_array))
            }
            IF_NULL => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfNull(((data_position as isize) + offset) as usize)
            }
            IF_NON_NULL => {
                let offset = read_u16_be!(context, data) as i16 as isize;

                Op::IfNonNull(((data_position as isize) + offset) as usize)
            }
            GOTO_W => {
                let offset = read_u32_be!(context, data) as i32 as isize;

                Op::Goto(((data_position as isize) + offset) as usize)
            }
            JSR_W => {
                let offset = read_u32_be!(context, data) as i32 as isize;

                Op::Jsr(((data_position as isize) + offset) as usize)
            }
            other => unimplemented!(
                "Unimplemented opcode {} ({}.{})",
                other,
                method.class().name(),
                method.name()
            ),
        };

        Ok((result, None))
    }

    pub fn can_throw_error(&self) -> bool {
        matches!(
            self,
            Op::Ldc(_)
                | Op::IaLoad
                | Op::LaLoad
                | Op::FaLoad
                | Op::DaLoad
                | Op::AaLoad
                | Op::BaLoad
                | Op::CaLoad
                | Op::SaLoad
                | Op::IaStore
                | Op::LaStore
                | Op::FaStore
                | Op::DaStore
                | Op::AaStore
                | Op::BaStore
                | Op::CaStore
                | Op::SaStore
                | Op::IDiv
                | Op::LDiv
                | Op::IRem
                | Op::LRem
                | Op::GetField(_, _)
                | Op::PutField(_, _)
                | Op::GetFieldWide(_, _)
                | Op::PutFieldWide(_, _)
                | Op::InvokeVirtual(_, _, _)
                | Op::InvokeVirtualWide(_, _, _)
                | Op::InvokeSpecial(_)
                | Op::InvokeStatic(_)
                | Op::InvokeInterface(_)
                | Op::NewArray(_)
                | Op::ANewArray(_)
                | Op::ArrayLength
                | Op::AThrow
                | Op::CheckCast(_)
                | Op::MonitorEnter
                | Op::MonitorExit
                | Op::MultiANewArray(_)
                | Op::Clinit(_)
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TableSwitchInfo {
    pub low_int: i32,
    pub matches: Box<[usize]>,
    pub default_offset: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct LookupSwitchInfo {
    pub matches: Box<[(i32, usize)]>,
    pub default_offset: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct InvokeInterfaceInfo {
    pub class: Class,
    pub name: JvmString,
    pub descriptor: MethodDescriptor,
}

#[derive(Clone, Debug)]
pub(crate) struct MultiANewArrayInfo {
    pub class: ResolvedDescriptor,
    pub dimensions: u8,
}

#[cfg(target_pointer_width = "64")]
const _: () = assert!(core::mem::size_of::<Op>() == 16);
