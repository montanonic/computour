pub enum Instruction {
    Add {
        s1: Register,
        s2: Register,
        d: Register,
    },
    AddIm {
        s1: Register,
        s2: Register,
        ///
        num: u8,
    },
}

/// Safe wrapper for ensuring in-bounds registers.
pub struct Register(u8);

impl Register {
    pub fn new(r: u8) -> Option<Self> {
        if r < 8 {
            Some(Self(r))
        } else {
            None
        }
    }
}
