use crate::{Eval, Lexer, Parser, env::Env, value::Value};
use std::fs;

pub type Builtin = fn(Value) -> Value;

pub fn find_builtin(name: &str) -> Option<Builtin> {
    match name {
        "println" => Some(builtin_println),
        "require" => Some(builtin_require),
        _ => None,
    }
}

pub fn builtin_println(val: Value) -> Value {
    println!("{val}");
    Value::IO(Box::new(Value::Null))
}

pub fn builtin_require(val: Value) -> Value {
    let Value::String(file_name) = val else {
        return Value::Error(format!("Could not open file with name of {val}"));
    };

    match fs::read_to_string(file_name) {
        Ok(file_contents) => {
            let res = Lexer::new(file_contents.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e: crate::lexer::Error| e.to_string())
                .and_then(|t| {
                    Parser::new(&t)
                        .parse_program()
                        .map_err(|e: crate::parser::Error| e.to_string())
                })
                .map(|p| Eval::new(p).eval(Env::new_wrapped()));
            match res {
                Ok(v) => v,
                Err(e) => Value::Error(e),
            }
        }
        Err(e) => Value::Error(e.to_string()),
    }
}
