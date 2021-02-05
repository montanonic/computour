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
