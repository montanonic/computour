//! Credit to https://justinmeiners.github.io/lc3-vm/ for guidance/inspiration.

mod instructions;

use instructions::Instruction;
use std::mem;

pub fn main() {
    println!("{}", (((0b101u16 << (16 - 3)) as i16) >> (16 - 3)) as i16);
    println!("{}", -0b0000_0000_0000_0011i16);
    println!("{:016b}", -0b0000_0000_0000_0011i16);
    println!("{}", 0b11111111111111111111111111111111u32 == u32::MAX);
}

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
    test_program_: Option<Vec<Instruction>>,
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

    pub fn new_test(test_program: Vec<Instruction>) -> Self {
        let mut this = Self::new();
        this.test_program_ = Some(test_program);
        this
    }

    pub fn run_once(&mut self) {
        match self.next_instruction() {
            Instruction::AddReg(s1, s2, d) => {
                self.registers[d] = self.registers[s1] + self.registers[s2];
            }
            Instruction::AndImm(s, d, imm) => {
                self.registers[d] = self.registers[s] + imm;
            }
            // Instruction::Ld(d, offset) => {}
            _ => panic!("instruction not handled"),
        }
    }

    fn next_instruction(&mut self) -> Instruction {
        match &mut self.test_program_ {
            None => todo!(),
            Some(program) => {
                let res = program[self.ip];
                self.ip += 1;
                res
            }
        }
    }
}

// /// A builder-style structure that allows us to simply build an LC-3 program in
// /// Rust code.
// struct TestProgram(Vec<Instruction>);

// impl TestProgram {
//     pub fn new() -> Self {
//         Self(Vec::new())
//     }

//     pub fn add()
// }

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

    #[test]
    fn add_works() {
        /// add r6 r2 r6
        let test_program = vec![AddReg(0, 1, 1), AndImm(1, 0, 13)];
        let mut lc3 = LC3::new_test(test_program);
        lc3.registers[0] = 15;
        lc3.registers[1] = 5;
        lc3.run_once();
        assert_eq!(lc3.registers[0], 15);
        assert_eq!(lc3.registers[1], 20);
        lc3.run_once();
        assert_eq!(lc3.registers[0], 33);
        assert_eq!(lc3.registers[1], 20);
    }
}
