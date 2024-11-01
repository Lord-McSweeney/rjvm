use super::error::{Error, NativeError};
use super::method::Exception;
use super::op::Op;

use std::collections::{HashMap, HashSet};

enum ValueType {
    Int,
    Long,
    Float,
    Double,
    Reference,
}

#[derive(Debug)]
enum BlockExits {
    // Such as for LookupSwitch. When execution is finished, the next block is
    // any of the blocks with its first op any of the given indices.
    BranchMultiple(Vec<usize>),

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

pub fn verify_ops<'a>(ops: &'a [Op], exceptions: &[Exception]) -> Result<(), Error> {
    let blocks = collect_basic_blocks(ops, exceptions)?;

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

    // Create a table mapping op indices to block indicies.
    let mut op_index_to_block_index_table = HashMap::new();
    for (i, block) in block_list.iter().enumerate() {
        op_index_to_block_index_table.insert(block.start_index, i);
    }

    // Now convert the op indices mentioned in BlockExits to BB indices.
    for mut block in block_list.iter_mut() {
        match block.exits {
            BlockExits::BranchMultiple(ref mut branches) => {
                for branch in branches {
                    *branch = *op_index_to_block_index_table
                        .get(branch)
                        .expect("Op indices should map to block indices");
                }
            }
            BlockExits::Branch(ref mut branch) | BlockExits::Goto(ref mut branch) => {
                *branch = *op_index_to_block_index_table
                    .get(branch)
                    .expect("Op indices should map to block indices");
            }
            _ => {}
        }
    }

    // TODO: Abstract interpreter

    Ok(block_list)
}
