#![feature(alloc_layout_extra)]
#![warn(rust_2018_idioms)]
#![allow(unused)]

mod alloc;
mod bytes;

use bytes::Bytes;
use once_cell::sync::Lazy;
use std::{alloc::Layout, cell::RefCell, default::Default, sync::Mutex};

#[global_allocator]
static ALLOCATOR: alloc::MyAllocator = alloc::MyAllocator::new();

// static LAYOUTS: Lazy<Mutex<Vec<Layout>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(1024)));

fn main() {
    ALLOCATOR.power(true);
    let mut v1: Vec<u8> = vec![1, 2, 3, 4];
    let mut v2: Vec<u8> = vec![5, 6, 7, 8, 9, 10, 11, 12];

    v1.push(252);
    v1.push(252);
    let x = v1[0].to_string();
    let sx = String::from('1');
    let x = v1[3].to_string();
    let sx = String::from('4');
    let x = v1[4].to_string();
    let sx = String::from(252u8.to_string());
    // let x = v1[0];
    // v2.push(99);
    // println!("{:?}", v3);
    // println!("{:?}", v1);
    // println!("{:?}", v2);

    // let v = vec![100, 200, 300, 400];
    ALLOCATOR.view_buf(|buf| {
        println!("{:?}", buf.0);
        println!("{:?}", buf.1);
    });
    unsafe {
        ALLOCATOR.use_global(&sx, |sx| {
            println!("{:?}", b"252");
            println!("{:?}", sx);
            println!("{:?}", Bytes::from_str(sx));
        });
    }
}
