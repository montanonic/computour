use std::{
    convert::TryFrom,
    fmt::{self, Debug, Formatter},
    mem,
    ops::{Deref, DerefMut, Shl},
};

/// Eventually I can do a more optimal implementation of Bits. Maybe I'll get
/// inspiration from my CPU VMs on how to make it more optimal! But I should
/// also consult Lib.rs for implementations like BitSet. For now, we're backed
/// by a simple u8 enum that wastes a lot of space, but makes this struct easy
/// to work with :).
#[derive(PartialEq, Eq)]
pub struct Bits(Vec<Bit>);

impl Bits {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Consult this for an implementation reference to other `push_uX` methods.
    pub fn push_u8(&mut self, byte: u8) {
        unsafe {
            // The first bit should be the most significant (that's how we read binary).
            for n in (0u8..8).rev() {
                // Safe because & 0b0000_0001 will always return either 0u8 or
                // 1u8, which covers all possible variants of our Bit enum. See
                // corresponding test.
                let bit: Bit = mem::transmute((byte >> n) & 1u8);
                self.push(bit);
            }
        }
    }

    /// Pops off the most recent 8 bits as a u8 value (the most recent bit being
    /// interpreted as the least significant). If there aren't at least 8 bits
    /// left, returns None and does nothing.
    ///
    /// Consult this for an implementation reference to other `pop_uX` methods.
    pub fn pop_u8(&mut self) -> Option<u8> {
        let len = self.len();
        let slice = &self[len.saturating_sub(8)..len];
        if slice.len() == 8 {
            let mut byte = 0u8;
            // We want the most significant digit to be last so that it gets
            // left-shifted into the most significant location corresponding
            // with `i` being the largest.
            for (i, b) in slice.iter().rev().enumerate() {
                byte |= (*b as u8) << i
            }

            // Drop these bits from our structure.
            self.resize(len - 8, Bit::O);

            Some(byte)
        } else {
            None
        }
    }

    pub fn push_u16(&mut self, val: u16) {
        unsafe {
            for n in (0u8..16).rev() {
                let bit: Bit = mem::transmute(((val >> n) & 1u16) as u8);
                self.push(bit);
            }
        }
    }

    pub fn push_u32(&mut self, val: u32) {
        unsafe {
            for n in (0u8..32).rev() {
                let bit: Bit = mem::transmute(((val >> n) & 1u32) as u8);
                self.push(bit);
            }
        }
    }
}

impl Deref for Bits {
    type Target = Vec<Bit>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Display Bits as a Vector of 1's and 0's.
impl Debug for Bits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|x| *x as u8))
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    /// Zero
    O = 0,
    /// One
    I = 1,
}

impl TryFrom<u8> for Bit {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bit::I),
            1 => Ok(Bit::I),
            _ => Err("value must be either 0 or 1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bit::*;
    use super::*;

    #[test]
    /// In the context of being &-masked to get the least significant bit.
    fn bits_can_safely_transmute_for_0_and_1() {
        assert_eq!(Bit::O as u8, 0);
        assert_eq!(Bit::I as u8, 1);

        // Sanity-check that all u8 values when &-ed with 1u8 result in either 0
        // or 1.
        for n in (u8::MIN..=u8::MAX) {
            assert!((n & 1u8) <= 1);
        }
    }

    #[test]
    fn test_push_u8() {
        let mut bits = Bits::new();
        bits.push_u8(3);
        assert_eq!(&bits.0, &[O, O, O, O, O, O, I, I]);
        bits.clear();
        bits.push_u8(3 + 2u8.pow(2));
        assert_eq!(&bits.0, &[O, O, O, O, O, I, I, I]);
        bits.clear();
        bits.push_u8(3 + 2u8.pow(2) + 2u8.pow(3));
        assert_eq!(&bits.0, &[O, O, O, O, I, I, I, I]);

        bits.clear();
        bits.push_u8(u8::MAX - 2);
        assert_eq!(&bits.0, &[I, I, I, I, I, I, O, I]);
    }

    #[test]
    fn test_push_u32() {
        let mut bits = Bits::new();
        bits.push_u32(u32::MAX - 1);
        assert_eq!(u8::MAX - 1, bits.pop_u8().unwrap());
        assert_eq!(u8::MAX, bits.pop_u8().unwrap());
        assert_eq!(u8::MAX, bits.pop_u8().unwrap());
        assert_eq!(u8::MAX, bits.pop_u8().unwrap());
        assert_eq!(None, bits.pop_u8());
    }

    #[test]
    fn test_pop_u8() {
        let mut bits = Bits::new();
        bits.push_u8(10);
        assert_eq!(Some(10), bits.pop_u8());
        assert_eq!(None, bits.pop_u8());
    }
}
