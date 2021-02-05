#![feature(alloc_layout_extra)]
#![warn(rust_2018_idioms)]
#![allow(unused)]

use std::{alloc::Layout, cell::RefCell, default::Default, sync::Mutex};

use once_cell::sync::Lazy;
mod alloc;

#[global_allocator]
static ALLOCATOR: alloc::MyAllocator = alloc::MyAllocator::new();

// static LAYOUTS: Lazy<Mutex<Vec<Layout>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(1024)));

fn main() {
    ALLOCATOR.power(true);
    let v = vec![1u8, 2, 3, 4];
    let v = vec![5u8, 6, 7, 8, 9, 10, 11, 12];
    Box::new(u64::MAX);
    let v = vec![1u8, 2, 3, 4];
    Box::new(u128::MAX);
    Box::new(42);

    Box::new(42);

    // let v = vec![100, 200, 300, 400];
    ALLOCATOR.view_buf(|buf| {
        println!("{:?}", buf.0);
        println!("{:?}", buf.1);
    });
}
