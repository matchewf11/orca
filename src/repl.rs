use crate::{lexer::Lexer, parser::Parser};
use std::io;

pub fn start() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let lex = Lexer::new(input.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let par = Parser::new(&lex).parse_program();
        println!("{par:?}");
    }
}
