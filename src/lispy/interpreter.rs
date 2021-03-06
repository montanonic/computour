use super::parser::{Expression, Node, Program, Token};
use std::collections::HashMap;

use Value::*;

/*
McCarthy's lisp:
It assumes quote, atom, eq, cons, car, cdr, and cond,
and defines null, and, not, append, list, pair, assoc, eval, evcon and evlis.
*/

pub struct Interpreter<'a> {
    program: &'a Program<'a>,
    definitions: HashMap<&'a str, Value<'a>>,
    results: Vec<(Expression<'a>, Value<'a>)>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program<'a>) -> Self {
        Self {
            program,
            definitions: HashMap::new(),
            results: Vec::new(),
        }
    }

    pub fn interpret(&mut self) {
        for expression in self.program.expressions() {
            let value = self.interpret_expression(expression.nodes());
            self.results.push((expression.clone(), value));
        }
    }

    fn interpret_expression(&mut self, nodes: &[Node<'a>]) -> Value<'a> {
        todo!()
        // match nodes.first().unwrap() {
        //     Node::Expr(expression) => {
        //         self.interpret_expression(expression.nodes());
        //     }
        //     Node::Builtin(Token::Def) => {
        //         let key = &nodes[1].get_ident();
        //         let value = self.interpret_expression(nodes[2].get_expr().nodes());
        //         self.definitions.insert(key, value);
        //         Unit
        //     }
        //     Node::Builtin(Token::Add) => {
        //         self.value = Int64(nodes[1].get_i64() + nodes[2].get_i64());
        //     }
        //     Node::Int64(val) => Int64(*val),
        //     Node::Ident(val) => Ident(*val),
        //     _ => unimplemented!(),
        // }
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
