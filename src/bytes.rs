use std::{
    borrow::Cow,
    fmt::{self, Formatter, LowerHex, Result, UpperHex},
    ops::Deref,
};
/// Newtype wrapper around byte vector. Derefs into Vec, so can be used as
/// normal, but supplements Vec with more tools for manipulating bytes.
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// This is a nice implementation as it actually performs 0 copying if
    /// passed a String, and 1 copy if passed &str.
    pub fn from_str<'a, I: Into<Cow<'a, str>>>(string: I) -> Self {
        Self(Self::cow_to_bytes(string))
    }

    pub fn cow_to_bytes<'a, I: Into<Cow<'a, str>>>(string: I) -> Vec<u8> {
        let string: String = string.into().into_owned();
        string.into_bytes()
    }

    pub fn insert(&mut self, index: usize, element: u8) {
        self.0.insert(index, element);
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    // /// Treats the underlying bytes as unicode and reverses them by grapheme.
    // pub fn reverse_as_string(&mut self) {
    //     let mut str = String::from_utf8_lossy(self);
    //     reverse_grapheme_clusters_in_place(str.to_mut());
    //     self.0 = Self::cow_to_bytes(str);
    // }
}

impl Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl DerefMut for Bytes {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

#[macro_use]
mod format_macro {
    macro_rules! fmt_impl {
        ($Self:ident, $format:literal) => {
            impl fmt::$Self for Bytes {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    write!(f, "[");
                    for (i, x) in self.iter().enumerate() {
                        let val = if f.alternate() {
                            format!(concat!("{:#", $format, "}"), x)
                        } else {
                            format!(concat!("{:", $format, "}"), x)
                        };
                        f.pad(&val);

                        if i != self.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, "]")
                }
            }
        };
    }
}

fmt_impl!(Binary, "b");
fmt_impl!(LowerHex, "x");
fmt_impl!(UpperHex, "X");

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "{}", String::from_utf8_lossy(self))
        } else {
            write!(f, "[");
            for (i, x) in self.iter().enumerate() {
                write!(f, "{:?}", x)?;

                if i != self.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")
        }
    }
}
