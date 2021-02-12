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
    /// A register is a slot for storing a single value on the CPU. Registers are
    /// like the "workbench" of the CPU. For the CPU to work with a piece of data,
    /// it has to be in one of the registers. However, since there are just a few
    /// registers, only a minimal amount of data can be loaded at any given time.
    /// Programs work around this by loading values from memory into registers,
    /// calculating values into other registers, and then storing the final results
    /// back in memory.
    registers: [u16; 8],
    /// ip is short for instruction pointer. The instruction pointer tracks the
    /// location of the processor's next instruction.
    ip: usize,
    /// Used as an alternate to running a program on memory, for easier testing.
    test_program_: Option<TestProgram>,
}

impl LC3 {
    pub fn new() -> Self {
        Self {
            memory: [0; u16::MAX as usize],
            registers: [0; 8],
            ip: 0,
            test_program_: None,
        }
    }

    pub fn new_test(test_program: Vec<u16>) -> Self {
        let mut this = Self::new();
        this.test_program_ = Some(test_program);
        this
    }

    fn next_instruction(&mut self) {
        self.test_program_.map_or_else(|| todo!(), |program| {});
    }
}

/// A builder-style structure that allows us to simply build a program
struct TestProgram {}

/// Force a value to the specified number of bits by truncating. Does not make
/// the value any smaller though.
trait ForceBits {
    fn force_bits(self, bits: u8) -> Self;
}

impl ForceBits for u8 {
    fn force_bits(self, bits: u8) -> Self {
        debug_assert!(bits < 8);
        let mask = u8::MAX >> (7 - bits);
        self & mask
    }
}

/// For ease of testing.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Add(u16),
    AddIm(u16),
}

/// Constructors use the following abbreviations:
///
/// Note that all of these may have numbers affixed to distinguish them.
///
/// sr = "source register"
///
/// dr = "destination register"
impl Instruction {
    /// 0 0 0 1 | sr = 3b | dr = 3b | 1 | imm = 5b
    pub fn add_im(sr: u8, dr: u8, imm: u8) -> Self {
        let mut res: u16 = 0;
        res = imm.force_bits(5) as u16;
        res |= 1 << 5;
        res |= (sr.force_bits(3) as u16) << 6;
        res |= (dr.force_bits(3) as u16) << 9;
        res |= 0b0001 << 12;
        Self::AddIm(res)
    }

    /// 0 0 0 1 | sr1 = 3b | sr2 = 3b | 000 | dr = 3b
    pub fn add(sr1: u8, sr2: u8, dr: u8) -> Self {
        let mut res: u16 = 0;
        res = dr.force_bits(3) as u16;
        res |= (sr2.force_bits(3) as u16) << 6;
        res |= (sr1.force_bits(3) as u16) << 9;
        res |= 0b0001 << 12;
        Self::Add(res)
    }
}

impl From<Instruction> for u16 {
    fn from(val: Instruction) -> Self {
        use Instruction::*;
        match val {
            AddIm(x) => x,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_builds_correctly() {
        let instr = Instruction::add(0, 1, 0);
        assert_eq!(Instruction::Add(0b0001_000_001_0_00_000), instr);
    }

    // #[test]
    // fn add_works() {
    //     /// add r6 r2 r6
    //     let test_program = vec![0b0001_000_001_0_00_001];
    //     let mut lc3 = LC3::new_test(test_program);
    //     lc3.registers[0] = 15;
    //     lc3.run_once();
    //     assert_eq!(lc3.registers[1], 20);
    // }
}
