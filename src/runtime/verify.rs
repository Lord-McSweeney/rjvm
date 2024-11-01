use super::context::Context;
use super::error::{Error, NativeError};
use super::method::{Exception, Method};
use super::op::Op;

use std::collections::{HashMap, HashSet};

enum ValueType {
    Int,
    Long,
    Float,
    Double,
    Reference,
}

// The possible ways a block can be exited.
#[derive(Debug)]
enum BlockExits {
    // Such as for LookupSwitch. When execution is finished, the next block is
    // any of the blocks with its first op any of the given indices.
    BranchMultiple(Vec<usize>),

    // Such as for exceptions. When execution is finished, the next block is any
    // of the blocks with its first op any of the given indices or the next
    // block in the block list, and a Reference will be pushed on the abstract
    // interpreter's stack.
    BranchMultipleException(Vec<usize>),

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
    ops: &'a [Op],
    exceptions: &[Exception],
) -> Result<(), Error> {
    let blocks = collect_basic_blocks(ops, exceptions)?;

    // TODO: Abstract interpreter
    validate_blocks(context, method, blocks)?;

    Ok(())
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
                    exits: BlockExits::BranchMultiple(possible_target_list),
                };

                block_list.push(block);

                current_block_start = i + 1;
            }
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
                        exits: BlockExits::BranchMultipleException(exception_branches),
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

fn validate_blocks<'a>(
    context: Context,
    method: Method,
    blocks: Vec<BasicBlock<'a>>,
) -> Result<(), Error> {
    Ok(())
}
