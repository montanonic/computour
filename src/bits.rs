/// Currently bits is backed by a Vec<u8>, but I can definitely optimize this if
/// I decide to later.
pub struct Bits(Vec<u8>);

impl Bits {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.0.push(byte)
    }
}

/// Read "Bit 16" for 16bit.
struct B16(u16);

// impl U16 {
//     pub fn new(u: u16) -> Self {
//         Self(u)
//     }

//     pub fn sign_extend(i: u16) -> Self {}
// }
