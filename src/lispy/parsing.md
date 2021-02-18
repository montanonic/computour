Hi there! In this post, I'm going to do a hands on walkthrough of some parsing essentials for programming languages. We're going to write a basic lisp-style parser (in the Rust programming language), and deeply think about every step involved. I feel that one of the things that we could use more of in education around language parsers is a hands-on intuition for why common parsing techniques are the way they are. I want us to experience together different implementation pitfalls as we go from naive code, up to something that works well and is extensible. I hope you'll enjoy this learning journey alongside me! Please feel welcome to jump ahead in your implementation as you read along, and also to extend things in your own ways! Have fun!

## Building an intuition for what parsing is

In the context of programming languages, parsing is basically the act of turning a *string* that contains code in it (so, any code source file ever), into data structures that can be more easily worked with to evaluate the ideas expressed in the "source code". In the fashion of learning what something is by learning what it is not, I want to look at something so simple that we might not even be sure if we're actually parsing anything.

Consider a calculator programming language that only supported a *single* addition, subtraction, multiplication, or division on single-digit positive numbers. This is hilariously restrictive. Here's an example program:
```
3*4
```
Yeah seriously, that's it. From this example, we can see that our grammar/syntax is: a digit, followed by an operation (`+`, `-`, `*`, `/`), followed by a digit. Let's write an *interpreter* for this. We want a function that takes in calculator code in the form of a string, returns the result of the calculation as an `i32` (though you certaintly can pick a smaller (or larger) number type if you want). I'm going to add `mod calculator;` in my `src/main.rs` file, create `calculator.rs` in `src/`, and add the code there.
```rust
fn simplest_calculator(code: &str) -> i32 {
    let chars: Vec<char> = code.chars().collect();
```
My interpretation strategy will just be to index into the string. But! Because Rust strings are UTF-8, in which a single character can be anywhere from 1 to 4 bytes long, Rust doesn't allow for naive indexing. So we call the `chars` method and collect them into a `Vec` that we can now index into. Our next lines:
```rust
    // Use base 10 for our familiar decimal numbering system.
    let num1 = chars[0].to_digit(10).unwrap() as i32;
    let op = chars[1];
    let num2 = chars[2].to_digit(10).unwrap() as i32;
```
Here we get a bit of Rust's explicitness interrupting our ergonomics. First, not all unicode characters are digits, so `to_digit` returns `Option<u32>` instead of `u32`. Second, we have to give a base for how to interpret our digit, so base 10. Third, we want to work with integers because even though we only support positive number inputs, things like `3-9` is `-6`, so our output can still be a number; we safely convert our positive `u32` values into `i32`s.

We could call what we just did parsing, as trivial as it is. Our "data structures" are: two numbers, and one string representing our operation. Let's finish off the code with our interpreter:
```rust
fn simplest_calculator(code: &str) -> i32 {
    let chars: Vec<char> = code.chars().collect();
    // Use base 10 for our familiar decimal numbering system.
    let num1 = chars[0].to_digit(10).unwrap() as i32;
    let op = chars[1];
    let num2 = chars[2].to_digit(10).unwrap() as i32;

    // "interpreter"
    match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        fail => panic!("bad operation: {}", fail),
    }
}
```
Let's start a test suite in our same file to verify our implementation.
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simplest_calculator_works() {
        let code = "9+3";
        assert_eq!(simplest_calculator(code), 12);
        let code = "5-9";
        assert_eq!(simplest_calculator(code), -4);
        let code = "7*7";
        assert_eq!(simplest_calculator(code), 49);
        let code = "5/3";
        assert_eq!(simplest_calculator(code), 1);
    }
}
```
As terribly boring as that might have been for you, what we've just done is the essence of interpretation, and to properly interpret the calculator code, we had to break down the string into the individual components we wanted to work with. Our interpreter demanded 3 different data structures: the first number, the second number, and their operation. The code at the beginning of our interpreter is the parsing phase, the `match op` part is our interpreting phase.

If you're interested in extending the calculator example,obvious areas to improve are: allow whitespace, multiple-digit numbers, negative numbers, and support for multiple operations `"3 + 4 * 9 - 1"`. Check out the standard docs and try out `String::split_whitespace` instead of `String::chars`! If you've gone through that gamut before, or just want to cut your teeth on something harder, don't worry, up next there's *much* harder interpretation problem!

## The challenge of parsing

Here's some really basic Rust code I want us to think about parsing:

```rust
pub fn double_all(data: &[i32]) -> Vec<i32> {
    let mut output = Vec::with_capacity(data.len());
    for x in data {
        output.push(x * 2);
    }
    output
}
```
Yeah, so, wow, there's a million more things going on there.

```clojure
(annotate double_all {:visibility public, :})
(fn double_all (data) 
    (mut-let output (vec::with_capacity (len data))
        (for x data 
            (push output (* x 2))
        )
    )
    data
)
```

A lispy syntax version

Now, in this guide, we're only going to be focusing on lisp *syntax*; I leave you to dream up your own keywords to implement!



I want to compare lispy syntax to Rust's. If you're not already familiar with any lisp languages, these comparisons (while not teaching you any "real" lisp implementation) should help make sense of their core syntax style! (aside: Lisp has a fascinating history; if you're interested in learning more, here's [SOME LINKS])


## Why lisp? (and what is it?)

I've long been super interested in programming language theory and implementation, and while syntax is important, I personally want to be spending my time thinking about language features as much as possible, and leave the art of figuring out a good syntax for a later time. The lisp language, originally devised in a math paper by John McCarthy in the early 60s, is very well-suited to my preferences. Let's see how!
