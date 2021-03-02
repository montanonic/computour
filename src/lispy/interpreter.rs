use super::parser::{Expression, Node, Program};

pub struct Interpreter<'a> {
    program: &'a Program<'a>,
}

impl<'a> Interpreter<'a> {
    fn new(program: &'a Program<'_>) -> Self {
        Self { program }
    }

    // fn interpret(&mut self) {
    //     for expression in self.program.expressions() {
    //         for node in expression.nodes() {
    //             Self::interpret_recur(node);
    //         }
    //     }
    // }

    // fn interpret_recur(node: &Node<'_>) {
    //     use Node::*;
    //     match node {
    //         Expr(expression) => {
    //             for node in expression.nodes() {
    //                 Self::interpret_recur(node)
    //             }
    //         }
    //         Ident(name) => {

    //         }
    //     }
    // }
}
