/*
So I have this idea for a learning game that explores lower-level concepts in a nice virtual
environment.

Pedagogically, one thing I'm thinking of for teaching, is that when you have a challenge to test your
learning skills, and give you free reign over how you come to solve it, we want a way to encourage
you if you get stuck, but a hints system kind of says "hey do you need help?".

And I think some people don't really like to feel like they need help, or to feel like they are
wasting time tryign to solve without help. So instead of offering "tips" or "hints", I'd like to
create sub-problems that are much easier. But each subproblem will test a skill that's used to
solve the problem. And each subproblem will have at least a second phase that does not require a
refactor.

These subproblems are thus the "hints", and they are always there. But unlike a normal hint, it doesn't
just tell you how to solve the problem. So knowing what a subproblem is, only feels like a hint once
you work on it. At a glance they should be designed to not give that much information away.

So then you can solve the main problem all by yourself if you want, and then the subproblems open up
with more complexity once you've solved the main problem. If you've already solved a subproblem, the
extra complexity does not necessitate rewriting how you solved it. But if you *havent* worked on them
yet, you still are given the opportunity to do so, but now they are more interesting.

So this creates a situation when the hard problem can always be tackled straight away for a challenge
and testing. But the subproblems outline it, and you'll eventually have to do them. So the main problem
will be illustrated through these subproblems: solving them will teach you how to solve the main one.
But after you've solved the main problem, the relationship will invert: the second phase of subproblems
will require an understanding of (or working version of) the main problem to solve.

This creates non-linear paths of navigating how you solve each phase of learning. The tension contour
will naturally be adapted by your own learning style.


Each phase of the way, a main dialogue will be available to guide you. It will be kind of like a
programming reference.
*/

/*
Write custom allocator code in Rust and see if I can swap out and drop one allocator for another
mid process. This sounds interesting!

Write a fixed size allocator that solely consists of an array.

Can create a custom allocator which actually sends requests to the game server so that
user code and game can interract. Could try using a double allocator pattern where I have
some special structures which bypass the custom allocator and use the default allocator
instead. I'm honestly not sure how I could do something like a socket or network call from
within allocation code, but I guess that makes it a fun challenge!

Implement a virtual stack as well!
*/

/* Apparently this does no allocation:
let init_jobs: Box<[UnsafeRay; RAYPOOL_SIZE]> =
  std::iter::repeat_with(UnsafeRay::default)
    .take(RAYPOOL_SIZE)
    .collect::<Vec<_>>()
    .into_boxed_slice()
    .try_into()
    .unwrap()

a boxed slice can never be pushed/popped
also, a Vec that you never push/pop to won't reallocate :slight_smile:
And seri's usage of collect will only perform one allocation
since the length of the iterator is known

yup, no reallocs, single alloc + no unsafe here

pedantically, this still allocates one struct on the stack and then copies it to memory
which is theoretically not required
but it's probably fine, considering this is probably setup code

esp. since the only problem was "my stack was too big"

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=b83ecfcf45eb941749a4ea44a92f6ca8
*/

/*
This wouldn't be the most useful, but perhaps as a fun way to explore reference/pointer invalidation,
we can create a vector that loans out a special reference pointer struct that is tracked by the vector
itself (so it's bound to the Vector's lifetime, as it must obviously be), and anytime the vector
reallocates/resizes, it will iterate through and flag all of these references so that they know
they are (almost certainly) invalid.

We do this lazily so that we don't manually update each reference earlier than needed.

Then, when a reference is used, if that flag is up, it will under the hood do a lookup to that
location in the new vector, possibly failing if the vector has, for example, decreased in size.

If I created a game-situation that actually made use of this quite artificial structure, it'd
certainly be a fun way to test out some nicher skills and demonstrate a deeper understanding of
why we very carefully don't allow references to be alive when a vector is mutated.

But, of course, we *can* use these skills to build a less restrictive vector that doesn't require
the user to drop references, but instead makes each of those references no longer have a guaranteed
lookup.
*/

/*
One thing I want to be *sure* to do is to illustrate a meaningful example of a race condition in a
single-threaded context. I want to make absolutely clear how an aliased mutable reference is harmful
even in this type of context.

It's obvious to imagine a simultaneous read-write failing in a concurrent context, but it's a lot
harder to see this in a single-threaded case where you might imagine "well doesn't that mutable thing
finish before the read happens no matter what?".

And personally, I don't know! That's why I feel this is important to understand, and important to
illustrate through a virtual CPU to concretize the problem. Obviously a "mutation" is usually
multiple CPU instructions, and so if there was a read that happened in-between you could easily
get clobbered state and thus UB. But to my understanding the Rust compiler wouldn't ever have
those things interleave... right?

Here's some answers: iterator invalidation: make an iterator, push/pop on the original data structure,
your iterator is now invalid unless it knows to alter itself when the data it's referencing changes
(which also means that by-default you'd have broken behavior in a way that is not possible in Safe Rust
due to disallowing mutation when there's any concurrent borrow (concurrent not implying multi-threaded
or parallel in this case)).

By extension, anything that has a state-dependency which can't be captured in a single step is a
possibility of a data race. Rust has semantics that guarantee data-race UB will never happen, and
these same semantics help make it *much* easier to guarantee that data-race logic failures will
never happen, because every mutation is, of course, exclusive by default, *no matter what*, without
manually adding special types that allow you to change this in specific ways, or going into unsafe Rust.
*/
