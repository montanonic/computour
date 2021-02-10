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

/*
Idea: have the user write a VM for a CPU. It will have its own instruction set and such.
The user is then going to fix a bug in an existing CPU. To do this, they'll need to feed it binary
opcodes, checking the state each time, and comparing with their own VM, to find discrepancies.

I also love the idea of us building a VM creator, to quickly debug novel processors. Then again,
a CPU bug should be hardware related, not software... So, to debug other VM's? Meh, I'm not sure
yet what the most fleshed out idea would be.

OH! What if the program were supposedly doing something dangerous in our game, so we wanted to test
it out in a low-stakes situation before running it for real. That could be fun, and conceptually
justify us doing things this way.
*/

/*
Question: How does computer startup initialize the memory space properly for the processor?

Does the processor need to run some initialization routine to set up the RAM properly?

Is there a way to use a processor that can only handle smaller register sizes with a larger
hardware RAM size than the maximum register value? (why can't we just use multiple registers to
more slowly lookup RAM addresses?)
*/

/*
So it looks like processors expect certain areas in RAM to be solely usable by them, and the
Operating System's job is to manage the address space so that things work as expected. This
means OS's tend to expose virtual addresses spaces which do not directly map onto the physical
address space; in this way, resources that shouldn't be touched by userland can be kept that way.
*/

/*
CPU nuances to look into:

Speculative execution (this is how SPECTRE/MELTDOWN happened, would be fun to explain the exploit!)
Instruction pipelining
*/

/*
"An even more simplistic processor than our VM might be missing a MUL instruction and require the
programmer to use ADD and JMP instructions.""
*/

/*
Figure out a way to bridge Rust's borrowchk into outputting in our Lispy assembly language.
What do I mean by this? Well: we want to show how to *really* make a compiler, but without the
messiness of actually getting LLVM/Assembly to run on your computer (not yet at least).
So, we create a Lispy assembly lang (because parsing is so easy) that compiles down into our
VM bytecode. Smalltalk syntax would also be nice.

Anyways, so, I personally don't want to write the Lispy code by hand. Instead, I'd like to
write my program in Rust, and have it give me the Lispy code as optional output (or just the
direct bytecode). Both can and will be implemented.

Now, if I have Rust giving me the Lispy code, then that means I can use all of the power of
Rust's system to do this. Ideally then, I'd love to use Rust to track and manage that we output
sane assembly code, with proper lifetimes and all that snazzy jazz.

In fact, since it's not intended to be written by hand, perhaps we could even use whitespace
sensitive syntax. That would be interesting, and make the output a lot cleaner. Perhaps offer
both versions? One for easy reading, and the other for general portability and easy writing.

Eventually, could we write a high-level, typed Lisp, that we attempt to rewrite full-on Rust code
in? The idea being that we could use such a language to actually, piece by piece, start to deeply
understand how Rust itself is implemented, but without wasting any time on parsing nonsense.

It would be absolutely fascinating to try and so some borrowchk stuff in this language. As in try
to make our sub-language itself not let us write bad programs, haha. Because we don't have to bring
in the whole ecosystem and complexity of Rust with us, especially as we'd be operating at an even
lower level than Rust, we'll be free to tweak how Rust actually works (like allow orphan instances).

All in all, it'd be a good way to explore Rust, and a fun way to understand programming languages.

Oh, and our Lisp will not involve lists :). Which reminds me: once we get some beefier theoretical
CPUs up, we should take a look at why lists are so flipping inefficient.
*/

/*
Once we have a register based VM, we could hilariously write a stack-based VM that compiles into
our register based bytecode. Everything can emulate everything, right? A computer is a computer.
This stuff goes deep.
*/

/*
I think we need at least 3 different CPU instruction sets. Probably two RISC, and one CISC.
We'll want them to be quite different if possible, and the goal will be: can we come up with
a higher-level Virtual CPU that makes it easy to compile into all of these architectures?

This essentially starts to get at what LLVM itself is really doing, as we'd be implementing our
own here, but of course more simplified in scope.

One thing we'd definitely want to do is look at the tradeoffs an intermediate language like this
makes. Add one more instruction and you'll make compiling to one CPU easier, but perhaps
make another more difficult as it has no natural representation for a certain command. Understanding
stuff like this should involve talking to more Rustacians and looking into LLVM stuff.
*/

/*
One of the fun things about VMs is that once you're compiling programs into bytecode, all you need
to run that code is an implementation of the VM. By making a particularly simple VM, which we
could test by running it itself in more complex CPU emulators to check cache-efficiency and the like,
and then benchmark comparing those predictions with actual runtime performance (ideally they will
actually be quite predictive, which would be awesome because then we'd be validating that our
machine models totally made sense), we'd have something with a potentially very small surface area
that any language could implement. Then, just by reading a file of bytecode data and running it on
this VM, we have fully portable code!

And for higher performance, more complex VM implementations could be used.

Things to explore here: Just in Time Compilation, and other tricks of the trade!
*/

/*
The robots want to build a new internet, so later stage tasks could be implementing simpler
versions of UDP/TCP/HTTP? Let's learn more about how network traffic propegates and so on.

But! Also this could be version 2 of the game, you know? Like let's focus on CPU and OS stuff
here to keep our focus more in alignment. I'd rather explore Programming Languages in more
depth than network stuff IMO, at least right now. On the OS side though network stuff does
make more sense to look into because of security implications and so on. But also that doesn't
necessarily require diving into the protocols to look at. Worth keeping in mind.
*/

/*
LC-3 idea. Okay so in our story let's say some actualy LC-3 CPUs were created, but there aren't
many, and the robots don't know of any programs written for them. So it will be our job to implement
them and get them talking to each other.

What's nice about LC-3 is that it gives us a super reduced instruction set. LC-4 would basically be:
what's the minimal amount of additions we can make to dive even deeper into the nuances of modern
CPUs?
*/

/*
As mentioned to Dee in our conversation, by expecting the player to provide us Interfaces to their
designs, we would open up much more room in our game to offer different areas of interest for learners
to go down without needing to overly commit to anything. I do think though that it's easiest to build
a compelling narrative linearly, so maybe I should start there and circle back to this as I have more
ideas fleshed out, and figure out how I can give the learner a lot of freedom and openness, and
facilitate their own learning of the things they're more interested in. Ideally a learner is curious
to explore 60-80% of our problems, and I don't think the game should demand that they go through
everything to get to the "end" (a final challenge that pulls together everything). They just need
to do enough, and hit the major concepts.

I feel like now I'm overthinking the high-level of this and need to get back into the nitty gritty of
working things out.
*/

/*
I want there to be an engineering hub in the game where the player can benchmark and test out their
virtual machines. One fun thing we can even do there is have the player write a program in a higher
level language and compile it down to all the different architectures, and then we benchmark the
performance of the different VMs they've made against each other.

How to benchmark? Program to an interface. Have an API that makes it easy for us to count every
instruction a VM performs. Then we use that interface to call it and track it through that. I'd
want to put a little effort to make sure it's not *obviously* exploitable, but I don't care about
people who want to hack it; I just don't want giving crap results to be so readily apparent that
people devalue the meaningfulness of the testing.
*/
