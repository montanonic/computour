//! Credit to https://justinmeiners.github.io/lc3-vm/ for guidance/inspiration.

mod instructions;

use std::mem;

pub fn main() {}

type Memory = [u16; u16::MAX as usize];

struct LC3 {
    /// The LC-3 has 65,536 memory locations (the maximum that is addressable by
    /// a 16-bit unsigned integer 2^16), each of which stores a 16-bit value.
    /// This means it can store a total of only 128kb, which is a lot smaller
    /// than you may be used to!
    memory: Memory,
}

/// A register is a slot for storing a single value on the CPU. Registers are
/// like the "workbench" of the CPU. For the CPU to work with a piece of data,
/// it has to be in one of the registers. However, since there are just a few
/// registers, only a minimal amount of data can be loaded at any given time.
/// Programs work around this by loading values from memory into registers,
/// calculating values into other registers, and then storing the final results
/// back in memory.
struct Registers {
    general: [u16; 8],
    counter: u16,
    flag: u16,
}

enum Register {
    R(u8),
    ProgramCounter,
    ConditionFlag,
}

enum ConditionFlag {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

enum Op {
    Branch,
    Add,
    Load,
    Store,
    And,
    Not,
    Jump,
    JumpRegister,
    LoadRegister,
    StoreRegister,
    LoadIndirect,
    StoreIndirect,
    LoadEffectiveAddress,
    ExecuteTrap,
}
