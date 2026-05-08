use crate::{
    ast::{Expr, InfixOp, PrefixOp, Program, Stmt},
    builtin::find_builtin,
    env::{Env, EnvRef},
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
                // early return on an error (throwing)
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
            Bool(b) => Value::Bool(b),
            Var(n) => env
                .borrow()
                .get(&n)
                .unwrap_or(find_builtin(&n).map(Value::Builtin).unwrap_or(Value::Null)),
            If(cond, a, b) => match Self::eval_expr(*cond, env.clone()) {
                Value::Bool(true) => Self::eval_expr(*a, env),
                Value::Bool(false) => Self::eval_expr(*b, env),
                v => Value::Error(format!("If Statement Condition can't be: {v}")),
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
                    v => Value::Error(format!("Need a function: {v}")),
                },
                (op, val) => Value::Error(format!("Can't use '{op}' for value: {val}")),
            },
            Infix(lhs, op, rhs) => match (*lhs, op, *rhs) {
                (Expr::Var(s), InfixOp::Arrow, rhs) => Value::Fn(s, rhs, Env::branch(env)),
                (lhs, op, rhs) => {
                    let lhs = Self::eval_expr(lhs, env.clone());
                    let rhs = Self::eval_expr(rhs, env);
                    match (lhs, op, rhs) {
                        (Value::Bool(n), InfixOp::And, Value::Bool(m)) => Value::Bool(n && m),
                        (Value::Bool(n), InfixOp::Or, Value::Bool(m)) => Value::Bool(n || m),
                        (Value::Int(n), InfixOp::Lte, Value::Int(m)) => Value::Bool(n <= m),
                        (Value::Int(n), InfixOp::Gte, Value::Int(m)) => Value::Bool(n >= m),
                        (Value::Int(n), InfixOp::Lt, Value::Int(m)) => Value::Bool(n < m),
                        (Value::Int(n), InfixOp::Gt, Value::Int(m)) => Value::Bool(n > m),
                        (Value::Int(n), InfixOp::Add, Value::Int(m)) => Value::Int(n + m),
                        (Value::Int(n), InfixOp::Sub, Value::Int(m)) => Value::Int(n - m),
                        (Value::Int(n), InfixOp::Mul, Value::Int(m)) => Value::Int(n * m),
                        (Value::Int(n), InfixOp::Div, Value::Int(m)) => Value::Int(n / m),
                        (Value::Int(n), InfixOp::Mod, Value::Int(m)) => Value::Int(n % m),
                        (Value::Int(n), InfixOp::Exp, Value::Int(m)) => Value::Int(n.pow(m as u32)),
                        (Value::Int(n), InfixOp::Eq, Value::Int(m)) => Value::Bool(n == m),
                        (Value::Bool(n), InfixOp::Eq, Value::Bool(m)) => Value::Bool(n == m),
                        (Value::Int(n), InfixOp::NEq, Value::Int(m)) => Value::Bool(n != m),
                        (Value::Bool(n), InfixOp::NEq, Value::Bool(m)) => Value::Bool(n != m),
                        (lhs, op, rhs) => {
                            Value::Error(format!("{op} not supported for {lhs} and {rhs}"))
                        }
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_eval_single_expr() {
        let tests = [
            ("", Some(Value::Null), ""),
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
        ];

        for (input, expected, desc) in tests {
            let env = Env::new();
            let tokens = Lexer::new(input.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let program = Parser::new(&tokens).parse_program().unwrap();
            let eval = Eval::new(program).eval(Rc::new(RefCell::new(env)));
            assert_eq!(eval, expected.unwrap(), "{desc}");
        }
    }

    #[test]
    fn test_lambda() {
        let tests = [
            (
                "x => 1",
                Some(Value::Fn(
                    "x".to_string(),
                    Expr::Int(1),
                    Rc::new(RefCell::new(Env::new())),
                )),
                "",
            ),
            (
                "y = 1; x => 1",
                Some(Value::Fn(
                    "x".to_string(),
                    Expr::Int(1),
                    Rc::new(RefCell::new(Env::from([("y".to_string(), Value::Int(1))]))),
                )),
                "",
            ),
            (
                "y = x => 1; y",
                Some(Value::Fn(
                    "x".to_string(),
                    Expr::Int(1),
                    Rc::new(RefCell::new(Env::new())),
                )),
                "",
            ),
            ("y = x => 1; y 0", Some(Value::Int(1)), "d"),
            ("a = 1; y = x => a; y 0", Some(Value::Int(1)), "d"),
            ("y = x => x; y 0", Some(Value::Int(0)), "c"),
            ("(x => x) 0", Some(Value::Int(0)), "b"),
            ("x = 10; y = x => x; y 0", Some(Value::Int(0)), "a"),
            (
                "add = x => y => x + y; add_two = add 2; add_two 4",
                Some(Value::Int(6)),
                "a",
            ),
            (
                "fact = self => n => if n == 0 then 1 else n * self self (n - 1); fact fact 3",
                Some(Value::Int(6)),
                "recursion",
            ),
            (
                "fib = self => x => if x < 2 then x else self self (x - 1) + self self (x - 2); fib fib 10",
                Some(Value::Int(55)),
                "recursion",
            ),
        ];

        for (input, expected, desc) in tests {
            let env = Env::new();
            let tokens = Lexer::new(input.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let program = Parser::new(&tokens).parse_program().unwrap();
            let eval = Eval::new(program).eval(Rc::new(RefCell::new(env)));
            assert_eq!(eval, expected.unwrap(), "{desc}");
        }
    }
}
