use std::{
    alloc::{self, GlobalAlloc, Layout, System},
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
};
use std::{borrow::BorrowMut, ptr};

pub struct MyAllocator {
    on: AtomicBool,
}

impl MyAllocator {
    pub const fn new() -> Self {
        Self {
            on: AtomicBool::new(false),
        }
    }

    pub fn power(&self, on: bool) {
        self.on.store(on, Ordering::SeqCst)
    }

    pub fn is_on(&self) -> bool {
        self.on.load(Ordering::SeqCst)
    }
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if self.is_on() {
            crate::LAYOUTS.try_lock().map(|mut v| {
                v.push(layout);
            });
        }
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}
