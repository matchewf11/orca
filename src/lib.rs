mod ast;
mod builtin;
mod cli;
mod cursor;
mod env;
mod eval;
mod lexer;
mod parser;
mod prec;
mod repl;
mod token;
mod value;

pub use cli::start;

enum Error {
    Parser(parser::Error),
    Lexer(lexer::Error),
}

fn evaluate(input: &str, env: env::EnvRef) {
    let res = lexer::Lexer::new(input.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::Lexer)
        .and_then(|t| {
            parser::Parser::new(&t)
                .parse_program()
                .map_err(Error::Parser)
        })
        .map(|p| eval::Eval::new(p).eval(env));
    match res {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}"),
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Parser(e) => write!(f, "Parse Error: {e}"),
            Lexer(e) => write!(f, "Lex Error: {e}"),
        }
    }
}
