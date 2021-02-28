use crate::monkey::lexer::Lexer;
use std::io::{prelude::*, BufReader};

pub fn start(input: impl Read, mut out: impl Write) {
    let mut br = BufReader::new(input);

    loop {
        write!(out, ">> ");
        out.flush();

        let mut line = String::new();
        br.read_line(&mut line);

        let lexer = Lexer::new(&line);

        writeln!(out, "{:?}", lexer.collect::<Vec<_>>());
    }
}
