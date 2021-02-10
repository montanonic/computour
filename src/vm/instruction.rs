#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    /// An absolute jump of the program counter to the value specified in
    /// register.
    JMP,
    JMPF,
    JMPB,
    IGL,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        use Opcode::*;
        match v {
            0 => HLT,
            1 => LOAD,
            2 => ADD,
            3 => SUB,
            4 => MUL,
            5 => DIV,
            6 => JMP,
            7 => JMPF,
            8 => JMPB,
            _ => IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
