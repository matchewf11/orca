use crate::{ast::Expr, builtin::Builtin, env::Env};
use std::{cell::RefCell, fmt, ptr, rc::Rc};

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::value::Value::Error(format!($($arg)*))
    };
}

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
            Builtin(v) => write!(f, "Bultin: {v:?}"),
            String(v) => write!(f, "\"{v}\""),
        }
    }
}

// Not to be used for interpreter stuff
// jsut for testing purposes
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(a), Int(b)) => a == b,
            (Int(_), _) => false,
            (Bool(a), Bool(b)) => a == b,
            (Bool(_), _) => false,
            (String(a), String(b)) => a == b,
            (String(_), _) => false,
            (Null, Null) => true,
            (Null, _) => false,
            (IO(a), IO(b)) => a == b,
            (IO(..), _) => false,
            (Error(a), Error(b)) => a == b,
            (Error(..), _) => false,
            (Fn(a1, e1, _), Fn(a2, e2, _)) => a1 == a2 && e1 == e2,
            (Fn(..), _) => false,
            (Builtin(a), Builtin(b)) => ptr::fn_addr_eq(*a, *b),
            (Builtin(_), _) => false,
        }
    }
}
