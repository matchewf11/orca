use crate::{
    ast::{Expr, InfixOp, PrefixOp, Program, Stmt},
    builtin::find_builtin,
    env::{Env, EnvRef},
    error,
    value::Value,
};

#[derive(Debug)]
pub struct Eval(Program);

impl Eval {
    pub fn new(prog: Program) -> Self {
        Self(prog)
    }

    pub fn eval(self, env: EnvRef) -> Value {
        let mut return_val = Value::Null;

        for stmt in self.0 {
            return_val = match Self::eval_stmt(stmt, env.clone()) {
                Value::Error(e) => return Value::Error(e),
                v => v,
            };
        }

        return_val
    }

    fn eval_stmt(stmt: Stmt, env: EnvRef) -> Value {
        use Stmt::*;
        match stmt {
            Expr(e) => Self::eval_expr(e, env),
            Bind(n, e) => {
                let expr = Self::eval_expr(e, env.clone());
                if matches!(expr, Value::Error(_)) {
                    expr
                } else {
                    env.borrow_mut().insert(n, expr);
                    Value::Null
                }
            }
        }
    }

    fn eval_expr(expr: Expr, env: EnvRef) -> Value {
        use Expr::*;
        use PrefixOp::*;
        match expr {
            Int(i) => Value::Int(i),
            String(i) => Value::String(i),
            Null => Value::Null,
            Bool(b) => Value::Bool(b),
            Var(n) => env
                .borrow()
                .get(&n)
                .unwrap_or(find_builtin(&n).map(Value::Builtin).unwrap_or(Value::Null)),
            If(cond, a, b) => match Self::eval_expr(*cond, env.clone()) {
                Value::Bool(true) => Self::eval_expr(*a, env),
                Value::Bool(false) => Self::eval_expr(*b, env),
                v => error!("If Statement Condition can't be: {v}"),
            },
            Prefix(op, arg) => match (*op, Self::eval_expr(*arg, env.clone())) {
                (Neg, Value::Int(n)) => Value::Int(-n),
                (Not, Value::Bool(n)) => Value::Bool(!n),
                (Call(fun), call_arg) => match Self::eval_expr(fun, env) {
                    Value::Builtin(b) => b(call_arg),
                    Value::Fn(fn_arg_name, fn_body, fn_env) => {
                        let fn_env = Env::new_inner_wrapped(fn_env);
                        fn_env
                            .borrow_mut()
                            .insert(fn_arg_name.to_string(), call_arg);
                        Self::eval_expr(fn_body, fn_env)
                    }
                    v => error!("Need a function: {v}"),
                },
                (op, val) => error!("Can't use '{op}' for value: {val}"),
            },
            Infix(lhs, op, rhs) => match (*lhs, op, *rhs) {
                (Expr::Var(s), InfixOp::Arrow, rhs) => Value::Fn(s, rhs, Env::branch(env)),
                (lhs, op, rhs) => {
                    let lhs = Self::eval_expr(lhs, env.clone());
                    let rhs = Self::eval_expr(rhs, env);
                    let err = error!("{op} not supported for {lhs} and {rhs}");
                    match (lhs.clone(), op.clone(), rhs.clone()) {
                        (Value::Int(n), InfixOp::Eq, Value::Int(m)) => Value::Bool(n == m),
                        (Value::Int(..), InfixOp::Eq, _) => err,
                        (Value::Bool(n), InfixOp::Eq, Value::Bool(m)) => Value::Bool(n == m),
                        (Value::Bool(..), InfixOp::Eq, _) => err,
                        (Value::String(n), InfixOp::Eq, Value::String(m)) => Value::Bool(n == m),
                        (Value::String(_), InfixOp::Eq, _) => err,
                        (Value::Null, InfixOp::Eq, Value::Null) => Value::Bool(true),
                        (Value::Null, InfixOp::Eq, _) => err,
                        (Value::Fn(..), InfixOp::Eq, _) => err,
                        (Value::Error(..), InfixOp::Eq, _) => err,
                        (Value::IO(..), InfixOp::Eq, _) => err,
                        (Value::Builtin(..), InfixOp::Eq, _) => err,

                        (Value::Int(n), InfixOp::NEq, Value::Int(m)) => Value::Bool(n != m),
                        (Value::Int(..), InfixOp::NEq, _) => err,
                        (Value::Bool(n), InfixOp::NEq, Value::Bool(m)) => Value::Bool(n != m),
                        (Value::Bool(..), InfixOp::NEq, _) => err,
                        (Value::String(n), InfixOp::NEq, Value::String(m)) => Value::Bool(n != m),
                        (Value::String(..), InfixOp::NEq, _) => err,
                        (Value::Null, InfixOp::NEq, Value::Null) => Value::Bool(false),
                        (Value::Null, InfixOp::NEq, _) => err,
                        (Value::Fn(..), InfixOp::NEq, _) => err,
                        (Value::Error(..), InfixOp::NEq, _) => err,
                        (Value::IO(..), InfixOp::NEq, _) => err,
                        (Value::Builtin(..), InfixOp::NEq, _) => err,

                        (Value::Int(n), InfixOp::Add, Value::Int(m)) => Value::Int(n + m),
                        (Value::Int(..), InfixOp::Add, _) => err,
                        (Value::String(n), InfixOp::Add, Value::String(m)) => Value::String(n + &m),
                        (Value::String(..), InfixOp::Add, _) => err,
                        (Value::Bool(..), InfixOp::Add, _) => err,
                        (Value::Fn(..), InfixOp::Add, _) => err,
                        (Value::Error(..), InfixOp::Add, _) => err,
                        (Value::Null, InfixOp::Add, _) => err,
                        (Value::IO(..), InfixOp::Add, _) => err,
                        (Value::Builtin(..), InfixOp::Add, _) => err,

                        (Value::Int(n), InfixOp::Sub, Value::Int(m)) => Value::Int(n - m),
                        (Value::Int(..), InfixOp::Sub, _) => err,
                        (Value::Bool(..), InfixOp::Sub, _) => err,
                        (Value::Fn(..), InfixOp::Sub, _) => err,
                        (Value::Error(..), InfixOp::Sub, _) => err,
                        (Value::Null, InfixOp::Sub, _) => err,
                        (Value::IO(..), InfixOp::Sub, _) => err,
                        (Value::Builtin(..), InfixOp::Sub, _) => err,
                        (Value::String(..), InfixOp::Sub, _) => err,

                        (Value::Int(n), InfixOp::Mul, Value::Int(m)) => Value::Int(n * m),
                        (Value::Int(..), InfixOp::Mul, _) => err,
                        (Value::String(..), InfixOp::Mul, _) => err,
                        (Value::Bool(..), InfixOp::Mul, _) => err,
                        (Value::Fn(..), InfixOp::Mul, _) => err,
                        (Value::Error(..), InfixOp::Mul, _) => err,
                        (Value::Null, InfixOp::Mul, _) => err,
                        (Value::IO(..), InfixOp::Mul, _) => err,
                        (Value::Builtin(..), InfixOp::Mul, _) => err,

                        (Value::Int(n), InfixOp::Div, Value::Int(m)) => Value::Int(n / m),
                        (Value::Int(..), InfixOp::Div, _) => err,
                        (Value::String(..), InfixOp::Div, _) => err,
                        (Value::Bool(..), InfixOp::Div, _) => err,
                        (Value::Fn(..), InfixOp::Div, _) => err,
                        (Value::Error(..), InfixOp::Div, _) => err,
                        (Value::Null, InfixOp::Div, _) => err,
                        (Value::IO(..), InfixOp::Div, _) => err,
                        (Value::Builtin(..), InfixOp::Div, _) => err,

                        (Value::Int(n), InfixOp::Mod, Value::Int(m)) => Value::Int(n % m),
                        (Value::Int(..), InfixOp::Mod, _) => err,
                        (Value::String(..), InfixOp::Mod, _) => err,
                        (Value::Bool(..), InfixOp::Mod, _) => err,
                        (Value::Fn(..), InfixOp::Mod, _) => err,
                        (Value::Error(..), InfixOp::Mod, _) => err,
                        (Value::Null, InfixOp::Mod, _) => err,
                        (Value::IO(..), InfixOp::Mod, _) => err,
                        (Value::Builtin(..), InfixOp::Mod, _) => err,

                        // TODO
                        (Value::Int(n), InfixOp::Gt, Value::Int(m)) => Value::Bool(n > m),
                        (Value::Int(n), InfixOp::Gte, Value::Int(m)) => Value::Bool(n >= m),
                        (Value::Int(n), InfixOp::Lt, Value::Int(m)) => Value::Bool(n < m),
                        (Value::Int(n), InfixOp::Lte, Value::Int(m)) => Value::Bool(n <= m),
                        (Value::Int(n), InfixOp::Exp, Value::Int(m)) => Value::Int(n.pow(m as u32)),
                        (Value::Bool(n), InfixOp::And, Value::Bool(m)) => Value::Bool(n && m),
                        (Value::Bool(n), InfixOp::Or, Value::Bool(m)) => Value::Bool(n || m),
                        (_, InfixOp::Gte, _) => err,
                        (_, InfixOp::Gt, _) => err,
                        (_, InfixOp::Lte, _) => err,
                        (_, InfixOp::Lt, _) => err,
                        (_, InfixOp::And, _) => err,
                        (_, InfixOp::Or, _) => err,
                        (_, InfixOp::Exp, _) => err,
                        (_, InfixOp::Arrow, _) => err,
                        (_, InfixOp::Dot, _) => err,
                        (_, InfixOp::Dollar, _) => err,
                        (_, InfixOp::Pipe, _) => err,
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests;
