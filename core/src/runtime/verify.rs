use super::descriptor::Descriptor;
use super::error::{Error, NativeError};
use super::method::{Exception, Method};
use super::op::Op;

use crate::classfile::constant_pool::ConstantPoolEntry;
use crate::classfile::flags::MethodFlags;

use std::collections::{HashMap, HashSet};

// The possible ways a block can be exited.
#[derive(Debug)]
enum BlockExits {
    // Such as for LookupSwitch. When execution is finished, the next block is
    // any of the blocks with its first op any of the given indices.
    BranchMultiple(Box<[usize]>),

    // Such as for exceptions. When execution is finished, the next block is any
    // of the blocks with its first op any of the given indices or the next
    // block in the block list, and a Reference will be pushed on the abstract
    // interpreter's stack.
    BranchMultipleException(Box<[usize]>),

    // Such as for IfEq. When execution is finished, the next block is either
    // the block with its first op at the index specified or the next block in the
    // block list.
    Branch(usize),

    // Such as for Goto. When execution is finished, the next block is the block
    // with its first op at the index specified.
    Goto(usize),

    // Such as for a block ending at the target of a jump. When execution is
    // finished, the next block is the next block in the block list.
    NextBlock,

    // Such as for Return. When execution is finished, the function returns.
    Return,
}

#[derive(Debug)]
struct BasicBlock<'a> {
    // The index of the first op making up this BasicBlock.
    start_index: usize,

    ops: &'a [Op],

    exits: BlockExits,
}

pub fn verify_ops<'a>(
    method: Method,
    max_stack: usize,
    max_locals: usize,
    ops: &'a [Op],
    exceptions: &[Exception],
) -> Result<(), Error> {
    let blocks = collect_basic_blocks(ops, exceptions)?;

    verify_blocks(method, max_stack, max_locals, blocks)
}

fn collect_basic_blocks<'a>(
    ops: &'a [Op],
    exceptions: &[Exception],
) -> Result<Vec<BasicBlock<'a>>, Error> {
    let mut block_list = Vec::with_capacity(2);
    let mut current_block_start = 0;

    // First, verify that code does not fall off the method and list jump targets.
    let mut visited_locations = HashSet::new();
    let mut worklist = Vec::with_capacity(2);
    worklist.push(0);
    while let Some(mut i) = worklist.pop() {
        if i >= ops.len() {
            return Err(Error::Native(NativeError::CodeFellOffMethod));
        }

        loop {
            let op = &ops[i];

            if op.can_throw_error() {
                for exception in exceptions {
                    if i >= exception.start && i < exception.end {
                        if !visited_locations.contains(&exception.target) {
                            visited_locations.insert(exception.target);
                            worklist.push(exception.target);
                        }
                    }
                }
            }

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
                | Op::IfNull(position)
                | Op::IfNonNull(position) => {
                    if !visited_locations.contains(position) {
                        visited_locations.insert(*position);
                        worklist.push(*position);
                    }
                }
                Op::Goto(position) => {
                    if !visited_locations.contains(position) {
                        visited_locations.insert(*position);
                        worklist.push(*position);
                    }
                    break;
                }
                Op::TableSwitch(_, _, matches, default_offset) => {
                    if !visited_locations.contains(default_offset) {
                        visited_locations.insert(*default_offset);
                        worklist.push(*default_offset);
                    }

                    for offset in matches {
                        if !visited_locations.contains(offset) {
                            visited_locations.insert(*offset);
                            worklist.push(*offset);
                        }
                    }

                    break;
                }
                Op::LookupSwitch(matches, default_offset) => {
                    if !visited_locations.contains(default_offset) {
                        visited_locations.insert(*default_offset);
                        worklist.push(*default_offset);
                    }

                    for (_, offset) in matches {
                        if !visited_locations.contains(offset) {
                            visited_locations.insert(*offset);
                            worklist.push(*offset);
                        }
                    }

                    break;
                }
                Op::AThrow | Op::IReturn | Op::LReturn | Op::DReturn | Op::AReturn | Op::Return => {
                    break;
                }
                _ => {}
            }

            i += 1;

            if i >= ops.len() {
                return Err(Error::Native(NativeError::CodeFellOffMethod));
            }
        }
    }

    // Next, actually assemble the basic block list.
    // TODO this assumes none of the branch/goto ops will be able to throw an error, is that true?
    // Seems like the return ops can throw IllegalMonitorStateException in a compliant
    // JVM implementation, but we don't support threading
    for (i, op) in ops.iter().enumerate() {
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
            | Op::IfNull(position)
            | Op::IfNonNull(position) => {
                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::Branch(*position),
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            Op::Goto(position) => {
                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::Goto(*position),
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            Op::TableSwitch(_, _, matches, default_offset) => {
                let mut possible_target_list = vec![*default_offset];

                for offset in matches {
                    possible_target_list.push(*offset);
                }

                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::BranchMultiple(possible_target_list.into_boxed_slice()),
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            Op::LookupSwitch(matches, default_offset) => {
                let mut possible_target_list = vec![*default_offset];

                for (_, offset) in matches {
                    possible_target_list.push(*offset);
                }

                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::BranchMultiple(possible_target_list.into_boxed_slice()),
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            // TODO should AThrow not in a try area also count as a return? Class
            // might expect unreachable code after the AThrow to not be verified
            Op::IReturn | Op::LReturn | Op::DReturn | Op::AReturn | Op::Return => {
                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::Return,
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            _ => {
                let mut could_exception_branch = false;
                let mut exception_branches = Vec::new();
                if op.can_throw_error() {
                    for exception in exceptions {
                        if i >= exception.start && i < exception.end {
                            // This op is a branch to the exception target block.
                            could_exception_branch = true;

                            exception_branches.push(exception.target);
                        }
                    }
                }

                if could_exception_branch {
                    let block = BasicBlock {
                        start_index: current_block_start,
                        ops: &ops[current_block_start..i + 1],
                        exits: BlockExits::BranchMultipleException(
                            exception_branches.into_boxed_slice(),
                        ),
                    };

                    block_list.push(block);

                    current_block_start = i + 1;
                } else if matches!(op, Op::AThrow) {
                    // AThrow without an exception around it must return.
                    let block = BasicBlock {
                        start_index: current_block_start,
                        ops: &ops[current_block_start..i + 1],
                        exits: BlockExits::Return,
                    };

                    block_list.push(block);

                    current_block_start = i + 1;
                } else {
                    // If the next op is a jump target, this block ends here.
                    // This includes exception targets.
                    if visited_locations.contains(&(i + 1)) {
                        let block = BasicBlock {
                            start_index: current_block_start,
                            ops: &ops[current_block_start..i + 1],
                            exits: BlockExits::NextBlock,
                        };

                        block_list.push(block);

                        current_block_start = i + 1;
                    }
                }
            }
        }
    }

    // Create a table mapping op indices to block indicies.
    let mut op_index_to_block_index_table = HashMap::new();
    for (i, block) in block_list.iter().enumerate() {
        op_index_to_block_index_table.insert(block.start_index, i);
    }

    // Now convert the op indices mentioned in BlockExits to BB indices.
    for block in block_list.iter_mut() {
        match block.exits {
            BlockExits::BranchMultiple(ref mut branches)
            | BlockExits::BranchMultipleException(ref mut branches) => {
                for branch in branches {
                    *branch = *op_index_to_block_index_table
                        .get(branch)
                        .expect("Op index should map to valid block index");
                }
            }
            BlockExits::Branch(ref mut branch) | BlockExits::Goto(ref mut branch) => {
                *branch = *op_index_to_block_index_table
                    .get(branch)
                    .expect("Op index should map to valid block index");
            }
            _ => {}
        }
    }

    Ok(block_list)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ValueType {
    Invalid,
    Integer,
    Long,
    Float,
    Double,
    Reference,
}

impl ValueType {
    fn is_wide(self) -> bool {
        match self {
            ValueType::Invalid => unreachable!(),
            ValueType::Integer | ValueType::Float | ValueType::Reference => false,
            ValueType::Long | ValueType::Double => true,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FrameState {
    locals: Box<[ValueType]>,
    stack: Vec<ValueType>,
}

fn verify_blocks<'a>(
    method: Method,
    max_stack: usize,
    max_locals: usize,
    blocks: Vec<BasicBlock<'a>>,
) -> Result<(), Error> {
    let descriptor = method.descriptor();
    let mut args = Vec::with_capacity(2);
    if !method.flags().contains(MethodFlags::STATIC) {
        args.push(ValueType::Reference);
    }

    for arg in descriptor.args() {
        match arg {
            Descriptor::Class(_) | Descriptor::Array(_) => {
                args.push(ValueType::Reference);
            }
            Descriptor::Boolean
            | Descriptor::Byte
            | Descriptor::Character
            | Descriptor::Short
            | Descriptor::Integer => {
                args.push(ValueType::Integer);
            }
            Descriptor::Float => {
                args.push(ValueType::Float);
            }
            Descriptor::Double => {
                args.push(ValueType::Double);
                args.push(ValueType::Invalid);
            }
            Descriptor::Long => {
                args.push(ValueType::Long);
                args.push(ValueType::Invalid);
            }
            Descriptor::Void => unreachable!(),
        }
    }

    if args.len() > max_locals {
        return Err(Error::Native(NativeError::VerifyCountWrong));
    }

    let mut entry_locals = vec![ValueType::Invalid; max_locals];
    for (i, arg) in args.iter().enumerate() {
        entry_locals[i] = *arg;
    }

    // The frame state when the entry block is executed.
    let entry_frame_state = FrameState {
        locals: entry_locals.into_boxed_slice(),
        stack: Vec::new(),
    };

    // A HashSet of (block index, frame state)
    let mut verified_states = HashSet::new();

    // Block #0 is the entry block
    let mut worklist = Vec::new();
    worklist.push((0, entry_frame_state));
    while let Some((block_idx, initial_frame_state)) = worklist.pop() {
        let block = &blocks[block_idx];

        if !verified_states.contains(&(block_idx, initial_frame_state.clone())) {
            // Clone :/
            verified_states.insert((block_idx, initial_frame_state.clone()));

            verify_block(
                block,
                block_idx,
                max_stack,
                initial_frame_state,
                &mut worklist,
            )?;
        }
    }

    Ok(())
}

fn verify_block<'a>(
    block: &BasicBlock<'a>,
    block_idx: usize,
    max_stack: usize,
    mut frame_state: FrameState,
    worklist: &mut Vec<(usize, FrameState)>,
) -> Result<(), Error> {
    let ops = block.ops;

    let stack = &mut frame_state.stack;
    let locals = &mut frame_state.locals;

    macro_rules! push_stack {
        ($value_type:ident) => {
            stack.push(ValueType::$value_type);
            if stack.len() > max_stack {
                return Err(Error::Native(NativeError::VerifyCountWrong));
            }
        };
    }

    macro_rules! expect_pop_stack {
        ($expected_type:ident) => {
            if let Some(value) = stack.pop() {
                if !matches!(value, ValueType::$expected_type) {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                }
            } else {
                return Err(Error::Native(NativeError::VerifyCountWrong));
            }
        };
    }

    macro_rules! set_local {
        ($local_index:expr, $value_type:ident) => {
            *locals
                .get_mut($local_index)
                .ok_or(Error::Native(NativeError::VerifyCountWrong))? = ValueType::$value_type;
        };
    }

    macro_rules! expect_local {
        ($local_index:expr, $expected_type:ident) => {
            let value = locals
                .get($local_index)
                .ok_or(Error::Native(NativeError::VerifyCountWrong))?;
            if !matches!(value, ValueType::$expected_type) {
                return Err(Error::Native(NativeError::VerifyTypeWrong));
            }
        };
    }

    for op in ops {
        match op {
            Op::Nop => {}
            Op::AConstNull => {
                push_stack!(Reference);
            }
            Op::IConst(_) => {
                push_stack!(Integer);
            }
            Op::LConst(_) => {
                push_stack!(Long);
            }
            Op::FConst(_) => {
                push_stack!(Float);
            }
            Op::DConst(_) => {
                push_stack!(Double);
            }
            Op::Ldc(constant_pool_entry) => match constant_pool_entry {
                ConstantPoolEntry::String { .. } => {
                    push_stack!(Reference);
                }
                ConstantPoolEntry::Integer { .. } => {
                    push_stack!(Integer);
                }
                ConstantPoolEntry::Float { .. } => {
                    push_stack!(Float);
                }
                ConstantPoolEntry::Class { .. } => {
                    push_stack!(Reference);
                }
                _ => unimplemented!(),
            },
            Op::Ldc2(constant_pool_entry) => match constant_pool_entry {
                ConstantPoolEntry::Long { .. } => {
                    push_stack!(Long);
                }
                ConstantPoolEntry::Double { .. } => {
                    push_stack!(Double);
                }
                _ => panic!("Ldc2 expects Long or Double entry"),
            },
            Op::ILoad(index) => {
                expect_local!(*index, Integer);
                push_stack!(Integer);
            }
            Op::LLoad(index) => {
                expect_local!(*index, Long);
                if index + 1 >= locals.len() {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }

                push_stack!(Long);
            }
            Op::FLoad(index) => {
                expect_local!(*index, Float);
                push_stack!(Float);
            }
            Op::DLoad(index) => {
                expect_local!(*index, Double);
                if index + 1 >= locals.len() {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }

                push_stack!(Double);
            }
            Op::ALoad(index) => {
                expect_local!(*index, Reference);
                push_stack!(Reference);
            }
            Op::IaLoad | Op::BaLoad | Op::CaLoad | Op::SaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Integer);
            }
            Op::LaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Long);
            }
            Op::FaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Float);
            }
            Op::DaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Double);
            }
            Op::AaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Reference);
            }
            Op::IStore(index) => {
                expect_pop_stack!(Integer);
                set_local!(*index, Integer);
            }
            Op::LStore(index) => {
                expect_pop_stack!(Long);
                set_local!(*index, Long);

                // The docs aren't clear on this, but this is expected
                set_local!(*index + 1, Invalid);
            }
            Op::FStore(index) => {
                expect_pop_stack!(Float);
                set_local!(*index, Float);
            }
            Op::DStore(index) => {
                expect_pop_stack!(Double);
                set_local!(*index, Double);

                // The docs aren't clear on this, but this is expected
                set_local!(*index + 1, Invalid);
            }
            Op::AStore(index) => {
                expect_pop_stack!(Reference);
                set_local!(*index, Reference);
            }
            Op::IaStore | Op::BaStore | Op::CaStore | Op::SaStore => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
            }
            Op::LaStore => {
                expect_pop_stack!(Long);
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
            }
            Op::FaStore => {
                expect_pop_stack!(Float);
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
            }
            Op::DaStore => {
                expect_pop_stack!(Double);
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
            }
            Op::AaStore => {
                expect_pop_stack!(Reference);
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
            }
            Op::Pop => {
                let value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if value.is_wide() {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                }
            }
            Op::Pop2 => {
                let value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if !value.is_wide() {
                    stack
                        .pop()
                        .ok_or(Error::Native(NativeError::VerifyCountWrong))?;
                }
            }
            Op::Dup => {
                let value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if value.is_wide() {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                } else {
                    stack.push(value);
                    stack.push(value);

                    if stack.len() > max_stack {
                        return Err(Error::Native(NativeError::VerifyCountWrong));
                    }
                }
            }
            Op::DupX1 => {
                let top_value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                let under_value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if top_value.is_wide() || under_value.is_wide() {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                }

                stack.push(top_value);
                stack.push(under_value);
                stack.push(top_value);

                if stack.len() > max_stack {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }
            }
            Op::DupX2 => {
                let top_value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if top_value.is_wide() {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                }

                let under_value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if under_value.is_wide() {
                    stack.push(top_value);
                    stack.push(under_value);
                    stack.push(top_value);
                } else {
                    let under_value_2 = stack
                        .pop()
                        .ok_or(Error::Native(NativeError::VerifyCountWrong))?;
                    stack.push(top_value);
                    stack.push(under_value_2);
                    stack.push(under_value);
                    stack.push(top_value);
                }

                if stack.len() > max_stack {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }
            }
            Op::Dup2 => {
                let value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if value.is_wide() {
                    stack.push(value);
                    stack.push(value);
                } else {
                    let second_value = stack
                        .pop()
                        .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                    if second_value.is_wide() {
                        return Err(Error::Native(NativeError::VerifyTypeWrong));
                    } else {
                        stack.push(second_value);
                        stack.push(value);
                        stack.push(second_value);
                        stack.push(value);
                    }
                }

                if stack.len() > max_stack {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }
            }
            Op::Swap => {
                let first_value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if first_value.is_wide() {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                }

                let second_value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;

                if second_value.is_wide() {
                    return Err(Error::Native(NativeError::VerifyTypeWrong));
                }

                stack.push(first_value);
                stack.push(second_value);
            }
            Op::IAdd | Op::ISub | Op::IMul | Op::IDiv | Op::IRem => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::LAdd | Op::LSub | Op::LMul | Op::LDiv | Op::LRem => {
                expect_pop_stack!(Long);
                expect_pop_stack!(Long);
                push_stack!(Long);
            }
            Op::FAdd | Op::FSub | Op::FMul | Op::FDiv | Op::FRem => {
                expect_pop_stack!(Float);
                expect_pop_stack!(Float);
                push_stack!(Float);
            }
            Op::DAdd | Op::DSub | Op::DMul | Op::DDiv | Op::DRem => {
                expect_pop_stack!(Double);
                expect_pop_stack!(Double);
                push_stack!(Double);
            }
            Op::INeg => {
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::LNeg => {
                expect_pop_stack!(Long);
                push_stack!(Long);
            }
            Op::FNeg => {
                expect_pop_stack!(Float);
                push_stack!(Float);
            }
            Op::DNeg => {
                expect_pop_stack!(Double);
                push_stack!(Double);
            }
            Op::IShl | Op::IShr | Op::IUshr => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::LShl | Op::LShr | Op::LUshr => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Long);
                push_stack!(Long);
            }
            Op::IAnd | Op::IOr | Op::IXor => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::LAnd | Op::LOr | Op::LXor => {
                expect_pop_stack!(Long);
                expect_pop_stack!(Long);
                push_stack!(Long);
            }
            Op::IInc(index, _) => {
                expect_local!(*index, Integer);
            }
            Op::I2L => {
                expect_pop_stack!(Integer);
                push_stack!(Long);
            }
            Op::I2F => {
                expect_pop_stack!(Integer);
                push_stack!(Float);
            }
            Op::I2D => {
                expect_pop_stack!(Integer);
                push_stack!(Double);
            }
            Op::L2I => {
                expect_pop_stack!(Long);
                push_stack!(Integer);
            }
            Op::F2I => {
                expect_pop_stack!(Float);
                push_stack!(Integer);
            }
            Op::D2I => {
                expect_pop_stack!(Double);
                push_stack!(Integer);
            }
            Op::I2B | Op::I2C | Op::I2S => {
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::LCmp => {
                expect_pop_stack!(Long);
                expect_pop_stack!(Long);
                push_stack!(Integer);
            }
            Op::FCmpL | Op::FCmpG => {
                expect_pop_stack!(Float);
                expect_pop_stack!(Float);
                push_stack!(Integer);
            }
            Op::DCmpL | Op::DCmpG => {
                expect_pop_stack!(Double);
                expect_pop_stack!(Double);
                push_stack!(Integer);
            }
            Op::IfEq(_) | Op::IfNe(_) | Op::IfLt(_) | Op::IfGe(_) | Op::IfGt(_) | Op::IfLe(_) => {
                expect_pop_stack!(Integer);
            }
            Op::IfICmpEq(_)
            | Op::IfICmpNe(_)
            | Op::IfICmpLt(_)
            | Op::IfICmpGe(_)
            | Op::IfICmpGt(_)
            | Op::IfICmpLe(_) => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
            }
            Op::IfACmpEq(_) | Op::IfACmpNe(_) => {
                expect_pop_stack!(Reference);
                expect_pop_stack!(Reference);
            }
            Op::Goto(_) => {
                // This does nothing
            }
            Op::TableSwitch(_, _, _, _) => {
                expect_pop_stack!(Integer);
            }
            Op::LookupSwitch(_, _) => {
                expect_pop_stack!(Integer);
            }
            Op::IReturn => {
                expect_pop_stack!(Integer);
            }
            Op::LReturn => {
                expect_pop_stack!(Long);
            }
            Op::DReturn => {
                expect_pop_stack!(Double);
            }
            Op::AReturn => {
                expect_pop_stack!(Reference);
            }
            Op::Return => {
                // This does nothing
            }
            Op::GetStatic(class, index) => {
                let field_descriptor = class.static_fields()[*index].descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(Float);
                    }
                    Descriptor::Double => {
                        push_stack!(Double);
                    }
                    Descriptor::Long => {
                        push_stack!(Long);
                    }
                    Descriptor::Void => unreachable!(),
                }
            }
            Op::PutStatic(class, index) => {
                let field_descriptor = class.static_fields()[*index].descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        expect_pop_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        expect_pop_stack!(Integer);
                    }
                    Descriptor::Float => {
                        expect_pop_stack!(Float);
                    }
                    Descriptor::Double => {
                        expect_pop_stack!(Double);
                    }
                    Descriptor::Long => {
                        expect_pop_stack!(Long);
                    }
                    Descriptor::Void => unreachable!(),
                }
            }
            Op::GetField(class, index) => {
                expect_pop_stack!(Reference);

                let field_descriptor = class.instance_fields()[*index].descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(Float);
                    }
                    Descriptor::Double => {
                        push_stack!(Double);
                    }
                    Descriptor::Long => {
                        push_stack!(Long);
                    }
                    Descriptor::Void => unreachable!(),
                }
            }
            Op::PutField(class, index) => {
                let field_descriptor = class.instance_fields()[*index].descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        expect_pop_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        expect_pop_stack!(Integer);
                    }
                    Descriptor::Float => {
                        expect_pop_stack!(Float);
                    }
                    Descriptor::Double => {
                        expect_pop_stack!(Double);
                    }
                    Descriptor::Long => {
                        expect_pop_stack!(Long);
                    }
                    Descriptor::Void => unreachable!(),
                }

                expect_pop_stack!(Reference);
            }
            Op::InvokeVirtual(class, method_index) => {
                // We need to know the number and type of args, so let's lookup the
                // method defined by the base class to get the descriptor- this is
                // the wrong method, but we can still use its descriptor
                let descriptor = class
                    .instance_method_vtable()
                    .get_element(*method_index)
                    .descriptor();

                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                // Receiver
                expect_pop_stack!(Reference);

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(Float);
                    }
                    Descriptor::Double => {
                        push_stack!(Double);
                    }
                    Descriptor::Long => {
                        push_stack!(Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::InvokeSpecial(_, method) => {
                let descriptor = method.descriptor();
                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                // Receiver
                expect_pop_stack!(Reference);

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(Float);
                    }
                    Descriptor::Double => {
                        push_stack!(Double);
                    }
                    Descriptor::Long => {
                        push_stack!(Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::InvokeStatic(method) => {
                let descriptor = method.descriptor();
                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(Float);
                    }
                    Descriptor::Double => {
                        push_stack!(Double);
                    }
                    Descriptor::Long => {
                        push_stack!(Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::InvokeInterface(_, (_, descriptor)) => {
                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                // Receiver
                expect_pop_stack!(Reference);

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(Float);
                    }
                    Descriptor::Double => {
                        push_stack!(Double);
                    }
                    Descriptor::Long => {
                        push_stack!(Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::New(_) => {
                push_stack!(Reference);
            }
            Op::NewArray(_) => {
                expect_pop_stack!(Integer);
                push_stack!(Reference);
            }
            Op::ANewArray(_) => {
                expect_pop_stack!(Integer);
                push_stack!(Reference);
            }
            Op::ArrayLength => {
                expect_pop_stack!(Reference);
                push_stack!(Integer);
            }
            Op::AThrow => {
                expect_pop_stack!(Reference);
            }
            Op::CheckCast(_) => {
                expect_pop_stack!(Reference);
                push_stack!(Reference);
            }
            Op::InstanceOf(_) => {
                expect_pop_stack!(Reference);
                push_stack!(Integer);
            }
            Op::MonitorEnter => {
                expect_pop_stack!(Reference);
            }
            Op::MonitorExit => {
                expect_pop_stack!(Reference);
            }
            Op::MultiANewArray(_, dimension_count) => {
                for _ in 0..(*dimension_count) {
                    expect_pop_stack!(Integer);
                }

                push_stack!(Reference);
            }
            Op::IfNull(_) => {
                expect_pop_stack!(Reference);
            }
            Op::IfNonNull(_) => {
                expect_pop_stack!(Reference);
            }
        }
    }

    match block.exits {
        BlockExits::BranchMultiple(ref to_blocks) => {
            for to_block in to_blocks {
                worklist.push((*to_block, frame_state.clone()));
            }
        }
        BlockExits::BranchMultipleException(ref to_blocks) => {
            // Ensure that the initial stack state at each of the exception
            // targets is a stack with a single Reference on it.
            *stack = vec![ValueType::Reference];

            for to_block in to_blocks {
                worklist.push((*to_block, frame_state.clone()));
            }
        }
        BlockExits::Goto(to_block) => {
            worklist.push((to_block, frame_state));
        }
        BlockExits::Branch(to_block) => {
            worklist.push((to_block, frame_state.clone()));
            worklist.push((block_idx + 1, frame_state));
        }
        BlockExits::NextBlock => {
            worklist.push((block_idx + 1, frame_state));
        }
        BlockExits::Return => {}
    }
    Ok(())
}
