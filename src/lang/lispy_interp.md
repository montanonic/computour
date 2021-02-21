# Parsing

## Not using tokenization

I was really curious to just try out a raw loop implementation of my parser, but this makes it more cumbersome to "sanitize" inputs to not be insane. So for example, `asd(4124)((3` is a perfectly valid word in my original parser. Tokenizing means we get to just focus on ensuring sane inputs.

## Tokenizing by word rather than character

I thought it'd be easier to use something like `split_whitespace`, but then what about things like `(yes)` right? I'd have to split into that yet again, whereas going character by character means that something like that would have trivially been split into tokens.

## Unique challenges of unicode

Rust doesn't hide what's happening under the hood from us. So if I want to avoid allocations, I want to reference our original string slice by pointer rather than allocating new strings; great! But if I want to do analysis on characters I need to use chars, 4byte sized code points. 