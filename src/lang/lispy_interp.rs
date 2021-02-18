//! Curious to try an imperative, unstructured implementation and without lexing.

use std::mem;

pub fn run() {
    let code = r"
Yes!
";
    interpret(code);
}

mod parsing {
    fn simplest_calculator(code: &str) -> i32 {
        let chars: Vec<char> = code.chars().collect();
        // Use base 10 for our familiar decimal numbering system.
        let num1 = chars[0].to_digit(10).unwrap() as i32;
        let op = chars[1];
        let num2 = chars[2].to_digit(10).unwrap() as i32;

        match op {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            fail => panic!("bad operation: {}", fail),
        }
    }

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
}

fn interpret(code: &str) {}

/// This parser will not be helpful for malformed expressions. For example,
/// parenthesis are allowed inside argument names, so things like h(et(((s)grap
/// are fully valid. Don't write that way...
///
/// Things like this are straightforwardly fixable (add a tokenizing step or use
/// pest and a PEG), but I just want to be quick and dirty right now.
///
/// One interesting design decision I could make is to remove from the language
/// the idea of a "nested" operation, and simply compile nestedness into a
/// top-level operation that precedes the current operation, binding it to a
/// result with an auto-generated name, and having the current operation simply
/// reference it by name.
///
/// I honestly like this idea, it opens up something low-level, makes our
/// language's parsed form something we can interpret more like assembly. This
/// might be in keeping with my design goals. Sounds fun to me. Essentially I'm
/// saying that my language doesn't really have nesting, it just has references,
/// but we let you write nested expressions for convenience, and we can output
/// the unrolled form for you to look at.
fn parse(code_string: &str) -> Vec<Operation> {
    let mut words = code_string.split_whitespace();
    // This is our results buffer.
    let mut operations: Vec<Operation> = Vec::new();
    let mut parens_bal = 0;
    // A stack of the operations that haven't finished building. We push to this
    // stack every time an operation has subexpressions that we need to parse.
    let mut op_stack: Vec<OpBuilder> = Vec::new();
    let mut current_op = OpBuilder::new();

    // I code this as a loop instead of direct iteration just in case I need to
    // get more complex later and do things like backtracking.
    'main: loop {
        // Get next word and end parsing if there isn't any.
        let word = match words.next() {
            Some(val) => val,
            None => return operations,
        };

        // A word beginning with a paren means it's an operation.
        if let Some(word) = word.strip_prefix('(') {
            parens_bal += 1;
            current_op.name = Some(word.to_string());
            // We want to check if this operation *belongs* inside of another
            // operation. This will be true if the balance of parens is higher
            // than 1.
            if parens_bal > 1 {
                // If it does, then we need to pause completing our parsing of
                // the current operation while we figure out this suboperation.
                op_stack.push(current_op);
                current_op = OpBuilder::new();
            }

            // We have the name, now we need to get the arguments (which
            // themselves might be operations). The loop will repeat.
        }
        // A word ending with a paren means it's the last argument to our
        // current operation, and we can wrap up parsing it. Except we have a
        // difficulty here: what if there's multiple parens stacked next to each
        // other? Well, we can check for that I guess.
        else if let Some(word) = word.strip_suffix(')') {
            parens_bal -= 1;
            // Finish up the current op by adding this as its last argument.
            current_op.arguments.push(ValueOrOperation::Val(
                word.trim_end_matches(')').to_string(),
            ));
            // If this ends us being nested, we have a full complete operation
            // to push to our results.
            if parens_bal == 0 {
                operations.push(current_op.to_op());
                current_op = OpBuilder::new();
                continue 'main;
            }
            // If we're still nested, so let's say our word is `20)` in the expr
            // `(hey (man 10 20) god)`, we've finished the current operation,
            // but haven't gotten a top-level operation yet.
            else {
            }

            // Continue checking for parenthesis
            while let Some(word) = word.strip_suffix(')') {}
            // current_op.arguments.push(word);
        }
        // A word without an open paren is just an argument for an operation
        else {
        }
    }
}

/// Expressions always begin and end with parenthesis.
fn is_expr(word: &str) {}

fn parse_expr<'a>(mut expression: impl Iterator<Item = &'a str>) {}

/// Syntax: (name arguments*).
struct Operation {
    name: String,
    arguments: Vec<ValueOrOperation>,
}

enum ValueOrOperation {
    Val(String),
    Op(Operation),
}

struct OpBuilder {
    name: Option<String>,
    arguments: Vec<ValueOrOperation>,
}

impl OpBuilder {
    fn new() -> Self {
        Self {
            name: None,
            arguments: Vec::new(),
        }
    }
}

impl OpBuilder {
    fn to_op(self) -> Operation {
        Operation {
            name: self.name.unwrap(),
            arguments: self.arguments,
        }
    }
}
