use crate::{
    ast::{Expr, InfixOp, PrefixOp, Program, Stmt},
    value::Value,
    env::Env,
};
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub enum Error {
    InvalidType,
    NullVar(String),
}

#[derive(Debug)]
pub struct Eval(Program);

impl Eval {
    pub fn new(prog: Program) -> Self {
        Self(prog)
    }

    pub fn eval<'a>(&self, env: &mut Env) -> Result<Option<Value>, Error> {
        let mut return_val = None;
        for stmt in &self.0 {
            return_val = Self::eval_stmt(stmt, env)?;
        }
        Ok(return_val)
    }

    fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Result<Option<Value>, Error> {
        use Stmt::*;
        match stmt {
            Expr(e) => Self::eval_expr(e, env).map(Some),
            Bind(n, e) => {
                env.insert(n.to_string(), Self::eval_expr(e, env)?);
                Ok(None)
            }
        }
    }

    fn eval_expr(expr: &Expr, env: &Env) -> Result<Value, Error> {
        use Expr::*;
        match expr {
            Int(i) => Ok(Value::Int(*i)),
            Bool(i) => Ok(Value::Bool(*i)),
            If(cond, a, b) => {
                let cond = Self::eval_expr(cond, env)?;
                match cond {
                    Value::Bool(true) => Ok(Self::eval_expr(a, env)?),
                    Value::Bool(false) => Ok(Self::eval_expr(b, env)?),
                    _ => return Err(Error::InvalidType),
                }
            }
            Infix(lhs, op, rhs) => {
                match (lhs.as_ref(), op) {
                    (Expr::Var(s), InfixOp::Arrow) => {
                        return Ok(Value::Fn(s.to_string(), *rhs.clone(), env.clone()));
                    },
                    _ => (),
                }

                let lhs = Self::eval_expr(lhs, env)?;
                let rhs = Self::eval_expr(rhs, env)?;
                match (lhs, op, rhs) {
                    (Value::Bool(n), InfixOp::And, Value::Bool(m)) => Ok(Value::Bool(n && m)),
                    (_, InfixOp::And, _) => Err(Error::InvalidType),
                    (Value::Bool(n), InfixOp::Or, Value::Bool(m)) => Ok(Value::Bool(n || m)),
                    (_, InfixOp::Or, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Lte, Value::Int(m)) => Ok(Value::Bool(n <= m)),
                    (_, InfixOp::Lte, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Gte, Value::Int(m)) => Ok(Value::Bool(n >= m)),
                    (_, InfixOp::Gte, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Lt, Value::Int(m)) => Ok(Value::Bool(n < m)),
                    (_, InfixOp::Lt, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Gt, Value::Int(m)) => Ok(Value::Bool(n > m)),
                    (_, InfixOp::Gt, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Add, Value::Int(m)) => Ok(Value::Int(n + m)),
                    (_, InfixOp::Add, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Sub, Value::Int(m)) => Ok(Value::Int(n - m)),
                    (_, InfixOp::Sub, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Mul, Value::Int(m)) => Ok(Value::Int(n * m)),
                    (_, InfixOp::Mul, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Div, Value::Int(m)) => Ok(Value::Int(n / m)),
                    (_, InfixOp::Div, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Mod, Value::Int(m)) => Ok(Value::Int(n % m)),
                    (_, InfixOp::Mod, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Eq, Value::Int(m)) => Ok(Value::Bool(n == m)),
                    (Value::Bool(n), InfixOp::Eq, Value::Bool(m)) => Ok(Value::Bool(n == m)),
                    (_, InfixOp::Eq, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::NEq, Value::Int(m)) => Ok(Value::Bool(n != m)),
                    (Value::Bool(n), InfixOp::NEq, Value::Bool(m)) => Ok(Value::Bool(n != m)),
                    (_, InfixOp::NEq, _) => Err(Error::InvalidType),
                    (Value::Int(n), InfixOp::Exp, Value::Int(m)) => Ok(Value::Int(n.pow(m as u32))),
                    (_, InfixOp::Exp, _) => Err(Error::InvalidType),
                    (_, InfixOp::Arrow, _) => Err(Error::InvalidType),
                }
            }
            Prefix(op, arg) => {
                let arg = Self::eval_expr(arg, env)?;
                use PrefixOp::*;
                match (op.as_ref(), arg) {
                    (Neg, Value::Int(n)) => Ok(Value::Int(-n)),
                    (Neg, _) => Err(Error::InvalidType),
                    (Not, Value::Bool(n)) => Ok(Value::Bool(!n)),
                    (Not, _) => Err(Error::InvalidType),
                    (Call(fun), call_arg) => {
                        match Self::eval_expr(fun, env)? {
                            Value::Fn(fn_arg_name, fn_body, mut fn_env) => {
                                let mut fn_env = Env::new_inner(fn_env);
                                fn_env.insert(fn_arg_name.to_string(), call_arg);
                                Self::eval_expr(&dbg!(fn_body), &dbg!(fn_env))
                            },
                            _ => Err(Error::InvalidType),
                        }
                    },
                }
            }
            Var(n) => env.get(n).ok_or(Error::NullVar(n.to_string())).cloned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    use std::collections::HashMap;

    #[test]
    fn test_eval_single_expr() {
        let tests = [
            ("", None, ""),
            ("1", Some(Value::Int(1)), ""),
            ("true", Some(Value::Bool(true)), ""),
            ("1 + 1", Some(Value::Int(2)), ""),
            ("1 - 1", Some(Value::Int(0)), ""),
            ("3 * 2", Some(Value::Int(6)), ""),
            ("3 * 2 + 1", Some(Value::Int(7)), ""),
            ("3 * (2 + 1)", Some(Value::Int(9)), ""),
            ("100 / 10", Some(Value::Int(10)), ""),
            ("-1", Some(Value::Int(-1)), ""),
            ("--1", Some(Value::Int(1)), ""),
            ("-(1 + 1)", Some(Value::Int(-2)), ""),
            ("1 == 1", Some(Value::Bool(true)), ""),
            ("1 == 2", Some(Value::Bool(false)), ""),
            ("1 != 1", Some(Value::Bool(false)), ""),
            ("1 != 2", Some(Value::Bool(true)), ""),
            ("true == true", Some(Value::Bool(true)), ""),
            ("true == false", Some(Value::Bool(false)), ""),
            ("true != true", Some(Value::Bool(false)), ""),
            ("true != false", Some(Value::Bool(true)), ""),
            ("!(1 == 1)", Some(Value::Bool(false)), ""),
            ("!(1 != 1)", Some(Value::Bool(true)), ""),
            ("!!(1 != 1)", Some(Value::Bool(false)), ""),
            ("3 % 2", Some(Value::Int(1)), ""),
            ("3 > 2", Some(Value::Bool(true)), ""),
            ("3 > 3", Some(Value::Bool(false)), ""),
            ("3 < 2", Some(Value::Bool(false)), ""),
            ("3 < 4", Some(Value::Bool(true)), ""),
            ("3 >= 3", Some(Value::Bool(true)), ""),
            ("2 >= 3", Some(Value::Bool(false)), ""),
            ("3 <= 3", Some(Value::Bool(true)), ""),
            ("3 <= 2", Some(Value::Bool(false)), ""),
            ("true && true", Some(Value::Bool(true)), ""),
            ("true && false", Some(Value::Bool(false)), ""),
            ("true || true", Some(Value::Bool(true)), ""),
            ("false || false", Some(Value::Bool(false)), ""),
            ("3 ** 2", Some(Value::Int(9)), ""),
            ("if true then 1 + 1 else 2 + 2", Some(Value::Int(2)), ""),
            ("if false then 1 + 1 else 2 + 2", Some(Value::Int(4)), ""),
            ("one = 1; one", Some(Value::Int(1)), ""),
            ("one = 1 + 1; one", Some(Value::Int(2)), ""),
            ("one = 1; two = one + one; two", Some(Value::Int(2)), ""),
            ("x => 1", Some(Value::Fn("x".to_string(), Expr::Int(1), Env::new())), ""),
        ];

        for (input, expected, desc) in tests {
            let mut env = Env::new();
            let tokens = Lexer::new(input.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let program = Parser::new(&tokens).parse_program().unwrap();
            let eval = Eval::new(program).eval(&mut env).unwrap();
            assert_eq!(eval, expected, "{desc}");
        }
    }

    #[test]
    fn test_lambda() {
        let tests = [
            (
                "y = 1; x => 1",
                Some(Value::Fn(
                    "x".to_string(),
                    Expr::Int(1),
                    Env::from([("y".to_string(), Value::Int(1))]),
                )),
                "",
            ),
            (
                "y = x => 1; y",
                Some(Value::Fn(
                    "x".to_string(),
                    Expr::Int(1),
                    Env::new(),
                )),
                "",
            ),
            (
                "y = x => 1; y 0",
                Some(Value::Int(1)),
                "d",
            ),
            (
                "a = 1; y = x => a; y 0",
                Some(Value::Int(1)),
                "d",
            ),
            (
                "y = x => x; y 0",
                Some(Value::Int(0)),
                "c",
            ),
            (
                "(x => x) 0",
                Some(Value::Int(0)),
                "b",
            ),
            (
                "x = 10; y = x => x; y 0",
                Some(Value::Int(0)),
                "a",
            ),
            (
                "add = x => y => x + y; add_two = add 2; add_two 4",
                Some(Value::Int(6)),
                "a",
            ),
            (
                "fact = x => if x == 0 then 1 else x * fact x - 1; fact 3",
                Some(Value::Int(6)),
                "recursion",
            ),
        ];

        for (input, expected, desc) in tests {
            let mut env = Env::new();
            let tokens = Lexer::new(input.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let program = Parser::new(&tokens).parse_program().unwrap();
            let eval = Eval::new(program).eval(&mut env).unwrap();
            assert_eq!(eval, expected, "{desc}");
        }

    }
}
