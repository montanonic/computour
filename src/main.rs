#![feature(
    alloc_layout_extra,
    or_patterns,
    bindings_after_at,
    type_alias_impl_trait
)]
#![warn(rust_2018_idioms)]
#![allow(unused)]

mod alloc;
mod bits;
mod bytes;
mod game;
mod lang;
mod lc3;
mod lispy;
mod nomicon;
mod old;

use old::vm;
use std::env::args;
use strum_macros::EnumString;

#[global_allocator]
static ALLOCATOR: alloc::MyAllocator = alloc::MyAllocator::new();

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum Run {
    Nomicon,
    LC3,
    VM,
    Game,
    Lang,
    Lispy,
    Default,
}
use Run::*;

fn main() {
    let mut args = args();
    let run = args.nth(1).map_or(Default, |str| {
        str.parse()
            .expect("argument did not match existing module to run")
    });

    match run {
        Nomicon => {
            return nomicon::main();
        }
        LC3 => {
            return lc3::main();
        }
        VM => {
            return vm::main();
        }
        Game => {
            return game::main();
        }
        Lang => {
            return lang::main();
        }
        Lispy => {
            return lispy::main();
        }
        Default => {
            println!("Running default main");
        }
    }

    unsafe {
        ALLOCATOR.power(true);
    }

    #[derive(Clone, Debug)]
    struct Test {
        u32: u32,
        u8: u8,
        u16: u16,
    }

    let t = Test {
        u32: 32,
        u8: 8,
        u16: 16,
    };

    let mut v1: Vec<Test> = vec![t.clone(), t];

    let mut v2: Vec<&Test> = Vec::with_capacity(1);

    v2.push(&v1[0]);

    let mut v4: Vec<&[Test]> = Vec::with_capacity(1);

    v4.push(&v1[..]);

    // v1.as_ptr()

    // let mut v2 = vec![1u8; 1];

    ALLOCATOR.view_buf(|buf| {
        println!("{:?}", buf.0);
        println!("{:?}", buf.1);
    });

    unsafe {
        ALLOCATOR.use_global(&(v4), |(v)| {
            let _ = println!("{:?}", v[0][0]);
        });
    }
}
