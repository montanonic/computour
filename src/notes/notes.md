# Scratchpad

## High level VM
Lisp? Mathematical sets? Categories (without the overmathy cruft)? The core point being to have a VM that supports low-level semantic meaning in a way that is straightforward to implement on machines, but that also makes far fewer assumptions about how machines are supposed to work. As valuable as it is to learn about how our processors work now, it is equally valuable to allow ourselves to think beyond the inherent limitations of the way things are, so that we can build the future. I think Bret Victor brought this up in one of his talks, heavily paraphrasing: "What ways do we let ourselves dictate how a program out to work because of the hardware we have right now?". My guess it was the beautiful talk "The Future of Programming".

## Teach linear algebra
Intuitively and computationally, in-browser, targetted to programmers, with visualizations everwhere.

## Explore WASM
Go deeper into the ISA and architecture for WASM. Write code that compiles to it. Learn how to make a file of WASM code that will actually run in the browser so that I don't (solely) have to rely upon Rust to do it for me.

WASM might be a good opportunity to focus on higher-level constructs. That said I wonder if I can implement an even higher level VM that compiles to it. Losing some efficiency is okay as long as I can have a programming environment where I get to explore both outputs and think and learn from them.

## Interlanguage communication
I was storm-braining some ideas here. Because of my recent studies I'm more performance-aware, so I was wanting to think of highly efficient interprocess communication. I think that this might be something inherently conjoined with high level VM ideas. Perhaps a Turing machine is a good example of a "high level" VM just in terms of: if you can express your program as one, you can communicate to any program, because any program should be able to interpret a Turing machine, and it's a known standard. There could of course be other types of models, with different performance characteristics.

I was imagining that there might be a negotiation stage where some binary format was figured out, perhaps to optimize for communication between two rather different architectures. The idea was that in the end, we're pushing bits, so it shouldn't be too hard to have an agreed protocol around (1) naming things (this is just memory), and (2) doing arithmetic and logic on those things. And the thing is that we can *really* boil down so many complex ideas into some basic-ass binary arithmetic and conditional branching. I'd love to explore this stuff more deeply, so that I can have stronger intuitions for what a good balance for a VM would be.

## Assembly everywhere
I want all programming to be learned alongside a "view" into the lower level mechanisms of execution. I don't find these details to be trivial or distracting, quite the opposite: it very clearly communicates away the magic behind our code. It brings us closer to reality and closer into relationship with our machines. This is deeply important; it is so apparent to me that I feel like justifying it is to undermine a truth that should be obvious. We spend so much time learning special rules for programming language, and are seldom offered exposure under the hood. If we understood more about what happens there, we become more empowered to write our programs how we want, unconstrained by libraries, frameworks, paradigms. Learn how the machine works, and do what you want with it. Right?

Why does garbage collection matter? Because it ties you to a "runtime". Unless we learn alongside the lower machine code, we won't really grasp what a so-called runtime is doing to our program. We will struggle, too, to see why our objects can't be interpreted by another program (this took me a long time to understand logically, and even still I don't feel like that lesson is something I feel in my bones, it's just intellectual).

## Learning how to build higher level languages
I'd love to draw inspiration from Realtalk and dynamicland, and implement higher level language paradigms, all the while while compile to machine code, or at least a VM that can compile into machine code (which could also just mean that our VM compiles to WASM or to Rust).

## Programming types
I'm interested to explore a type system with my assembly Lisp. My intention would be to interleave it between compilation and runtime. I'd like to learn how to do both. And I'd really like to play around with programmable type systems, where we can change it as we wish on the fly. Create types and associate them however we like. 

## Programming language syntax matters less when you have 2d graphics
I want to work with Canvas and then OpenGL, because I am sick and tired of the vestigial mental models offered through HTML+CSS. I want to learn to think about graphics as a programmer. I want to make these things work from my own knowledge. Graphics is hard, and I don't want to hide behind abstractions that grossly limit my ability to play around and understand. They are nice for getting something together quick, but I want deeper.

Now onto PL stuff. A dead-simple syntax is totally fine if you have a graphical interface. Here's the neat thing about graphics, 2d space, and user input within that space: you don't have to parse the positionality of a user's inputs, and you can also *real time verify* that they are constructing valid programs. I mean, it's what IDE's do, except instead of just text we'd have discrete locations (like with an HTML form but adapted to be idiomatic in this context) with different ways of navigating between those locations quickly, instead of the default assumptions we have with text navigation. It's at least worth exploring right? Luna definitely had a good idea with dual representation. And in the end we spit out something like Lisp, or perhaps a more useful data structure format.

## Explore the "buffer" idioms in assembly code
Higher performance code often uses independent buffers instead of the way we're used to programming code. I want to use assemblish output to help people understand why we use buffer-style code.

Beyond this, I'd like to see how compiler optimization could happen with streaming together (implicitly) return types into one mutable buffer.

## Idea: create 2D program flow for optimization
Can we take our high-level AST, show the way that data is hooked up, and at a high-level allow coders to customize the pipeline steps with optimizations? Can we fully separate programming language semantics and performance by keeping the layers literally apart, such that the source code is unchanged, but references and modifications can be made to it to add performance without altering semantics. Are there ways to statically verify the modifications don't alter semantics? Interesting right? This is touching on the "code as a database" things I was thinking about months ago back when I was dreaming up better worlds of programming languages.

## How would I write a lisp parser in ideally imagined pseudocode

```clojure
(context assembly)

(fn main [] (
    (push 3)
    (push 4)
    (add)
    (push 10)
    (add)
    (pop)
))
```
I will note that this is an awful asm program as it has no type information, but let's try to parse it at least.

```
// Goal: be able to take raw text and iterate through every word of that text.

type WordType = Command name | Name name | Number value | List WordType

// Already we see the need to distinguish 0, 1, many. This is a deep concept. Two simple ways to make a list are a bunch of references that hold values and lead to other references, or a contiguous section of memory with the list values spaced out, and so having an enum tag for heterogenous lists (this makes me curious to understand how JS code gets optimized if objects have different sized data).
```