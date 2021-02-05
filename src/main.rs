#![warn(rust_2018_idioms)]
#![allow(unused)]

use std::{alloc::Layout, cell::RefCell, default::Default, sync::Mutex};

use once_cell::sync::Lazy;
mod alloc;

#[global_allocator]
static ALLOCATOR: alloc::MyAllocator = alloc::MyAllocator::new();

static LAYOUTS: Lazy<Mutex<Vec<Layout>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(1024)));

fn main() {
    LAYOUTS.lock().unwrap(); // Force initialization.
    ALLOCATOR.power(true);
    let v = vec![1, 2, 3];
    ALLOCATOR.power(false);
    println!("Hello, world! {:?}", LAYOUTS);
}
