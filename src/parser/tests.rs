use super::*;
use crate::{ast::InfixOp, lexer::Lexer};

fn program(input: &str) -> Program {
    let lexer = Lexer::new(input.as_bytes())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let parser = Parser::new(&lexer).parse_program().unwrap();
    parser
}

#[test]
fn test_parse_expr() {
    let tests = [
        ("null", "null;"),
        ("42", "42;"),
        ("42;", "42;"),
        ("x", "x;"),
        ("1 + 2", "(1 + 2);"),
        ("x + y", "(x + y);"),
        ("3 - 4", "(3 - 4);"),
        ("8 / 2", "(8 / 2);"),
        ("5 * 6", "(5 * 6);"),
        ("1 + 2 * 3", "(1 + (2 * 3));"),
        ("1 * 2 + 3", "((1 * 2) + 3);"),
        ("1 + 2 + 3", "((1 + 2) + 3);"),
        ("8 / 4 / 2", "((8 / 4) / 2);"),
        ("(1 + (2 + 3))", "(1 + (2 + 3));"),
        ("(1 + 2)", "(1 + 2);"),
        ("x * 2 + y", "((x * 2) + y);"),
        ("1 + 2 * 3 + 4", "((1 + (2 * 3)) + 4);"),
        ("f x", "(f x);"),
        ("f 1", "(f 1);"),
        ("f x + 1", "((f x) + 1);"),
        ("f (x + 1)", "(f (x + 1));"),
        ("f x + g y", "((f x) + (g y));"),
        ("f (g x)", "(f (g x));"),
        ("f (x + y) * 2", "((f (x + y)) * 2);"),
        ("f g x", "((f g) x);"),
        ("((f g) x)", "((f g) x);"),
        ("true;", "true;"),
        ("false;", "false;"),
        ("f x true;", "((f x) true);"),
        ("-42", "(- 42);"),
        ("2 * -1", "(2 * (- 1));"),
        ("-2 * 1", "((- 2) * 1);"),
        ("2 + -1", "(2 + (- 1));"),
        ("-2 + 1", "((- 2) + 1);"),
        ("--2", "(- (- 2));"),
        ("-(2)", "(- 2);"),
        ("-f x", "(- (f x));"),
        ("f - 1", "(f - 1);"),
        ("f (-1)", "(f (- 1));"),
        ("-1 * 2 + 3", "(((- 1) * 2) + 3);"),
        ("1 + -2 * 3", "(1 + ((- 2) * 3));"),
        ("-(1 + 2) * 3", "((- (1 + 2)) * 3);"),
        ("f + 1 == x + 1", "((f + 1) == (x + 1));"),
        ("f + 1 != x + 1", "((f + 1) != (x + 1));"),
        ("!!true", "(! (! true));"),
        ("!true == !true", "((! true) == (! true));"),
        ("2 % -1", "(2 % (- 1));"),
        ("2 % 2 * 2", "((2 % 2) * 2);"),
        ("foo == 2 > -1 + 1", "(foo == (2 > ((- 1) + 1)));"),
        ("foo == 2 < -1 + 1", "(foo == (2 < ((- 1) + 1)));"),
        ("2 < x > 2", "((2 < x) > 2);"),
        ("2 > x < 2", "((2 > x) < 2);"),
        ("2 < x >= 2", "((2 < x) >= 2);"),
        ("2 >= x < 2", "((2 >= x) < 2);"),
        ("2 < x <= 2", "((2 < x) <= 2);"),
        ("2 <= x < 2", "((2 <= x) < 2);"),
        ("true && false || true", "((true && false) || true);"),
        ("true && 2 == 1", "(true && (2 == 1));"),
        ("true || 2 == 1", "(true || (2 == 1));"),
        ("2 ** -2", "(2 ** (- 2));"),
        ("-2 ** 2", "(- (2 ** 2));"),
        ("2 ** 2 * 2", "((2 ** 2) * 2);"),
        ("2 ** 2 ** 2", "(2 ** (2 ** 2));"),
        ("x => 1 + 1", "(x => (1 + 1));"),
        ("x => y => x + y", "(x => (y => (x + y)));"),
        ("a . b . c", "(a . (b . c));"),
        ("a $ b $ c", "(a $ (b $ c));"),
        ("a |> b |> c", "((a |> b) |> c);"),
        ("f $ 1 + 1", "(f $ (1 + 1));"),
        ("1 + 1 |> f", "((1 + 1) |> f);"),
        ("f . g + 1", "((f . g) + 1);"),
    ];

    for (input, expected) in tests {
        let parser = program(input);
        assert_eq!(parser.len(), 1);
        assert_eq!(parser[0].to_string(), expected)
    }
}

#[test]
fn test_parse_bind() {
    let parser = program("x = 1 + 1");
    assert_eq!(parser.len(), 1);
    assert_eq!(
        parser[0],
        Stmt::Bind(
            "x".to_string(),
            Expr::Infix(Box::new(Expr::Int(1)), InfixOp::Add, Box::new(Expr::Int(1)),)
        )
    );
}

#[test]
fn test_parse_bind_arg() {
    let parser = program("add x y = x + y");
    assert_eq!(parser.len(), 1);
    assert_eq!(
        parser[0],
        Stmt::Bind(
            "add".to_string(),
            Expr::Infix(
                Box::new(Expr::Var("x".to_string())),
                InfixOp::Arrow,
                Box::new(Expr::Infix(
                    Box::new(Expr::Var("y".to_string())),
                    InfixOp::Arrow,
                    Box::new(Expr::Infix(
                        Box::new(Expr::Var("x".to_string())),
                        InfixOp::Add,
                        Box::new(Expr::Var("y".to_string())),
                    )),
                )),
            )
        )
    );
}
