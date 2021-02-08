//! Explore lifetime borrow limitations.

use std::{sync::Arc, thread::spawn};

#[derive(Debug)]
struct Foo {
    tasty: u32,
}

impl Foo {
    /// This shows how to fix a mutable borrow that *ends* the mutable borrow
    /// within itself and returns and immutable
    fn mutate_and_share<'a>(&'_ mut self) -> &'a Self {
        self.tasty += 1;
        unsafe { &*(self as *const Self) }
    }
    fn share(&self) -> &u32 {
        &self.tasty
    }
}

pub fn main() {
    let mut foo = Foo { tasty: 0 };
    let loan = foo.mutate_and_share();
    let tasty = loan.share();
    println!("{:?}", tasty);
    let tasty = foo.mutate_and_share().share();
    println!("{:?}", tasty);
    println!("{:?}", loan);

    let a = Arc::new(foo);
    let a1 = Arc::clone(&a);
    spawn(move || {
        let foo = a;
        for i in 0..10 {
            println!("thread #1, i: {}, {}", i, foo.mutate_and_share().share())
        }
    });
    spawn(move || {
        let foo = a1;
    });
    println!("{:?}", foo);
}
