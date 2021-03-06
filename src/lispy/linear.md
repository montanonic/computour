# Linear Lisp

I just had the idea of making lisp even more easily bootstrappable into any system, by making a proto-lisp that only requires linear parsing but which can itself implement a lisp parser so that anyone can just implement a (hopefully even simpler, nonrecursive?) parser/interpreter pair for our proto-lisp language, and from it we bootstrap a lisp parser-interpreter! Fun! Of course... this would also work if we just outputted VM code to execute our lisp out of. That's an even more obvious, simple interface to implement...

And excitingly, if we have a VM interpreter, because interpretation is itself slow we could simply use the interpreted side to handle the most basic plumming setup to get the fully compiled version of our language working through FFI. In other words: once the host language writes a VM interpreter, there's a simple codebase for handling FFI that let's us get two-way FFI working at the boundary point. I think that's a super interesting idea worth looking into.

# Other notes

I want to imagine a lower syntax language than Lisp, but somewhat distinguished from Forth and Assembly. The semantics of Forth really interest me, but I'm wondering about making something a little safer and more modern. There's some vestigial things about Forth that make more sense for microcontrollers than they do a modern dev computer like mine, and I *do* think it's meaningful to have something like Forth, but it's not as much in the direction of what I want to focus most heavily on right now.

So, though we may want to liberate ourselves from the von-neuman architecture, we also can't really understand modern computing performance characteristics *without* relying upon such an understanding, as well as things like processor caches, implications of data alignment, branch prediction, et cetera. The short version though is that: contiguous data is better, and breaking things down into small loops that operate on as much of the same data locations in memory for as long as possible is a general win.

So yes, high-level computing is important, but should not exclude also having a more intimate relationship with our computers, because having a concrete, material understanding of what's really happening, is a fundamentally empowering experience. I want then, to keep these semantics in mind as I model out how our linear language might work.

Our goal is to implement a Lisp, for this we'll need to be able to loop, we'll need to be able to manage memory, and we want to support at least ASCII. The first problem I see is: how do we generically deal with numbers? I think the Lisp notion of having each digit be its own symbol, and treating numbers as lists, might be a great way to get started. It is *obviously* slow, but it will at the least be correct. And can support arbitrarily large numbers without any additional conditional logic. We'll get into fast implementations later, but I think that's a fun place to start, at least conceptually. Likewise for strings we'll offer ASCII mode, and unicode support via lists of chars.

One thing that Forth does is it really makes single words very powerful, every word is a computation, and this implicitly lets you get some of syntax of more syntactically complicated languages, purely through computation. For example, if I want to make a command that emulates user(name: "john", age: 23), then I could:
```
: user allocate_space_for_name_and_age ; // returns the location in memory
: name: dup allocate_name ; // allocates at the name location of the pointer
: age: dup allocate_age ; // allocates at the age location of the pointer

// And you'd write
user name: "john" age: 23
// or
user age: 23 name: "john"
```
So that's a really simple example of the expressiveness of Forth and how you can just like manually implement features that make it seem like it can do things that other languages can do. We could even use a secondary calling stack to add some logic to make sure both `name:` and `age:` are called after `user`.

Now this all relies upon the concept of a background stack, and we track these things in our head. The data flow is then is implicit and so we need to have a very obvious and explicit flow of data to reason well. In object oriented programming method chains are an example of a similar form, except that instead of an implicit stack there are objects which contain encapsulated state that may/may-not change between method calls. This loses serious explicitness versus always seeing the data being manipulated in a stack. Elxir pipelines are another example, and it's actually similar to the object case except for that only the data that flows between the pipeline exists, and nothing else occurs except for an output value (which of course is not actually true in elixir because of side-effects, but the state is very much more explicit).

One way to get around the naming problem is literally just having less modular code: then there's less things to name. Sometimes it's just hard to think of good names to given helper steps or functions in a program. Obviously we don't want to sacrifice modularity, but how do we get around function_helper, get_and_preprocess_name_and_age, and other monsters? A more literate programming style would surely help.

I should probably just settle on i32 values and ASCII just for sake of getting going. I can re-implement the Forth later, and honestly if I do it within a Lisp I can easily extend and alter numerical support without having to alter the source Forth, so that's probably a nice (are more interesting!) option.

The main reason function composition is confusing is because it complicates viewing intermediate values of your dataflow. If I can, when programming and debugging, look at each step in the flow of my computation (and of course go further into substeps), I'll have far less confusion and mental modeling need to see how and why my program works. So an interactive program stepper is really important <3.

So consider `square  .  size  .  rest`, the idea is to take a list, skip its first element (getting the rest), getting the size of it, and squaring that size. So classic first issue: we have to read it from right to left which is annoying, but also what if I want to see how each step transforms the input? Pain in the ass in most langs. Let's rewrite as `rest ; size ; square`, or even just `rest size square` since we know they're functions, and now imagine that they are in fact streaming, and we can call a `visualize` function on all of them and either view each intermediate stage of computation, or view the iteration applied element-wise.

Would an effective macro strategy in lisp be to de-nest expression into lines of single-operator statements? The idea would be to use this anytime contextual parsing isn't required. Honestly I really like this idea. Contextual parsing can still be reduced to a list of steps of expressions, but you'd of course need to embellish the expressions with required context, or add expressions in the flow (in other words you might contextual parse an expression with a nested expression into 3+ expressions, with extra ones added to hook in the semantics of what we want to imbue with context; this may be more easily computable, but not necessarily more easily readable).

I want there to be a simpler way of bootstrapping into a higher level programming environment ecosystem. My goal with a linear language is to have dirt-simple parsing that requires no backtracking or lookahead, and a dirt-simple AST that requires no recursion to evaluate.

Let's get Lispy working in Forthy then, yeah? In this implementation, for simplicity, I'll be assuming 32-bit values across the entire system. Strings will be UTF32 encoded, numbers will be 32-bit (signed) integers, pointers 32-bit, and memory storage size 32-bit. This will cut off quite a bit of opportunities for memory efficiency, but will make it easier to get rolling. Future versions should be more flexible, and consult the webassembly spec for inspiration.

I like the idea of having a secondary stack that more closely resembles a standard programming call stack. The default Forth stack is thus for primary program execution flow, and can easily be compiled to fit into registers because the stack stays so narrow, but the secondary stack can be used to store larger amounts of data, like arrays and such, that don't need to be dynamically sized (so, basically all Rust structs). In fact, we can have as many stacks as we want, we just need to add language support for them either externally or internally. Anyways, we needn't overcomplicate.

One thing I'm wondering with Forth is: why not use a base-pointer + offset to access data? And now I'm thinking, hmm, that's actually less great than just using a variable (which gives named access), and then I'm thinking: why not default allocate variables to the stack, right?

Hmm, Forth is expressive yet low-level enough to implement pretty trivially on a microcontroller. It's a super cool language, but look, I'm gonna write a VM and get into assembly stuff soon enough, so I'm thinking I should maybe embellish Forthy with some more features and make it a bit higher level. Like, maybe we'll *not* do raw pointers and the like. I'll take a another stab at a higher level interface. Again, Forthy will be dynamically called, but strongly typed: there will be data types that will crash if the wrong method is called on them. This makes Forthy much more of a DSL than generic lang, but that's fine for now. In Forthy there will be tagged pointers representing our data structures, as we'll need *some* value on the stack to reference them.

```
// We'll want a file api for reading a lispy file
// file/open ( str -- file )
// file/read-line ( file -- )

// next will be our generic name for sequential iteration
: words ( str -- str[] ) next-char

/// Tokenizer.
/// All words are ( str[] -- str[] )
: skip-whitespace next \ws
: ws skip-whitespace ;
: lparen 

// To parse lispy we want to count parens. (+ (* 3 4) 3)
: parse ( str )
```

### How to do a simple dict

The two main problems to solve with a hash-map are (1) what is your hashing algorithm (is it fast and does it provide a nice distribution?), and (2) what is your strategy on collision?


### Forthy combinators:

Via https://factor-language.blogspot.com/search?q=cleave.

"The basic picture is that we have three groups of combinators:
The "cleave" family take m values and n quotations. They apply each quotation to all m values in turn.
The "spread" family take m*n values and n quotations. They apply each quotation to the corresponding group of m values in turn.
The "apply" family take m*n values and 1 quotation. They apply the quotation to each m-element group of values in turn."

"The naming convention is simple. The suffix on the combinator name determines which family it belongs to. No suffix denotes a "cleave", a * suffix denotes a "spread", and a @ suffix denotes an "apply". If the combinator name does not start with a digit, m is 1. Otherwise, the first digit is the value of m. Finally, the rest of the name, except for the prefix and suffix, determines n. This name is either bi, for n=2 or tri, for n=3"

bi: 1 value, 2 quotations, quotation has arity 1
tri: 1 value, 3 quotations, quotation has arity 1
2bi: 2 values, 2 quotations, quotation has arity 2
2tri: 2 values, 3 quotations, quotation has arity 2
3bi: 3 values, 2 quotations, quotation has arity 3
3tri: 3 values, 3 quotations, quotation has arity 3
bi* - 2 values, 2 quotations, quotation has arity 1
tri* - 3 values, 3 quotations, quotation has arity 1
2bi* - 4 values, 2 quotations, quotation has arity 2
bi@ - 2 values, 1 quotation, quotation has arity 1
tri@ - 3 values, 1 quotation, quotation has arity 1
2bi@ - 4 values, 1 quotation, quotation has arity 2

"Note that 2tri*, 3bi* and 3tri* do not exist: if they did, they would take 6, 6 and 9 input values, respectively, and if your code needs that many values on the stack, then you should either use sequences, assocs, tuples, or variables.

Finally, there is a generalized versions of bi and bi*: cleave and spread take a sequence of quotations."
