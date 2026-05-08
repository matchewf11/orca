use crate::{eval::Eval, lexer::Lexer, parser::Parser, env::Env};
use std::{io::{self, Write}, collections::HashMap};

pub fn start() {
    let mut env = Env::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match Lexer::new(input.as_bytes()).collect::<Result<Vec<_>, _>>() {
            Ok(toks) => match Parser::new(&toks).parse_program() {
                Ok(prog) => match Eval::new(prog).eval(&mut env) {
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
