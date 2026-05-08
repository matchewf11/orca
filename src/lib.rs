mod ast;
mod builtin;
mod cli;
mod cursor;
mod env;
mod eval;
mod lexer;
mod parser;
mod repl;
mod token;
mod value;

pub use cli::start;

use env::EnvRef;
use eval::Eval;
use lexer::Lexer;
use parser::Parser;

use std::fmt;

enum Error {
    Parser(parser::Error),
    Lexer(lexer::Error),
}

fn evaluate(input: &str, env: EnvRef) {
    let res = Lexer::new(input.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::Lexer)
        .and_then(|t| Parser::new(&t).parse_program().map_err(Error::Parser))
        .map(|p| Eval::new(p).eval(env));
    match res {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}"),
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Parser(e) => write!(f, "Parse Error: {e}"),
            Lexer(e) => write!(f, "Lex Error: {e}"),
        }
    }
}
