//! Credit to https://justinmeiners.github.io/lc3-vm/ for guidance/inspiration.

mod instructions;

use instructions::Instruction;
use std::mem;

pub fn main() {
    println!(
        "{}",
        2f64.powi(-2) + 2f64.powi(-3) + 2f64.powi(-5) + 2f64.powi(-6)
    );
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
        let Self {
            mut registers,
            mut memory,
            mut ip,
            ..
        } = self;

        match self.next_instruction() {
            Instruction::AddReg(dr, sr1, sr2) => {
                registers[dr] = registers[sr1] + registers[sr2];
            }
            Instruction::AddImm(dr, sr1, imm) => {
                registers[dr] = registers[sr1] + sign_extend(imm, 5);
            }
            Instruction::AndReg(dr, sr1, sr2) => {
                registers[dr] = registers[sr1] & registers[sr2];
            }
            Instruction::AndImm(dr, sr1, imm) => {
                registers[dr] = registers[sr1] & sign_extend(imm, 5);
            }
            Instruction::Not(dr, sr1) => {
                registers[dr] = !registers[sr1];
            }
            Instruction::Br(n, z, p, offset9) => {}
            Instruction::Jmp(base) => {
                memory[registers[base] as usize];
            }
            Instruction::Jsr(offset11) => {}
            Instruction::Jsrr(base) => {}
            Instruction::Ld(dr, offset9) => {
                registers[dr] = memory[ip.wrapping_add(sign_extend(offset9, 9) as usize)]
            }
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

#[inline]
fn sign_extend(x: u16, bits: u8) -> u16 {
    let shift = 16 - bits;
    (((x << shift) as i16) >> shift) as u16
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
        let test_program = vec![AddReg(1, 0, 1), AddImm(0, 1, 13)];
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
