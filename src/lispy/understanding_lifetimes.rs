use std::mem;

struct MyIterator<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let elem = self.slice.get(0);
        self.slice = &self.slice[1..];
        elem
    }
}

struct MyMutableIterator<'slice, T> {
    slice: &'slice mut [T],
}

impl<'slice, T> Iterator for MyMutableIterator<'slice, T> {
    type Item = &'slice T;
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let blah: &'_ mut [T] = self.slice;
        // This lives for a lifetime *less than 'next*. The anonymous lifetime
        // here is the shortest one in this entire scope, because it relies upon
        // a borrow of self, which has a 'next lifetime, but we want to release
        // the borrow before the end of the function. So under the hood this is
        // what happens: our shorter lifetime is 'a, we bound 'next: 'a, to live
        // as least as long as this lifetime, or conversely, that 'a cannot
        // outlive 'next. We borrow this, up through calling mem::replace, and
        // then shadow it, after which the lifetime 'a is no longer in use.
        let mut slice: &'_ mut &'slice mut [T] = &mut self.slice;
        // &mut self.slice is a pointer to a pointer, and we swap the existing
        // pointer to our iterator's slice out with a pointer to an empty slice,
        // &mut []. This means that we've literally changed the value at
        // self.slice, but in a way that doesn't require borrowing past this
        // line of code (since it returns an owned value). In our case, the
        // owned value is just a (fat) pointer, so we can drop it with no
        // consequences (the owner of the data pointed to by the slice will free
        // it eventually, when it's time, but our pointer just resides on the
        // stack and gets popped off).
        let slice: &'slice mut [T] = mem::replace(slice, &mut []);
        let (first, rest): (&'slice mut T, &'slice mut [T]) = slice.split_first_mut()?;
        // We're able to do this because we have no outstanding borrows to self
        // at this point.
        self.slice = rest;
        // And we're able to do this because we managed to take ownership of the
        // original slice via mem::replace, so by owning it we're able to
        // operate with the exact same lifetime as the original slice, rather
        // than a reduced lifetime from the implicit reborrowing in self.slice
        // (which reduces the lifetime to &'a mut [T] (where <'next: 'a>, which
        // says that all references in 'next must outlive 'a)).
        Some(first)
    }
}

fn test<'a: 'b, 'b, T>(mut slice: &'a mut [T]) -> &'b mut [T] {
    let slice: &'b mut [T] = &mut *slice;
    slice
}

// Exploring some interesting things. &mut references do not provide copy or
// clone, are always moved, thus you totally lose access to the struct if you
// dot into them, because they *move* out of the struct rather than copying like
// immutable references do. You can put them behind an immutable pointer, and
// then that's fine, because those can be copied. But they prevent you from
// using mutation, duh.

// impl<T> MyMutableIterator<'_, T> {
//     fn touch(&mut self) {}
// }
// fn stuff() {
//     struct Foo;
//     let iter = MyMutableIterator { slice: &mut [Foo] };
//     let slice = &mut iter.slice;
//     iter.touch();
//     slice[0] = Foo;
// }
