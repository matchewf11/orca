use std::{env, fs};
use crate::{repl, env::Env, lexer::Lexer, parser::Parser, eval::Eval};

pub fn start() {
    match env::args().skip(1).next() {
        None => repl::start(),
        Some(file_name) => {
            let input = fs::read_to_string(file_name).unwrap();
            let mut env = Env::new();

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

        },
    }
}
