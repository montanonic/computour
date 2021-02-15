## Drawing from common/modern CPU architectures
I think it's important to keep in mind how our modern era processors *tend* to work; though there's a ton of variation, there are some fairly common trends. By understanding these things, and having some intuition on how this stuff can be implemented in real hardware, I'm able to write programming language code that compiles to assembly for a simple CPU VM that should give me a fairly intuitive understanding of how code like this will run on my actual computer. Going further I can compile my language to Rust code, or my VM assembly to Rust code, and compare the godbolt output to my assembly output to roughly compare the correspondance between these mental models. Even if my code is much slower, as long as it can translate fairly logically to actual CPU instructions, the intuition of "what's really happening" will be preserved.

From this bases then I can more educatively explore alternative CPU models, and dream up non-existant computing models. I can write VM's for these different ideas, and explore different models of programming languages given this freedom of constraint. That said, I still think it will be really important to keep my head in the "now" as well, and not lose sight over how code like that would execute on contemporary machines, not so much to fearfully optimize it under the modern constraints, but to just ensure that my intuition for the actual machines I use today is retained, because as much as it may constrain possibilities to think in that mode, it also gives *reality*, the way these things work *now*, and that sense of immediacy is important in a deep way.

Extensions of this path are to think of communication protocols between machines with different architectures, to dream up a high-level machine language that allows us to express our programs in a much more constrained way than general programming languages, but which could be definitely implementatable across many different architectures. This is in a sense the JVM portability problem, but the difference for me is that I don't want to constrain to the JVM's world of things with classes and the like. I'll likely go for a more mathematical or linguistic approach. Maybe a more automata approach? That's definitely worth looking into too.

One important alternative target, with wildly different characteristics, are quantum computing machines. These will not work the way we're used to, and it would be good practice to understand how a generic machine language would be implementable on such an exotic architecture.

## Bril compiler IR lang
https://capra.cs.cornell.edu/bril/intro.html

Worth checking this out once I get more comfortable with these things, so I can figure out how to improve.

## Popping vs. Stack References 

So, a FORTH-like stack machine, to