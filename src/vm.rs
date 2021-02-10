mod instruction;

use instruction::{Instruction, Opcode};

pub fn main() {}

pub struct VM {
    registers: [i32; 32],
    /// Contains the binary code read by the VM.
    program: Vec<u8>,
    /// Program Counter. Tracks which byte is executing.
    pc: usize,
}

/// Panics on programs that do not halt.
impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            program: vec![],
            pc: 0,
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
                self.registers[register] = number as i32;
            }
        }
        true
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
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
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
