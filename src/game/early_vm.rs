/*
Some ideas I'd like for early VMs.

Add numbers together with a stack machine (with simple IO instr).

Subtract numbers by overflowing add; give optional detour into making binary adders out of
circuits to get a deeper understanding for why processors do this.

Add numbers that are larger than our VM can hold by introducing registers.
Alternatively.


Story setting: an abandoned factory with sentient robots. Theme: non-obsolence. So the
idea is that the robots like improving things, but they don't like replacing things. The
notion of tech being obsolete is considered a dismissive human perspective, and is not
taken seriously by the robots (and is actually a very strong point of contention with humans).
The factor is largely ran down, but some robots are building it back up again, repairing
old machines and computers throughout. They want to protect their friends, so rather than
replacing "outdated" technology, they figure out crafty ways of getting those robots
working in this new world.

This premise gives us some wobble room for justifying writing weird code. The point is that
we're really trying to make it work for these older robots, because they deserve to stay
engaged.

"You can't upgrade a bot without fundamentally changing them. Something deep is lost to us
when none can remember the old ways."

Players will send bytecode over TCP (through localhost) to the running game. This will have
an in-game contextualization (you're remoting into a robot factory).

"I'm a robot, I'm extremely good at doing what I know. But it's hard for me to come up with
stuff I don't know. So you know, if you could give us some ideas every now and then, we'd
appreciate it. You can start with ole' EightBit: we don't know how to get them to count
at our new rate. Instead they keep overflowing to zero. We hooked up some shared RAM that
8-bit can write to, even gave them a way to boost up their numbers to the 32-bits the RAM uses,
but we keep seeing the same thing: 255, then wham, EightBit acts like the number 256 doesn't
even exist. Poor old fool. But we love 'em anyway; we just need them to count the dang things!"
*/

/// So as a very simple first exploration, we want to create a machine to add up
/// to 1000. But we have a problem: our machine will not be able to handle
/// instructions larger than 8-bit. The maximum value of an 8-bit number is 255,
/// and the minimum is 0.
///
/// Eight bit has a spare register on him. He also has 256 bits of addressable
/// memory.
///
/// We have a backlog of orders that we buffered, when you're ready, we'll send
/// them down to ole' EightBit.
struct EightBit {
    r1: u8,
    r2: u8,
    memory: [u8; u8::MAX as usize],
    program: Vec<u8>,
}

impl EightBit {
    fn inc(&mut self, reg: u8) {}
}
