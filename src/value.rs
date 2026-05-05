use crate::ast::Expr;

type Params = Vec<String>;

pub enum Value {
    Int(i64),
    Fn(Params, Expr),
}
