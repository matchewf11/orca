use crate::{error, env::Env, value::Value, parser::{Parser, Error as ParseError}, eval::Eval, lexer::{Lexer, Error as LexError}};
use std::fs;

pub type Builtin = fn(Value) -> Value;

pub fn find_builtin(name: &str) -> Option<Builtin> {
    Some(match name {
        "println" => builtin_println,
        "require" => builtin_require,
        _ => return None,
    })
}

pub fn builtin_println(val: Value) -> Value {
    println!("{val}");
    Value::IO(Box::new(Value::Null))
}

pub fn builtin_require(val: Value) -> Value {
    let Value::String(file_name) = val else {
        return error!("Could not open file with name of {val}");
    };

    match fs::read_to_string(file_name) {
        Ok(file_contents) => {
            let res = Lexer::new(file_contents.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e: LexError| e.to_string())
                .and_then(|t| {
                    Parser::new(&t)
                        .parse_program()
                        .map_err(|e: ParseError| e.to_string())
                })
                .map(|p| Eval::new(p).eval(Env::new_wrapped()));
            match res {
                Ok(v) => v,
                Err(e) => error!("{e}"),
            }
        }
        Err(e) => error!("{e}"),
    }
}
