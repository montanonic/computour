mod instruction;

use instruction::{Instruction, Opcode};

pub fn main() {}

#[derive(Debug)]
pub struct VM {
    /// Array that simulates having hardware registers.
    registers: [i32; 32],
    /// Contains the binary code (bytecode) read by the VM.
    program: Vec<u8>,
    /// Program Counter. Tracks which byte is executing.
    pc: usize,
    /// Contains the remainder of modulo division ops
    remainder: u32,
    /// Contains the result of the last equality comparison operation
    equal_flag: bool,
}

/// Panics on programs that do not halt.
impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        /// Continue running our VM until an instruction tells us to stop.
        while self.execute_instruction() {}
    }

    /// Executes a single instruction, for more controlled execution of the VM.
    fn execute_instruction(&mut self) -> bool {
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next_byte() as usize;
                let number = self.next_u16();
                self.register_store(register, number);
            }
            Opcode::ADD => {
                let n1 = self.next_byte_as_register_lookup();
                let n2 = self.next_byte_as_register_lookup();
                self.next_byte_as_register_store(n1 + n2);
            }
            Opcode::SUB => {
                let n1 = self.next_byte_as_register_lookup();
                let n2 = self.next_byte_as_register_lookup();
                self.next_byte_as_register_store(n1 - n2);
            }
            Opcode::MUL => {
                let n1 = self.next_byte_as_register_lookup();
                let n2 = self.next_byte_as_register_lookup();
                self.next_byte_as_register_store(n1 * n2);
            }
            Opcode::DIV => {
                let n1 = self.next_byte_as_register_lookup();
                let n2 = self.next_byte_as_register_lookup();
                self.next_byte_as_register_store(n1 / n2);
                self.remainder = (n1 % n2) as u32;
            }
            Opcode::JMP => {
                self.pc = self.next_byte_as_register_lookup() as usize;
            }
            Opcode::JMPF => {
                self.pc += self.next_byte() as usize;
            }
            Opcode::JMPB => {
                self.pc -= self.next_byte() as usize;
            }
            Opcode::EQ => {
                // Flag if the values at the bytecode specified registers are equal.
                self.equal_flag =
                    self.next_byte_as_register_lookup() == self.next_byte_as_register_lookup();
                // Move pc out of the last instruction byte so that the next
                // instruction can be processed.
                self.next_byte();
            }
            Opcode::JEQ => {
                if self.equal_flag {
                    self.pc = self.next_byte_as_register_lookup() as usize;
                } else {
                    self.pc += 3;
                }
            }
        }
        true
    }

    /// Reads the next byte as a register, and stores the given value at that
    /// register.
    fn next_byte_as_register_store(&mut self, val: impl Into<i32>) {
        let register = self.next_byte();
        self.register_store(register, val);
    }

    /// Stores the value at the given register.
    fn register_store(&mut self, register: impl Into<usize>, val: impl Into<i32>) {
        self.registers[register.into()] = val.into();
    }

    /// Reads the next byte as a register, and looks up the value at that
    /// register.
    fn next_byte_as_register_lookup(&mut self) -> i32 {
        self.registers[self.next_byte() as usize]
    }

    /// Reads a byte at the current counter, advancing the program counter.
    /// Panics if program counter exceeds the length of our program (which
    /// indicates an invalid program).
    fn next_byte(&mut self) -> u8 {
        let ret = self.program[self.pc]; // This may panic.
        self.pc += 1;
        ret
    }

    fn next_u16(&mut self) -> u16 {
        let (upper, lower) = (self.next_byte(), self.next_byte());
        ((upper as u16) << 8) | (lower as u16)
    }

    fn decode_opcode(&mut self) -> Opcode {
        Opcode::from(self.next_byte())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut vm = VM::new();
        vm.program = vec![Opcode::LOAD as u8, 10, 1, 244];
        vm.execute_instruction();
        assert_eq!(vm.registers[10], 500);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.registers[0] = 99;
        vm.program = vec![Opcode::JMP as u8, 0, 0, 0];
        vm.execute_instruction();
        assert_eq!(vm.pc, 99);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut vm = VM::new();
        vm.program = vec![Opcode::JMPF as u8, 10, 0, 0];
        vm.execute_instruction();
        assert_eq!(vm.pc, 2 + 10);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut vm = VM::new();
        // The 2 moves us up to JMPB, then the 6 moves us back to the start.
        vm.program = vec![Opcode::JMPF as u8, 2, 0, 0, Opcode::JMPB as u8, 6, 0, 0];
        vm.execute_instruction();
        vm.execute_instruction();
        assert_eq!(vm.pc, 0);
        // This means we should be able to loop.
        vm.execute_instruction();
        vm.execute_instruction();
        assert_eq!(vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut vm = VM::new();
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.program = vec![Opcode::EQ as u8, 0, 1, 0, Opcode::EQ as u8, 0, 1, 0];
        vm.execute_instruction();
        assert_eq!(vm.equal_flag, true);
        vm.registers[1] = 20;
        vm.execute_instruction();
        assert_eq!(vm.equal_flag, false);
    }

    #[test]
    fn test_u8_to_u16_conversion() {
        let mut vm = VM::new();
        vm.program = vec![0, u8::MAX];
        assert_eq!(vm.next_u16(), u8::MAX as u16);

        vm.program.extend_from_slice(&[2, 0]);
        assert_eq!(vm.next_u16(), (u8::MAX as u16 + 1) * 2);

        vm.program.extend_from_slice(&[u8::MAX, u8::MAX]);
        assert_eq!(vm.next_u16(), u16::MAX);
    }
}
