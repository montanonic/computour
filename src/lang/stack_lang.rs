//! I had some ideas for how to implement this so wanted to start from a clean slate.

use std::{
    mem,
    ops::{Deref, DerefMut},
};

use strum_macros::EnumString;

// #[derive(Debug, PartialEq, EnumString)]
// enum Instruction {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }

/// A (basic) block of instructions.
struct Block<T>(Vec<T>);

impl<T> Block<T> {
    fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Deref for Block<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Block<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

trait Instruction {
    fn is_control(&self) -> bool;
}

fn blocks_from_instructions<I: Clone + Instruction>(instructions: &[I]) -> Vec<Block<I>> {
    let mut res = Vec::new();
    let mut block = Block::new();
    for instr in instructions.iter().cloned() {
        let is_control = instr.is_control();
        block.push(instr);
        if is_control {
            res.push(mem::replace(&mut block, Block::new()));
        }
    }
    res
}
