//! I'm abandoning this attempt towards a recursive approach in favor of a
//! stack-pased parse.

// fn parse(source: &str) {
//     let tokenizer = Tokenizer::new(source);
//     let tokens = tokenizer.tokenize();
//     let mut expressions = Vec::new();

//     let mut i = 0;
//     while i < tokens.len() {
//         use Token::*;

//         match tokens[i] {
//             LParen => {
//                 let expr_tokens = extract_expression(&tokens[i..])
//                     .expect(&format!("parens weren't balanced at token #{}", i));

//                 let mut expr = Expr { tokens: expr_tokens, nested: None};
//                 expr.nested = handle_nested_expressions(expr_tokens);
//                 expressions.push(expr);
//                 i += expr_tokens.len();
//             }
//             RParen => panic!("extract_expression handles all well-formed parens, starting with a LParen. If you matched an RParen, your program is malformed."),
//             _ => panic!("expected expression")
//         }
//     }
// }

// fn parse_expression(
//     tokens: &[Token<'_>],
//     expressions: &mut Vec<Expr<'_>>,
// ) -> Result<(), &'static str> {
//     let mut i = 0;
//     while i < tokens.len() {
//         use Token::*;

//         match tokens[i] {
//             LParen => {
//                 let expr_tokens = extract_expression(&tokens[i..])
//                     .expect(&format!("parens weren't balanced at token #{}", i));

//                 let mut expr = Expr { tokens: expr_tokens, nested: None};
//                 expr.nested = handle_nested_expressions(expr_tokens);
//                 expressions.push(expr);
//                 i += expr_tokens.len();
//             }
//             RParen => return Err("extract_expression handles all well-formed parens, starting with a LParen. If you matched an RParen, your program is malformed."),
//             _ => return Err("expected expression")
//         }
//     }
//     Ok(())
// }

// fn handle_nested_expressions<'a>(
//     expr_tokens: &'a [Token<'a>],
// ) -> Result<Vec<Expr<'a>>, &'static str> {
//     let possible_next_expr = expr_tokens
//         .iter()
//         .skip_while(|&&token| token != Token::LParen)
//         .collect();

//     let mut nested_expressions = Vec::new();
//     parse_expression(possible_next_expr, &mut nested_expressions)?;
//     Ok(nested_expressions)
// }
