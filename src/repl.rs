use crate::{eval::Eval, lexer::Lexer, parser::Parser};
use std::io::{self, Write};

pub fn start() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match Lexer::new(input.as_bytes()).collect::<Result<Vec<_>, _>>() {
            Ok(toks) => match Parser::new(&toks).parse_program() {
                Ok(prog) => match Eval::new(prog).eval() {
                    Ok(Some(v)) => println!("{v}"),
                    Ok(None) => println!(),
                    Err(e) => println!("Error Ocurred: {e}"),
                },
                Err(e) => println!("Error Ocurred: {e}"),
            },
            Err(e) => println!("Error Ocurred: {e}"),
        }
    }
}
