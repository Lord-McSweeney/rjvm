use super::context::Context;
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
    context: Context,
    method: Method,
    max_stack: usize,
    max_locals: usize,
    ops: &'a [Op],
    exceptions: &[Exception],
) -> Result<(), Error> {
    let blocks = collect_basic_blocks(ops, exceptions)?;

    verify_blocks(context, method, max_stack, max_locals, blocks)
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
                | Op::IfLe(position)
                | Op::IfICmpNe(position)
                | Op::IfICmpGe(position)
                | Op::IfICmpGt(position)
                | Op::IfICmpLe(position)
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
                Op::AThrow | Op::IReturn | Op::AReturn | Op::Return => {
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
            | Op::IfLe(position)
            | Op::IfICmpNe(position)
            | Op::IfICmpGe(position)
            | Op::IfICmpGt(position)
            | Op::IfICmpLe(position)
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
            Op::IReturn | Op::AReturn | Op::Return => {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FrameState {
    locals: Box<[ValueType]>,
    stack: Vec<ValueType>,
}

fn verify_blocks<'a>(
    context: Context,
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
                context,
                max_stack,
                max_locals,
                block,
                block_idx,
                initial_frame_state,
                &mut worklist,
            )?;
        }
    }

    Ok(())
}

fn verify_block<'a>(
    context: Context,
    max_stack: usize,
    max_locals: usize,
    block: &BasicBlock<'a>,
    block_idx: usize,
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
            Op::AConstNull => {
                push_stack!(Reference);
            }
            Op::IConst(_) => {
                push_stack!(Integer);
            }
            Op::Ldc(constant_pool_entry) => match constant_pool_entry {
                ConstantPoolEntry::String { .. } => {
                    push_stack!(Reference);
                }
                ConstantPoolEntry::Integer { .. } => {
                    push_stack!(Integer);
                }
                _ => unimplemented!(),
            },
            Op::ILoad(index) => {
                expect_local!(*index, Integer);
                push_stack!(Integer);
            }
            Op::ALoad(index) => {
                expect_local!(*index, Reference);
                push_stack!(Reference);
            }
            Op::IaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Integer);
            }
            Op::AaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Reference);
            }
            Op::BaLoad => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
                push_stack!(Integer);
            }
            Op::IStore(index) => {
                expect_pop_stack!(Integer);
                set_local!(*index, Integer);
            }
            Op::AStore(index) => {
                expect_pop_stack!(Reference);
                set_local!(*index, Reference);
            }
            Op::CaStore => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
                expect_pop_stack!(Reference);
            }
            Op::Dup => {
                let value = stack
                    .pop()
                    .ok_or(Error::Native(NativeError::VerifyCountWrong))?;
                stack.push(value);
                stack.push(value);
                if stack.len() > max_stack {
                    return Err(Error::Native(NativeError::VerifyCountWrong));
                }
            }
            Op::IAdd | Op::ISub | Op::IDiv | Op::IRem => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::INeg => {
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::IInc(index, _) => {
                expect_local!(*index, Integer);
            }
            Op::I2C => {
                expect_pop_stack!(Integer);
                push_stack!(Integer);
            }
            Op::IfEq(_) | Op::IfNe(_) | Op::IfLt(_) | Op::IfGe(_) | Op::IfLe(_) => {
                expect_pop_stack!(Integer);
            }
            Op::IfICmpNe(_) | Op::IfICmpGe(_) | Op::IfICmpGt(_) | Op::IfICmpLe(_) => {
                expect_pop_stack!(Integer);
                expect_pop_stack!(Integer);
            }
            Op::Goto(_) => {
                // This does nothing
            }
            Op::LookupSwitch(_, _) => {
                expect_pop_stack!(Integer);
            }
            Op::IReturn => {
                expect_pop_stack!(Integer);
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
            // TODO: Verify that overridden methods match their subclass descriptors
            Op::InvokeVirtual((_, descriptor)) => {
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
            Op::New(_) => {
                push_stack!(Reference);
            }
            Op::NewArray(_) => {
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
