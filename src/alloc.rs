use std::{
    alloc::{self, GlobalAlloc, Layout, System},
    cell::{Cell, RefCell},
    error::Error,
    mem, panic,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::{borrow::BorrowMut, ptr};

pub struct MyAllocator {
    on: AtomicBool,
    inner: ArrAllocator,
}

impl MyAllocator {
    pub const fn new() -> Self {
        Self {
            on: AtomicBool::new(false),
            inner: ArrAllocator::new(),
        }
    }

    pub fn power(&self, on: bool) {
        self.on.store(on, Ordering::SeqCst)
    }

    pub fn is_on(&self) -> bool {
        self.on.load(Ordering::SeqCst)
    }

    fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner.alloc(layout)
    }

    fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout)
    }

    pub fn print_buf(&self) {
        unsafe {
            let msg = "Here's the buffer!\n";
            libc::write(libc::STDOUT_FILENO, msg.as_ptr() as _, msg.len() as _);
            libc::write(
                libc::STDOUT_FILENO,
                self.inner.arr.as_ptr() as _,
                self.inner.arr.len() as _,
            );
        };
    }

    /// Allows access to the buffer, disabling custom allocation for the
    /// duration.
    pub fn view_buf(&self, f: impl FnOnce((&[u8], &[usize]))) {
        let last_power = self.is_on();
        unsafe {
            self.power(false);
            f(self.inner.get_buf());
            self.power(last_power);
        }
    }
}

struct ArrAllocator {
    /// Points to the next valid spot to write to.
    write_ptr: AtomicUsize,
    arr: [u8; 100],
    layouts: [usize; 100],
    layout_ptr: AtomicUsize,
}

impl ArrAllocator {
    const fn new() -> Self {
        Self {
            write_ptr: AtomicUsize::new(0),
            arr: [0; 100],
            layouts: [0usize; 100],
            layout_ptr: AtomicUsize::new(0),
        }
    }

    unsafe fn get_buf(&self) -> (&[u8], &[usize]) {
        (&self.arr, &self.layouts)
    }

    fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let old_ptr = self.write_ptr.load(Ordering::SeqCst);
        let mut new_ptr = old_ptr;
        // First iter will be 0 as old and new ptr are the same. ptr will grow
        // by align amount, we want it to equal size eventually.
        while new_ptr - old_ptr < size {
            new_ptr += align;
        }
        self.write_ptr.store(new_ptr, Ordering::SeqCst);

        self.note_layout(layout);

        &self.arr[old_ptr] as *const _ as *mut _
    }

    fn note_layout(&self, layout: Layout) {
        let mut ptr = self.layout_ptr.load(Ordering::SeqCst);
        let arr = &self.layouts as *const _ as *mut [usize; 100];
        unsafe {
            (*arr)[ptr] = layout.size();
            (*arr)[ptr + 1] = layout.align();
        }
        self.layout_ptr.store(ptr + 4, Ordering::SeqCst);
    }

    /// Currently don't actually deallocate.
    fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if self.is_on() {
            self.alloc(layout)
        } else {
            System.alloc(layout)
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.is_on() {
            self.dealloc(ptr, layout)
        } else {
            System.dealloc(ptr, layout)
        }
    }
}
