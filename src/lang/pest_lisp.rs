use pest::Parser;
#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct LispParser;

pub fn main() {
    let parse = parse_program(r#"(+ 1 2)"#);
    println!("{:#?}", parse);
}

#[derive(Debug, PartialEq, Eq)]
struct Expression<'a> {
    identifier: &'a str,
    args: Vec<LispyValue<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
enum LispyValue<'a> {
    Number(i64),
    Word(&'a str),
}

fn parse_program<'a>(code_str: &'a str) -> Vec<Expression<'_>> {
    let pairs = LispParser::parse(Rule::program, code_str).unwrap();

    use pest::iterators::Pair;

    let mut exprs = Vec::new();
    let mut parse_expr = |pair: Pair<'a, Rule>| match pair.as_rule() {
        Rule::expr => {
            let mut pairs = pair.into_inner();

            let pair = pairs.next().unwrap();
            let identifier = match pair.as_rule() {
                Rule::operator => pair.as_str(),
                Rule::word => pair.as_str(),
                _ => unimplemented!(),
            };

            let mut expr = Expression {
                identifier,
                args: Vec::new(),
            };

            for pair in pairs {
                let val = match pair.as_rule() {
                    Rule::word => LispyValue::Word(pair.as_str()),
                    Rule::number => LispyValue::Number(pair.as_str().parse().unwrap()),
                    _ => unimplemented!(),
                };
                expr.args.push(val);
            }

            exprs.push(expr);
        }
        Rule::EOI => (),
        r => unimplemented!("{:?}", r),
        _ => unimplemented!(),
    };

    for pair in pairs {
        parse_expr(pair);
    }

    exprs
}

#[cfg(test)]
mod tests {
    use super::*;
    use LispyValue::*;
    #[test]
    fn ops() {
        let pgm = parse_program("(+ 1 2)");
        assert_eq!(
            pgm[0],
            Expression {
                identifier: "+",
                args: vec![Number(1), Number(2)]
            }
        );

        let pgm = parse_program("(+= x 2)");
        assert_eq!(
            pgm[0],
            Expression {
                identifier: "+=",
                args: vec![Word("x"), Number(2)]
            }
        );
    }

    #[test]
    fn words() {
        let pgm = parse_program("(add 97 12)");
        assert_eq!(
            pgm[0],
            Expression {
                identifier: "add",
                args: vec![Number(97), Number(12)]
            }
        );

        let pgm = parse_program("(jmp0 x 1 2)");
        assert_eq!(
            pgm[0],
            Expression {
                identifier: "jmp0",
                args: vec![Word("x"), Number(1), Number(2)]
            }
        );
    }
}
