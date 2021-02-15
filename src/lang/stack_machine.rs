use crate::bytes;
use bytes::Bytes;
use io::{Read, Write};
use std::{cmp, io, mem, ops::Add};

/* The idea with a stack machine will be to... */

pub fn run() {
    let mut input = String::new();
    let mut sm = StackMachine::new();

    println!("Welcome!");

    let usize_bytes = mem::size_of::<usize>();
    let usize_bits = usize_bytes * 8;
    println!(
        "Your architecture uses {} bytes ({} bits) to reference values in memory.",
        usize_bytes, usize_bits
    );

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
            "bp" => println!("{:#016x}", sm.bp()),
            "sp" => println!("{:#016x}", sm.sp()),
            "pop" => {
                sm.pop().map(|x| println!("{}", x));
            }
            "add" => sm.add(get_op_bytes(args, &sm)),
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
        println!("Stack: {:#x}", bytes);
    }
}

fn get_op_bytes<'a>(mut args: impl Iterator<Item = &'a str>, sm: &StackMachine) -> u8 {
    args.next()
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(sm.op_bytes)
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

    pub fn push(&mut self, val: u8) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.stack.pop()
    }

    fn pop_n(&mut self, bytes: u8) -> Vec<u8> {
        self.stack.split_off(self.sp() - bytes as usize)
    }

    pub fn add(&mut self, bytes: u8) {
        let mut overflow: u8 = 0;
        let vec = self
            .pop_n(bytes)
            .into_iter()
            .zip(self.pop_n(bytes).into_iter())
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
