use crate::bytes;
use bytes::Bytes;
use io::{Read, Write};
use std::{
    borrow::Cow,
    cmp, io, mem,
    ops::{Add, Deref, DerefMut},
    str::FromStr,
};
use strum_macros::{EnumString, EnumVariantNames};

/* The idea with a stack machine will be to... */
#[derive(Debug, PartialEq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum StackFormat {
    Decimal,
    #[strum(serialize = "0b", serialize = "binary")]
    Binary,
    #[strum(serialize = "0x", serialize = "lowerhex")]
    LowerHex,
    #[strum(serialize = "0X", serialize = "upperhex")]
    UpperHex,
}

impl From<&str> for StackFormat {
    fn from(val: &str) -> Self {
        StackFormat::from_str(val).unwrap_or(StackFormat::Decimal)
    }
}

pub fn run() {
    let mut input = String::new();
    let mut sm = StackMachine::new();
    let mut format = StackFormat::Decimal;

    println!("Welcome!");

    let usize_bytes = mem::size_of::<usize>();
    let usize_bits = usize_bytes * 8;
    println!(
        "Your architecture uses {} bytes ({} bits) to reference values in memory.",
        usize_bytes, usize_bits
    );

    println!("{:?}", (u16::MAX.wrapping_add(1)).to_be_bytes());

    loop {
        input.clear();
        print!(">> ");
        io::stdout().flush();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => println!("error: {}", e),
        }

        let mut args = input.split_whitespace();

        match args.next().unwrap_or("") {
            s @ ("0x" | "0X" | "0b") => format = s.into(),
            "format" => {
                args.next().map(|s| format = s.into());
            }
            "bp" => println!("{:#016x}", sm.bp()),
            "sp" => println!("{:#016x}", sm.sp()),
            "pop" => {
                sm.pop().map(|x| println!("{}", x));
            }
            "add" => sm.add(get_op_bytes(args, &sm)),
            "dup" => n_times(args, |_| sm.dup()),
            // Try to interpret numbers.
            s => {
                if let Ok(n) = s.parse::<u8>() {
                    sm.push(n);
                } else if s.starts_with("0x") {
                    let hex = s.strip_prefix("0x").unwrap();
                    u8::from_str_radix(hex, 16).map(|x| sm.push(x));
                } else {
                    continue;
                }
            }
        }

        let bytes = Bytes::from_slice(&sm.stack);
        match format {
            StackFormat::Decimal => println!("Stack: {:?}", sm.stack),
            StackFormat::Binary => println!("Stack (0b): {:0>8b}", bytes),
            StackFormat::LowerHex => println!("Stack (0x): {:0>2x}", bytes),
            StackFormat::UpperHex => println!("Stack (0X): {:0>2X}", bytes),
        }
    }
}

struct LangInterpreter<'sm> {
    input: String,
    multiline: bool,
    sm: &'sm mut StackMachine,
    format: StackFormat,
}

impl<'sm> LangInterpreter<'sm> {
    /// Prepare for parsing a program written in our stack machine.
    pub fn new<'a>(program: impl Into<Cow<'a, str>>, sm: &'sm mut StackMachine) -> Self {
        Self {
            input: program.into().into(),
            multiline: true,
            format: StackFormat::Decimal,
            sm,
        }
    }

    /// Prepare for parsing only a single line of program input.
    pub fn from_line<'a>(line: impl Into<Cow<'a, str>>, sm: &'sm mut StackMachine) -> Self {
        let mut this = Self::new(line, sm);
        this.multiline = false;
        this
    }

    pub fn interpret(&mut self) {
        if self.multiline {
            todo!()
        } else {
            self.interpret_line()
        }
    }

    /// This could be made pure, but not gonna.
    fn interpret_line(&mut self) {
        let mut args = self.input.split_whitespace();
        let sm = self.sm;
        let format = &mut self.format;

        // if let Some(first) = args.next() {

        // } else {
        //     println!("bad argument")
        // }

        match args.next().unwrap_or("") {
            s @ ("0x" | "0X" | "0b") => *format = s.into(),
            "format" => {
                args.next().map(|s| *format = s.into());
            }
            "bp" => println!("{:#016x}", sm.bp()),
            "sp" => println!("{:#016x}", sm.sp()),
            "pop" => {
                sm.pop().map(|x| println!("{}", x));
            }
            "add" => sm.add(get_op_bytes(args, &sm)),
            "dup" => n_times(args, |_| sm.dup()),
            // Try to interpret numbers.
            s => {
                if let Ok(n) = s.parse::<u8>() {
                    sm.push(n);
                } else if s.starts_with("0x") {
                    let hex = s.strip_prefix("0x").unwrap();
                    u8::from_str_radix(hex, 16).map(|x| sm.push(x));
                }
            }
        }
    }
}

/// Allows repeating an operation based upon a second arg.
fn n_times<'a>(mut args: impl Iterator<Item = &'a str>, mut f: impl FnMut(u8)) {
    with_next_arg(args, |times: u8| {
        for n in 0..times {
            f(n)
        }
    });
}

fn get_next_arg<'a, T: FromStr>(mut args: impl Iterator<Item = &'a str>) -> Option<T> {
    args.next().and_then(|s| s.parse::<T>().ok())
}

/// Does nothing if arg is missing or fails to parse, returning None.
fn with_next_arg<'a, T: FromStr, U>(
    mut args: impl Iterator<Item = &'a str>,
    mut f: impl FnMut(T) -> U,
) -> Option<U> {
    get_next_arg(args).map(|x| f(x))
}

fn get_op_bytes<'a>(mut args: impl Iterator<Item = &'a str>, sm: &StackMachine) -> u8 {
    get_next_arg(args).unwrap_or(sm.op_bytes)
}

pub struct StackMachine {
    stack: Vec<u8>,
    /// The number of bytes an operation acts on as a default.
    op_bytes: u8,
}

impl StackMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            op_bytes: 1,
        }
    }

    /// Base pointer: the place the stack begins. We will for simplicity always
    /// choose this to begin at 0.
    pub fn bp(&self) -> usize {
        0
    }

    /// Stack pointer: the top of the stack, which is where the *next* element
    /// should be added. The last element is sp - 1.
    pub fn sp(&self) -> usize {
        self.stack.len()
    }

    // pub fn push(&mut self, val: u8) {
    //     self.stack.push(val);
    // }

    // pub fn extend_from_slice(&mut self, slice: &[u8]) {
    //     self.stack.extend_from_slice(slice);
    // }

    // pub fn pop(&mut self) -> Option<u8> {
    //     self.stack.pop()
    // }

    fn pop_n(&mut self, bytes: u8) -> Vec<u8> {
        self.stack.split_off(self.sp() - bytes as usize)
    }

    pub fn add(&mut self, bytes: u8) {
        let mut overflow: u8 = 0;
        let vec = self
            .pop_n(bytes)
            .into_iter()
            .zip(self.pop_n(bytes).into_iter())
            .rev() // Big endian, start with the least significant first.
            .map(
                |(b1, b2)| match b1.checked_add(b2).and_then(|x| x.checked_add(overflow)) {
                    Some(n) => {
                        overflow = 0;
                        n
                    }
                    None => {
                        let res = b1.wrapping_add(b2).wrapping_add(overflow);
                        overflow = 1;
                        res
                    }
                },
            );

        self.stack.extend(vec);
    }

    pub fn dup(&mut self) {
        if let Some(&x) = self.stack.last() {
            self.stack.push(x);
        }
    }
}

impl Deref for StackMachine {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

impl DerefMut for StackMachine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stack
    }
}

enum Instruction {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Comparison
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
    // Logic
    Not,
    And,
    Or,
    // Control
    Jmp,
    Branch,
    Call,
    Return,
    Print,
    Noop,
}

#[cfg(test)]
mod tests {
    use super::StackMachine;

    #[test]
    fn adding_16bit_works() {
        let mut sm = StackMachine::new();
        // Overflow to 0.
        sm.extend_from_slice(&[255, 255, 0, 1]);
        sm.add(2);
        assert_eq!(&sm.stack, &[0, 0]);
        // Overflow to n u8.
        for n in 0..u8::MAX {
            sm.clear();
            sm.extend_from_slice(&[255, 255, 0, 1 + n]);
            sm.add(2);
            assert_eq!(&sm.stack, &[0, n], "failed on {}", n);
        }
        // Overflow to n, u16.
        for n in (u8::MAX as u16)..u16::MAX {
            sm.clear();
            sm.extend_from_slice(&[255, 255]);
            sm.extend_from_slice(&(n + 1).to_be_bytes());
            sm.add(2);
            assert_eq!(&sm.stack, &n.to_be_bytes());
        }
    }
}
