//! Via https://github.com/erfur/lc3-vm-rust/blob/master/src/instruction.rs

pub type Offset = u16;
pub type Imm = u16;
pub type TrapVector = u8;
pub type Flag = bool;
pub type Register = usize;

#[derive(Debug)]
pub enum Instruction {
    AddReg(Register, Register, Register),
    AddImm(Register, Register, Imm),
    AndReg(Register, Register, Register),
    AndImm(Register, Register, Imm),
    Br(Flag, Flag, Flag, Offset),
    Jmp(Register),
    Jsr(Offset),
    Jsrr(Register),
    Ld(Register, Offset),
    Ldi(Register, Offset),
    Ldr(Register, Register, Offset),
    Lea(Register, Offset),
    Not(Register, Register),
    Rti(),
    St(Register, Offset),
    Sti(Register, Offset),
    Str(Register, Register, Offset),
    Trap(TrapVector),
    Reserved(),
}
