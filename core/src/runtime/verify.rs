use super::context::Context;
use super::descriptor::Descriptor;
use super::error::Error;
use super::method::{Exception, Method};
use super::op::Op;

use crate::classfile::constant_pool::ConstantPoolEntry;
use crate::classfile::flags::MethodFlags;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use hashbrown::{HashMap, HashSet};

/*
Verifier notes

The verifier first creates a list of basic blocks and then runs a "perfect"
abstract interpreter over them, i.e. it verifies every possible state of the
interpreter. The A.I. being "perfect" allows us to perfectly validate
subroutines (jsr/ret), which means that this VM/verifier will accept some
valid subroutines that other VMs/verifiers will not. The verifier is still
perfectly sound.

However, the verifier being "perfect" comes with some downsides:

- The A.I. is algorithmically (very) slow. Because it cannot do program state
  merging, certain blocks may be needlessly re-verified.

- The verifier does not output a "canon"/fully merged state list post-
  verification, because it does not have one. If we want a canon state list, we
  must perform merging manually post-verification. We don't currently need a
  canon state list, though we may need one in the future if we ever want to
  implement certain non-peephole bytecode optimizations.
*/

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

    // For Ret. When execution is finished, the next block is the block
    // specified by the ReturnAddress in a local slot specified by the Ret
    // instruction. It is impossible to statically know (pre-abstract
    // interpretation) where this block exit can return to.
    SubroutineReturn,

    // Such as for a block ending at the target of a jump. When execution is
    // finished, the next block is the next block in the block list.
    NextBlock,

    // Such as for Return. When execution is finished, the function returns.
    Return,
}

struct BasicBlock<'a> {
    // The index of the first op making up this BasicBlock.
    start_index: usize,

    ops: &'a [Op],

    exits: BlockExits,
}

impl fmt::Debug for BasicBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Start: {}\n", self.start_index)?;
        for op in self.ops {
            write!(f, "    {:?}\n", op)?;
        }
        write!(f, "Exits: {:?}", self.exits)
    }
}

pub fn verify_ops(
    method: Method,
    max_stack: usize,
    max_locals: usize,
    ops: &[Op],
    exceptions: &[Exception],
) -> Result<(), VerifyError> {
    let (blocks, op_index_to_block_index_table) = collect_basic_blocks(ops, exceptions)?;

    verify_blocks(
        method,
        max_stack,
        max_locals,
        &blocks,
        &op_index_to_block_index_table,
    )
}

fn collect_basic_blocks<'a>(
    ops: &'a [Op],
    exceptions: &[Exception],
) -> Result<(Vec<BasicBlock<'a>>, HashMap<usize, usize>), VerifyError> {
    let mut block_list = Vec::with_capacity(2);
    let mut current_block_start = 0;

    // First, verify that code does not fall off the method and list jump targets.
    let mut visited_locations = HashSet::new();
    let mut worklist = Vec::with_capacity(2);
    worklist.push(0);
    while let Some(mut i) = worklist.pop() {
        if i >= ops.len() {
            return Err(VerifyError::CodeFellOffMethod);
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
                Op::Jsr(position) => {
                    if !visited_locations.contains(position) {
                        visited_locations.insert(*position);
                        worklist.push(*position);
                    }
                    // The Ret corresponding to a Jsr can allow the Jsr to
                    // "fall through", so we can't `break` here.
                }
                Op::TableSwitch(table_switch) => {
                    let matches = &table_switch.matches;
                    let default_offset = table_switch.default_offset;

                    if !visited_locations.contains(&default_offset) {
                        visited_locations.insert(default_offset);
                        worklist.push(default_offset);
                    }

                    for offset in matches {
                        if !visited_locations.contains(offset) {
                            visited_locations.insert(*offset);
                            worklist.push(*offset);
                        }
                    }

                    break;
                }
                Op::LookupSwitch(lookup_switch) => {
                    let matches = &lookup_switch.matches;
                    let default_offset = lookup_switch.default_offset;

                    if !visited_locations.contains(&default_offset) {
                        visited_locations.insert(default_offset);
                        worklist.push(default_offset);
                    }

                    for (_, offset) in matches {
                        if !visited_locations.contains(offset) {
                            visited_locations.insert(*offset);
                            worklist.push(*offset);
                        }
                    }

                    break;
                }
                Op::AThrow
                | Op::IReturn
                | Op::LReturn
                | Op::FReturn
                | Op::DReturn
                | Op::AReturn
                | Op::Return => {
                    break;
                }
                _ => {}
            }

            i += 1;

            if i >= ops.len() {
                return Err(VerifyError::CodeFellOffMethod);
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
            Op::Jsr(position) => {
                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::Goto(*position),
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            Op::Ret(_) => {
                let block = BasicBlock {
                    start_index: current_block_start,
                    ops: &ops[current_block_start..i + 1],
                    exits: BlockExits::SubroutineReturn,
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
            Op::TableSwitch(table_switch) => {
                let matches = &table_switch.matches;
                let default_offset = table_switch.default_offset;

                let mut possible_target_list = vec![default_offset];

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
            Op::LookupSwitch(lookup_switch) => {
                let matches = &lookup_switch.matches;
                let default_offset = lookup_switch.default_offset;

                let mut possible_target_list = vec![default_offset];

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
            Op::IReturn | Op::LReturn | Op::FReturn | Op::DReturn | Op::AReturn | Op::Return => {
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

    Ok((block_list, op_index_to_block_index_table))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ValueType {
    Invalid,
    Integer,
    Long,
    Float,
    Double,
    Reference,
    ReturnAddress(usize),
}

impl ValueType {
    fn is_wide(self) -> bool {
        match self {
            ValueType::Invalid => unreachable!(),
            ValueType::Integer
            | ValueType::Float
            | ValueType::Reference
            | ValueType::ReturnAddress(_) => false,
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
    blocks: &[BasicBlock<'a>],
    op_index_to_block_index_table: &HashMap<usize, usize>,
) -> Result<(), VerifyError> {
    let mut entry_locals = vec![ValueType::Invalid; max_locals];

    // Set up the entry state's locals from the method's receiver (if it has
    // one) and arguments
    let mut i = 0;

    if !method.flags().contains(MethodFlags::STATIC) {
        if max_locals == 0 {
            // If the method has a receiver, we need at least one local slot
            // to store the receiver into
            return Err(VerifyError::WrongCount);
        }

        entry_locals[i] = ValueType::Reference;
        i += 1;
    }

    let descriptor = method.descriptor();
    for arg in descriptor.args() {
        if i >= entry_locals.len() {
            return Err(VerifyError::WrongCount);
        }

        match arg {
            Descriptor::Class(_) | Descriptor::Array(_) => {
                entry_locals[i] = ValueType::Reference;
                i += 1;
            }
            Descriptor::Boolean
            | Descriptor::Byte
            | Descriptor::Character
            | Descriptor::Short
            | Descriptor::Integer => {
                entry_locals[i] = ValueType::Integer;
                i += 1;
            }
            Descriptor::Float => {
                entry_locals[i] = ValueType::Float;
                i += 1;
            }
            Descriptor::Double => {
                entry_locals[i] = ValueType::Double;
                // Skip over the local after this one- it will remain invalid
                i += 2;
            }
            Descriptor::Long => {
                entry_locals[i] = ValueType::Long;
                // Skip over the local after this one- it will remain invalid
                i += 2;
            }
            Descriptor::Void => unreachable!(),
        }
    }

    // Now we have the frame state when the entry block is executed.
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

        let new_state = (block_idx, initial_frame_state.clone());
        if !verified_states.contains(&new_state) {
            verified_states.insert(new_state);

            verify_block(
                block,
                block_idx,
                max_stack,
                initial_frame_state,
                &mut worklist,
                op_index_to_block_index_table,
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
    op_index_to_block_index_table: &HashMap<usize, usize>,
) -> Result<(), VerifyError> {
    let ops = block.ops;

    let stack = &mut frame_state.stack;
    let locals = &mut frame_state.locals;

    macro_rules! push_stack {
        ($value_type:expr) => {
            stack.push($value_type);
            if stack.len() > max_stack {
                return Err(VerifyError::WrongCount);
            }
        };
    }

    macro_rules! expect_pop_stack {
        ($expected_type:pat) => {
            if let Some(value) = stack.pop() {
                if !matches!(value, $expected_type) {
                    return Err(VerifyError::WrongType);
                } else {
                    value
                }
            } else {
                return Err(VerifyError::WrongCount);
            }
        };
    }

    macro_rules! set_local {
        ($local_index:expr, $value_type:expr) => {
            *locals
                .get_mut($local_index)
                .ok_or(VerifyError::WrongCount)? = $value_type;
        };
    }

    macro_rules! expect_local {
        ($local_index:expr, $expected_type:pat) => {
            let value = locals.get($local_index).ok_or(VerifyError::WrongCount)?;
            if !matches!(value, $expected_type) {
                return Err(VerifyError::WrongType);
            }
        };
    }

    for (intra_block_index, op) in ops.iter().enumerate() {
        match op {
            Op::Nop => {}
            Op::AConstNull => {
                push_stack!(ValueType::Reference);
            }
            Op::IConst(_) => {
                push_stack!(ValueType::Integer);
            }
            Op::LConst(_) => {
                push_stack!(ValueType::Long);
            }
            Op::FConst(_) => {
                push_stack!(ValueType::Float);
            }
            Op::DConst(_) => {
                push_stack!(ValueType::Double);
            }
            Op::Ldc(entry) => match **entry {
                ConstantPoolEntry::String { .. } | ConstantPoolEntry::Class { .. } => {
                    push_stack!(ValueType::Reference);
                }
                ConstantPoolEntry::Integer { .. } => {
                    push_stack!(ValueType::Integer);
                }
                ConstantPoolEntry::Float { .. } => {
                    push_stack!(ValueType::Float);
                }
                _ => unreachable!(),
            },
            Op::LoadLong(_) => {
                push_stack!(ValueType::Long);
            }
            Op::LoadDouble(_) => {
                push_stack!(ValueType::Double);
            }
            Op::ILoad(index) => {
                expect_local!(*index, ValueType::Integer);
                push_stack!(ValueType::Integer);
            }
            Op::LLoad(index) => {
                expect_local!(*index, ValueType::Long);
                if index + 1 >= locals.len() {
                    return Err(VerifyError::WrongCount);
                }

                push_stack!(ValueType::Long);
            }
            Op::FLoad(index) => {
                expect_local!(*index, ValueType::Float);
                push_stack!(ValueType::Float);
            }
            Op::DLoad(index) => {
                expect_local!(*index, ValueType::Double);
                if index + 1 >= locals.len() {
                    return Err(VerifyError::WrongCount);
                }

                push_stack!(ValueType::Double);
            }
            Op::ALoad(index) => {
                expect_local!(*index, ValueType::Reference);
                push_stack!(ValueType::Reference);
            }
            Op::IaLoad | Op::BaLoad | Op::CaLoad | Op::SaLoad => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Integer);
            }
            Op::LaLoad => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Long);
            }
            Op::FaLoad => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Float);
            }
            Op::DaLoad => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Double);
            }
            Op::AaLoad => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Reference);
            }
            Op::IStore(index) => {
                expect_pop_stack!(ValueType::Integer);
                set_local!(*index, ValueType::Integer);
            }
            Op::LStore(index) => {
                expect_pop_stack!(ValueType::Long);
                set_local!(*index, ValueType::Long);

                // The docs aren't clear on this, but this is expected
                set_local!(*index + 1, ValueType::Invalid);
            }
            Op::FStore(index) => {
                expect_pop_stack!(ValueType::Float);
                set_local!(*index, ValueType::Float);
            }
            Op::DStore(index) => {
                expect_pop_stack!(ValueType::Double);
                set_local!(*index, ValueType::Double);

                // The docs aren't clear on this, but this is expected
                set_local!(*index + 1, ValueType::Invalid);
            }
            Op::AStore(index) => {
                let stack_value =
                    expect_pop_stack!(ValueType::Reference | ValueType::ReturnAddress(_));
                set_local!(*index, stack_value);
            }
            Op::IaStore | Op::BaStore | Op::CaStore | Op::SaStore => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
            }
            Op::LaStore => {
                expect_pop_stack!(ValueType::Long);
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
            }
            Op::FaStore => {
                expect_pop_stack!(ValueType::Float);
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
            }
            Op::DaStore => {
                expect_pop_stack!(ValueType::Double);
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
            }
            Op::AaStore => {
                expect_pop_stack!(ValueType::Reference);
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Reference);
            }
            Op::Pop => {
                let value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if value.is_wide() {
                    return Err(VerifyError::WrongType);
                }
            }
            Op::Pop2 => {
                let value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if !value.is_wide() {
                    stack.pop().ok_or(VerifyError::WrongCount)?;
                }
            }
            Op::Dup => {
                let value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if value.is_wide() {
                    return Err(VerifyError::WrongType);
                } else {
                    stack.push(value);
                    stack.push(value);

                    if stack.len() > max_stack {
                        return Err(VerifyError::WrongCount);
                    }
                }
            }
            Op::DupX1 => {
                let top_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                let under_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if top_value.is_wide() || under_value.is_wide() {
                    return Err(VerifyError::WrongType);
                }

                stack.push(top_value);
                stack.push(under_value);
                stack.push(top_value);

                if stack.len() > max_stack {
                    return Err(VerifyError::WrongCount);
                }
            }
            Op::DupX2 => {
                let top_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if top_value.is_wide() {
                    return Err(VerifyError::WrongType);
                }

                let under_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if under_value.is_wide() {
                    stack.push(top_value);
                    stack.push(under_value);
                    stack.push(top_value);
                } else {
                    let under_value_2 = stack.pop().ok_or(VerifyError::WrongCount)?;
                    stack.push(top_value);
                    stack.push(under_value_2);
                    stack.push(under_value);
                    stack.push(top_value);
                }

                if stack.len() > max_stack {
                    return Err(VerifyError::WrongCount);
                }
            }
            Op::Dup2 => {
                let value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if value.is_wide() {
                    stack.push(value);
                    stack.push(value);
                } else {
                    let second_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                    if second_value.is_wide() {
                        return Err(VerifyError::WrongType);
                    } else {
                        stack.push(second_value);
                        stack.push(value);
                        stack.push(second_value);
                        stack.push(value);
                    }
                }

                if stack.len() > max_stack {
                    return Err(VerifyError::WrongCount);
                }
            }
            Op::Dup2X2 => {
                let value1 = stack.pop().ok_or(VerifyError::WrongCount)?;
                let value2 = stack.pop().ok_or(VerifyError::WrongCount)?;

                if value1.is_wide() {
                    if value2.is_wide() {
                        // Form 4 in the JVMS
                        stack.push(value1);
                        stack.push(value2);
                        stack.push(value1);
                    } else {
                        let value3 = stack.pop().ok_or(VerifyError::WrongCount)?;
                        if value3.is_wide() {
                            return Err(VerifyError::WrongType);
                        } else {
                            // Form 2 in the JVMS
                            stack.push(value1);
                            stack.push(value3);
                            stack.push(value2);
                            stack.push(value1);
                        }
                    }
                } else {
                    if value2.is_wide() {
                        return Err(VerifyError::WrongType);
                    } else {
                        let value3 = stack.pop().ok_or(VerifyError::WrongCount)?;
                        if value3.is_wide() {
                            // Form 3 in the JVMS
                            stack.push(value2);
                            stack.push(value1);
                            stack.push(value3);
                            stack.push(value2);
                            stack.push(value1);
                        } else {
                            let value4 = stack.pop().ok_or(VerifyError::WrongCount)?;
                            if value4.is_wide() {
                                return Err(VerifyError::WrongType);
                            } else {
                                // Form 1 in the JVMS
                                stack.push(value2);
                                stack.push(value1);
                                stack.push(value4);
                                stack.push(value3);
                                stack.push(value2);
                                stack.push(value1);
                            }
                        }
                    }
                }

                if stack.len() > max_stack {
                    return Err(VerifyError::WrongCount);
                }
            }
            Op::Swap => {
                let first_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if first_value.is_wide() {
                    return Err(VerifyError::WrongType);
                }

                let second_value = stack.pop().ok_or(VerifyError::WrongCount)?;

                if second_value.is_wide() {
                    return Err(VerifyError::WrongType);
                }

                stack.push(first_value);
                stack.push(second_value);
            }
            Op::IAdd | Op::ISub | Op::IMul | Op::IDiv | Op::IRem => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Integer);
            }
            Op::LAdd | Op::LSub | Op::LMul | Op::LDiv | Op::LRem => {
                expect_pop_stack!(ValueType::Long);
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Long);
            }
            Op::FAdd | Op::FSub | Op::FMul | Op::FDiv | Op::FRem => {
                expect_pop_stack!(ValueType::Float);
                expect_pop_stack!(ValueType::Float);
                push_stack!(ValueType::Float);
            }
            Op::DAdd | Op::DSub | Op::DMul | Op::DDiv | Op::DRem => {
                expect_pop_stack!(ValueType::Double);
                expect_pop_stack!(ValueType::Double);
                push_stack!(ValueType::Double);
            }
            Op::INeg => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Integer);
            }
            Op::LNeg => {
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Long);
            }
            Op::FNeg => {
                expect_pop_stack!(ValueType::Float);
                push_stack!(ValueType::Float);
            }
            Op::DNeg => {
                expect_pop_stack!(ValueType::Double);
                push_stack!(ValueType::Double);
            }
            Op::IShl | Op::IShr | Op::IUshr => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Integer);
            }
            Op::LShl | Op::LShr | Op::LUshr => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Long);
            }
            Op::IAnd | Op::IOr | Op::IXor => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Integer);
            }
            Op::LAnd | Op::LOr | Op::LXor => {
                expect_pop_stack!(ValueType::Long);
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Long);
            }
            Op::IInc(index, _) => {
                expect_local!(*index, ValueType::Integer);
            }
            Op::I2L => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Long);
            }
            Op::I2F => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Float);
            }
            Op::I2D => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Double);
            }
            Op::L2I => {
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Integer);
            }
            Op::L2F => {
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Float);
            }
            Op::L2D => {
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Double);
            }
            Op::F2I => {
                expect_pop_stack!(ValueType::Float);
                push_stack!(ValueType::Integer);
            }
            Op::F2L => {
                expect_pop_stack!(ValueType::Float);
                push_stack!(ValueType::Long);
            }
            Op::F2D => {
                expect_pop_stack!(ValueType::Float);
                push_stack!(ValueType::Double);
            }
            Op::D2I => {
                expect_pop_stack!(ValueType::Double);
                push_stack!(ValueType::Integer);
            }
            Op::D2L => {
                expect_pop_stack!(ValueType::Double);
                push_stack!(ValueType::Long);
            }
            Op::D2F => {
                expect_pop_stack!(ValueType::Double);
                push_stack!(ValueType::Float);
            }
            Op::I2B | Op::I2C | Op::I2S => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Integer);
            }
            Op::LCmp => {
                expect_pop_stack!(ValueType::Long);
                expect_pop_stack!(ValueType::Long);
                push_stack!(ValueType::Integer);
            }
            Op::FCmpL | Op::FCmpG => {
                expect_pop_stack!(ValueType::Float);
                expect_pop_stack!(ValueType::Float);
                push_stack!(ValueType::Integer);
            }
            Op::DCmpL | Op::DCmpG => {
                expect_pop_stack!(ValueType::Double);
                expect_pop_stack!(ValueType::Double);
                push_stack!(ValueType::Integer);
            }
            Op::IfEq(_) | Op::IfNe(_) | Op::IfLt(_) | Op::IfGe(_) | Op::IfGt(_) | Op::IfLe(_) => {
                expect_pop_stack!(ValueType::Integer);
            }
            Op::IfICmpEq(_)
            | Op::IfICmpNe(_)
            | Op::IfICmpLt(_)
            | Op::IfICmpGe(_)
            | Op::IfICmpGt(_)
            | Op::IfICmpLe(_) => {
                expect_pop_stack!(ValueType::Integer);
                expect_pop_stack!(ValueType::Integer);
            }
            Op::IfACmpEq(_) | Op::IfACmpNe(_) => {
                expect_pop_stack!(ValueType::Reference);
                expect_pop_stack!(ValueType::Reference);
            }
            Op::Goto(_) => {
                // This does nothing
            }
            Op::Jsr(_) => {
                // This is the index of the op right after this one
                let op_index = block.start_index + intra_block_index + 1;

                push_stack!(ValueType::ReturnAddress(op_index));
            }
            Op::Ret(index) => {
                let value = locals.get(*index).ok_or(VerifyError::WrongCount)?;

                match value {
                    ValueType::ReturnAddress(position) => {
                        // Convert the position to a block and push to the
                        // worklist here, manually.
                        let returned_block_idx = *op_index_to_block_index_table
                            .get(position)
                            .expect("Return address should map to valid block index");
                        worklist.push((returned_block_idx, frame_state));

                        // This must be the last op in this block, so we're
                        // finished verifying the block
                        return Ok(());
                    }
                    _ => return Err(VerifyError::WrongType),
                }
            }
            Op::TableSwitch(_) => {
                expect_pop_stack!(ValueType::Integer);
            }
            Op::LookupSwitch(_) => {
                expect_pop_stack!(ValueType::Integer);
            }
            Op::IReturn => {
                expect_pop_stack!(ValueType::Integer);
            }
            Op::LReturn => {
                expect_pop_stack!(ValueType::Long);
            }
            Op::FReturn => {
                expect_pop_stack!(ValueType::Float);
            }
            Op::DReturn => {
                expect_pop_stack!(ValueType::Double);
            }
            Op::AReturn => {
                expect_pop_stack!(ValueType::Reference);
            }
            Op::Return => {
                // This does nothing
            }
            Op::GetStatic(class, index) | Op::GetStaticWide(class, index) => {
                let field_descriptor = class.get_static_field(*index).descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        push_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        push_stack!(ValueType::Long);
                    }
                    Descriptor::Void => unreachable!(),
                }
            }
            Op::PutStatic(class, index) | Op::PutStaticWide(class, index) => {
                let field_descriptor = class.get_static_field(*index).descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        expect_pop_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        expect_pop_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        expect_pop_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        expect_pop_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        expect_pop_stack!(ValueType::Long);
                    }
                    Descriptor::Void => unreachable!(),
                }
            }
            Op::GetField(class, index) | Op::GetFieldWide(class, index) => {
                expect_pop_stack!(ValueType::Reference);

                let field_descriptor = class.get_instance_field(*index).descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        push_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        push_stack!(ValueType::Long);
                    }
                    Descriptor::Void => unreachable!(),
                }
            }
            Op::PutField(class, index) | Op::PutFieldWide(class, index) => {
                let field_descriptor = class.get_instance_field(*index).descriptor();
                match field_descriptor {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        expect_pop_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        expect_pop_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        expect_pop_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        expect_pop_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        expect_pop_stack!(ValueType::Long);
                    }
                    Descriptor::Void => unreachable!(),
                }

                expect_pop_stack!(ValueType::Reference);
            }
            Op::InvokeVirtual(class, method_index, _)
            | Op::InvokeVirtualWide(class, method_index, _) => {
                let descriptor = class
                    .instance_method_vtable()
                    .get_element(*method_index)
                    .descriptor();

                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(ValueType::Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(ValueType::Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(ValueType::Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(ValueType::Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(ValueType::Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                // Receiver
                expect_pop_stack!(ValueType::Reference);

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        push_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        push_stack!(ValueType::Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::InvokeSpecial(method) => {
                let descriptor = method.descriptor();
                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(ValueType::Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(ValueType::Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(ValueType::Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(ValueType::Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(ValueType::Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                // Receiver
                expect_pop_stack!(ValueType::Reference);

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        push_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        push_stack!(ValueType::Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::InvokeStatic(method) => {
                let descriptor = method.descriptor();
                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(ValueType::Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(ValueType::Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(ValueType::Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(ValueType::Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(ValueType::Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        push_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        push_stack!(ValueType::Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::InvokeInterface(invoke_interface) => {
                let descriptor = invoke_interface.descriptor;

                for arg in descriptor.args().iter().rev() {
                    match arg {
                        Descriptor::Class(_) | Descriptor::Array(_) => {
                            expect_pop_stack!(ValueType::Reference);
                        }
                        Descriptor::Boolean
                        | Descriptor::Byte
                        | Descriptor::Character
                        | Descriptor::Short
                        | Descriptor::Integer => {
                            expect_pop_stack!(ValueType::Integer);
                        }
                        Descriptor::Float => {
                            expect_pop_stack!(ValueType::Float);
                        }
                        Descriptor::Double => {
                            expect_pop_stack!(ValueType::Double);
                        }
                        Descriptor::Long => {
                            expect_pop_stack!(ValueType::Long);
                        }
                        Descriptor::Void => unreachable!(),
                    }
                }

                // Receiver
                expect_pop_stack!(ValueType::Reference);

                match descriptor.return_type() {
                    Descriptor::Class(_) | Descriptor::Array(_) => {
                        push_stack!(ValueType::Reference);
                    }
                    Descriptor::Boolean
                    | Descriptor::Byte
                    | Descriptor::Character
                    | Descriptor::Short
                    | Descriptor::Integer => {
                        push_stack!(ValueType::Integer);
                    }
                    Descriptor::Float => {
                        push_stack!(ValueType::Float);
                    }
                    Descriptor::Double => {
                        push_stack!(ValueType::Double);
                    }
                    Descriptor::Long => {
                        push_stack!(ValueType::Long);
                    }
                    Descriptor::Void => {}
                }
            }
            Op::New(_) => {
                push_stack!(ValueType::Reference);
            }
            Op::NewArray(_) => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Reference);
            }
            Op::ANewArray(_) => {
                expect_pop_stack!(ValueType::Integer);
                push_stack!(ValueType::Reference);
            }
            Op::ArrayLength => {
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Integer);
            }
            Op::AThrow => {
                expect_pop_stack!(ValueType::Reference);
            }
            Op::CheckCast(_) => {
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Reference);
            }
            Op::InstanceOf(_) => {
                expect_pop_stack!(ValueType::Reference);
                push_stack!(ValueType::Integer);
            }
            Op::MonitorEnter => {
                expect_pop_stack!(ValueType::Reference);
            }
            Op::MonitorExit => {
                expect_pop_stack!(ValueType::Reference);
            }
            Op::MultiANewArray(multi_a_new_array) => {
                for _ in 0..multi_a_new_array.dimensions {
                    expect_pop_stack!(ValueType::Integer);
                }

                push_stack!(ValueType::Reference);
            }
            Op::IfNull(_) => {
                expect_pop_stack!(ValueType::Reference);
            }
            Op::IfNonNull(_) => {
                expect_pop_stack!(ValueType::Reference);
            }

            Op::Clinit(_) => {
                // Doesn't modify stack
            }
            Op::GcCheck => {
                // Doesn't modify stack
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
        BlockExits::SubroutineReturn => {
            // The Ret op should have manually added the correct index
            // to the worklist.
        }
        BlockExits::Return => {}
    }

    Ok(())
}

pub enum VerifyError {
    CodeFellOffMethod,
    WrongCount,
    WrongType,
}

impl VerifyError {
    pub fn to_error(self, context: &Context) -> Error {
        let message = match self {
            VerifyError::CodeFellOffMethod => "Code fell off method",
            VerifyError::WrongCount => "Wrong count",
            VerifyError::WrongType => "Wrong type",
        };

        context.verify_error(message)
    }
}
