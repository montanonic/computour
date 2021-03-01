# Parsing

## Easier alternative?

What about a bubbling out strategy? We first do a full pass of our source code, counting our paren balance along the way, and recording the location of the max balance. Parsing from that nested expression will be the most depthful, but now that I think about it that won't help at all with general parsing. If this is the highest hill, there are still likely to be many local maximums, so this strategy wouldn't really give us much. What about if we went in tiers though, from highest count down? Doing this would ensure that for every surrounding parenthesis we parse, any terms inside are guaranteed to be non-parenthesis tokens, or expressions. Worth thinking about.

But here's an alternative stack-based impl. Everytime our paren balence increases, our previous expression didn't finish forming, and we should push it onto our stack and continue. Everytime our paren balance decreases, the last expression on the stack has matched, and can be formed. If we form it, and pop it off the stack into a sub expression of the next expression, we'd be able to finish.