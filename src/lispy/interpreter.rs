use super::parser::{Expression, Node, Program, Token};
use std::collections::HashMap;

use Value::*;

pub struct Interpreter<'a> {
    program: &'a Program<'a>,
    definitions: HashMap<&'a str, Value<'a>>,
    /// Workspace value used to make control flow easier.
    value: Value<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program<'a>) -> Self {
        Self {
            program,
            definitions: HashMap::new(),
            value: Unit,
        }
    }

    pub fn interpret(&mut self) {
        for expression in self.program.expressions() {
            self.interpret_nodes(expression.nodes());
        }
    }

    fn interpret_nodes(&mut self, nodes: &[Node<'a>]) {
        match nodes.first().unwrap() {
            Node::Builtin(Token::Def) => {
                let key = &nodes[1].get_ident();
                self.interpret_nodes(nodes[2].get_expr().nodes());
                self.definitions.insert(key, self.value);
            }
            Node::Builtin(Token::Add) => {
                // self.interpret_nodes(&nodes[1..2]);
                // let n1 = self.value;
                self.value = Int64(nodes[1].get_i64() + nodes[2].get_i64());
            }
            Node::Builtin(Token::Print) => {
                println!("{:?}", self.lookup(nodes[1].get_ident()));
            }
            Node::Int64(val) => self.value = Int64(*val),
            Node::Expr(expression) => {
                self.interpret_nodes(expression.nodes());
            }
            Node::Ident(val) => self.value = Ident(*val),
            _ => unimplemented!(),
        }
    }

    fn lookup(&self, key: &str) -> Option<&Value<'_>> {
        self.definitions.get(key)
    }

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

#[derive(Debug, Clone, Copy)]
enum Value<'ident> {
    Unit,
    Int64(i64),
    Ident(&'ident str),
}
