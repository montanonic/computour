use std::{
    alloc::{self, GlobalAlloc, Layout, System},
    array,
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    error::Error,
    mem, panic, ptr,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
};

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

    /// Unsafe because you must ensure that all values allocated by one
    /// allocator are dropped by that same allocator, and also that every
    /// distinct type is entirely handled by one allocator.
    ///
    /// So, basically, you need to very clearly delimit boundaries between
    /// custom and default allocation, and *not* mix without careful
    /// consideration of possible allocations that could happen and lead to UB.
    pub unsafe fn power(&self, on: bool) {
        self.on.store(on, Ordering::SeqCst)
    }

    pub fn is_on(&self) -> bool {
        self.on.load(Ordering::SeqCst)
    }

    /// Get the internal allocator.
    unsafe fn get_alloc(&self) -> &ArrAllocator {
        &self.inner
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if self.is_on() {
            self.inner.alloc(layout)
        } else {
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.is_on() {
            self.inner.dealloc(ptr, layout)
        } else {
            System.dealloc(ptr, layout)
        }
    }

    /// Allows access to the buffer, and layout request buffer, disabling custom
    /// allocation for the duration.
    pub fn view_buf(&self, f: fn((&[u8], &[usize]))) {
        unsafe {
            self.use_global_with_closure(|| {
                f(self.inner.get_buf());
            })
        }
    }

    /// Run code without using the custom allocator. To ensure safety, we only
    /// allow references to types to be passed, so that we can avoid any
    /// allocating behavior, and thus don't accept closures (which could allow
    /// for moved values). You must be very careful to ensure that no code which
    /// has any allocation effect on &Args is ran; this means that possibly even
    /// standard library code could cause UB here if it allows for the
    /// possibility of allocation through an & reference.
    ///
    /// For this reason, even though there's already some built-in safety here,
    /// we cannot guarantee that this function is safe to call on arbitrary safe
    /// code.
    pub unsafe fn use_global<Args>(&self, args: &Args, f: fn(&Args)) {
        unsafe {
            self.use_global_with_closure(|| {
                f(args);
            })
        }
    }

    /// Care has to be taken to ensure no custom allocated values get moved into
    /// the closure, otherwise the default allocator will deallocate them, which
    /// is UB.
    unsafe fn use_global_with_closure(&self, f: impl Fn()) {
        let last_power = self.is_on();
        self.power(false);
        f();
        self.power(last_power);
    }
}

// /// Low level interface to tracking layout requests made by an allocator.
// struct LayoutTracker {
//     arr: Mutex<*mut [u8]>,
//     ptr: AtomicUsize,
//     len: AtomicUsize,
// }

// impl LayoutTracker {
//     pub fn new() -> Self {
//         const LEN: usize = 100; // Default to 100 size.
//         let arr = [0u8; 100];
//         let x = arr.as_mut_ptr();
//         let arr = Mutex::new(arr.as_mut_ptr());
//         Self {
//             arr,
//             ptr: AtomicUsize::new(0),
//             len: AtomicUsize::new(LEN),
//         }
//     }
// }

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

    /// Unsafe because this allows slices to be read while they also might be
    /// mutably aliased by the allocator. Only safe to read when no custom
    /// allocation is happening, and refs must be dropped before the next custom
    /// allocation happens.
    unsafe fn get_buf(&self) -> (&[u8], &[usize]) {
        (&self.arr, &self.layouts)
    }

    /// The interesting thing about alloc is that we never actually write the
    /// structure ourself, instead we just *prepare* for allocation, and then
    /// give Rust a pointer to a location where it has the appropriate amount of
    /// space to allocate within.
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

        self.note_layout(layout, false);

        &self.arr[old_ptr] as *const _ as *mut _
    }

    fn note_layout(&self, layout: Layout, is_dealloc: bool) {
        let mut ptr = self.layout_ptr.load(Ordering::SeqCst);
        let arr = &self.layouts as *const _ as *mut [usize; 100];
        unsafe {
            (*arr)[ptr] = layout.size();
            (*arr)[ptr + 1] = layout.align();
            if is_dealloc {
                (*arr)[ptr + 2] = 99;
            }
        }
        self.layout_ptr.store(ptr + 4, Ordering::SeqCst);
    }

    /// Currently don't actually deallocate.
    fn dealloc(&self, mut ptr: *mut u8, layout: Layout) {
        unsafe {
            let mut n = 0;
            let size = layout.size();
            // Zero out the deallocation.
            while n < size {
                *ptr = 0;
                ptr = ptr.offset(1);
                n += 1;
            }
        }
        self.note_layout(layout, true);
    }

    fn log_layout_req(layout: Layout) {
        unsafe {
            // let align_ref = &layout.align() as *const usize;
            // libc::write(
            //     libc::STDOUT_FILENO,
            //     align_ref as _,
            //     mem::size_of::<usize>() as _,
            // );
            // libc::write(
            //     libc::STDOUT_FILENO,
            //     align_ref as _,
            //     mem::size_of::<usize>() as _,
            // );
            // let msg
            // libc::write(
            //     libc::STDOUT_FILENO,
            //     align_ref as _,
            //     mem::size_of::<usize>() as _,
            // );
        };
    }
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout)
    }
}
