use crate::{
    ast::{Program, Stmt, Expr, InfixOp},
    value::Value
};

pub struct Eval(Program);

impl Eval {
    pub fn new(prog: Program) -> Self {
        Eval(prog)
    }

    pub fn eval(prog: Program) -> Option<Value> {
        prog.0
            .iter()
            .map(|s| Self::eval_stmt(s))
            .last()
            .flatten()
    }

    pub fn eval_stmt(stmt: &Stmt) -> Option<Value> {
        match stmt {
            Stmt::Bind(..) => None,
            Stmt::Expr(e) => Some(Self::eval_expr(e)),
        }
    }

    pub fn eval_expr(expr: &Expr) -> Value {
        match expr {
            Expr::Int(n) => Value::Int(*n),
            Expr::Infix(lhs, op, rhs) => {
                let lhs = Self::eval_expr(lhs);
                let rhs = Self::eval_expr(rhs);
                match (lhs, op, rhs) {
                    (Value::Int(l), InfixOp::Add, Value::Int(r)) => Value::Int(l + r),
                    _ => todo!(),
                }
            }
            Expr::Var(..) => todo!(),
            Expr::Call(..) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {}
}
