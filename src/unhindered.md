# How would I write a lisp parser in ideally imagined pseudocode

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