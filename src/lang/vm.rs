pub fn main() {}

/// I feel it's important to understand how any programming language semantics
/// translate to modern CPU architectures, at least at a mid-to-high-level.
/// Generally speaking we can assume our CPU computes one instruction per cycle
/// (albeit with each instruction not itself finishing on the very next cycle;
/// see pipelining), that our CPU relies upon RAM, that our CPU uses a
/// multi-tiered cache to avoid lookups to the RAM. One another important thing
/// to remember is that instructions that either contain the values we wish to
/// operate on, or which use *registers*, are the most performant (and by a
/// large margin).
///
/// So our memory falls into 3 high-level forms (unless we need to talk about
/// persistence). There's RAM, which is much larger and much slower, the cache,
/// which can be filled with values from RAM as they're looked up but which is
/// much smaller, and there's registers, which basically amounts to an even
/// tinier amount of memory but it's super duper fast to access. The more of
/// your problem that is expressible in terms of registers (or immediate
/// instruction values), the faster it will be.
///
/// The core takeaway should be this: generally speaking, the fewer instructions
/// required to complete a task the better, the more concentrated your memory is
/// the faster (cache lookups, easy to iterate contiguously through RAM), and
/// the more you can re-use memory locations the faster (either because you'll
/// be hitting the cache, or able to assign your scratch space to be in
/// registers).
///
/// I'm curious to compare the performance between stack access with pushing and
/// popping, heap access, and register access. I can probably write some Rust
/// code that godbolt compiles to those different versions, and then compare
/// benchmarks.
struct Modern {}

/// The instructions we have here are very simple, almost maddeningly so. We
/// parameratize over the numeric type to give us greater ease in creating new
/// types of instructions.
enum Instruction<Number> {
    Add(Number, Number),
}
