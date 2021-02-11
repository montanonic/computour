//! https://en.wikipedia.org/wiki/Arithmetic_logic_unit

/// An ALU has two inputs and one output. It performs addition by adding the
/// binary bit patterns at its inputs, producing a bit pattern at its output
/// that is the sum of the two input bit patterns.
///
/// The addition of two binary strings is performed in the same way the addition
/// of two decimal strings is performed, from right to left, column by column.
/// If the addition in a column generates a carry, the carry is added to the
/// column immediately to its left.

/// We pick our representation of negative numbers in binary to be something
/// such that when adding happens, -x + x = 0. These circuits track carries for
/// all the bits, but do not need to (and should in fact ignore) the carry for
/// the most significant digit in these types of operations. So, for a 5-bit
/// adder, if 3 is 00011, then -3 must be the binary number that yields 00000
/// when added to 3. So:
///
/// 00011 + 11101
///
/// This is essentially just NOT 00011, aka 11100, plus 1 to force an overflow
/// from 11111 to 0.

/// Idea: let's do a simple-ass CPU that gives an explicitly binary interface