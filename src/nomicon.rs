//! Notes on unsafety:
//!
//! A reference/pointer is "dangling" if it is null or not all of the bytes it
//! points to are part of the same allocation (so in particular they all have to
//! be part of some allocation).
//!
//! *const () (or equivalent) works reasonably well for void*, and can be made
//! into a reference without any safety problems. It still doesn't prevent you
//! from trying to read or write values, but at least it compiles to a no-op
//! instead of UB.

pub fn main() {}
