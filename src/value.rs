use crate::ast::Expr;
use std::fmt;

type Params = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Fn(Params, Expr),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Int(i) => write!(f, "{i}"),
            Bool(i) => write!(f, "{i}"),
            Fn(p, e) => write!(f, "[{}] => {e}", p.join(",")),
        }
    }
}
