use std::fmt;
use crate::{env::Env, ast::Expr};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Fn(String, Expr, Env),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Int(i) => write!(f, "{i}"),
            Bool(i) => write!(f, "{i}"),
            Fn(a, e , env) => write!(f, "()"),
        }
    }
}
