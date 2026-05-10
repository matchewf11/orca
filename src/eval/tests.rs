use super::*;
use crate::{lexer::Lexer, parser::Parser};
use std::{cell::RefCell, rc::Rc};

#[test]
fn test_eval_single_expr() {
    let tests = [
        ("", Some(Value::Null), ""),
        ("1", Some(Value::Int(1)), ""),
        ("true", Some(Value::Bool(true)), ""),
        ("null", Some(Value::Null), ""),
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
