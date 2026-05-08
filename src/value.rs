use crate::{ast::Expr, builtin::Builtin, env::Env};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Fn(String, Expr, Rc<RefCell<Env>>),
    Error(String),
    Null,
    IO(Box<Value>),
    Builtin(Builtin),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Int(i) => write!(f, "{i}"),
            Bool(i) => write!(f, "{i}"),
            Fn(a, e, _) => write!(f, "({a} => {e})"),
            Error(e) => write!(f, "Error: {e}"),
            Null => write!(f, "null"),
            IO(v) => write!(f, "(IO {v})"),
            Builtin(v) => write!(f, "{v:?}"),
            String(v) => write!(f, "\"{v}\""),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(a), Int(b)) => a == b,
            (Int(_), _) => false,
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (String(a), _) => false,
            (Bool(_), _) => false,
            (Error(a), Error(b)) => a == b,
            (Error(..), _) => false,
            (Fn(a1, e1, _), Fn(a2, e2, _)) => a1 == a2 && e1 == e2,
            (Fn(..), _) => false,
            (Null, Null) => true,
            (Null, _) => false,
            (IO(a), IO(b)) => a == b,
            (IO(..), _) => false,
            (Builtin(_), _) => false,
        }
    }
}
