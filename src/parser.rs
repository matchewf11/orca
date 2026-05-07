use crate::{
    ast::{Expr, PrefixOp, Program, Stmt},
    cursor::Cursor,
    token::Token,
};
use std::fmt;

pub struct Parser<'a>(Cursor<'a, Token<'a>>);

#[derive(Debug)]
pub enum Error {
    PrefixFn(String),
    InfixFn(String),
    FnBody,
    ClosingParen,
    InfixRhs,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let s = match self {
            PrefixFn(s) => return write!(f, "no prefix fn found: {s}"),
            InfixFn(s) => return write!(f, "no infix fn found: {s}"),
            FnBody => "function has no body",
            ClosingParen => "parenthesis are not closed",
            InfixRhs => "no rhs to the infix",
        };
        write!(f, "{s}")
    }
}

#[derive(PartialOrd, PartialEq)]
enum Prec {
    Lowest,
    Or,
    And,
    Equality,
    Relational,
    Sum,
    Product,
    Exponent,
    Prefix,
    Call,
}

impl Prec {
    fn token_prec(token: &Token) -> Self {
        use Prec::*;
        use Token::*;
        match token {
            Exp => Exponent,
            Token::And => Prec::And,
            Token::Or => Prec::Or,
            Lte | Gte | Lt | Gt => Relational,
            If | Then | Else | Semicolon | RParen | Assign => Lowest,
            Not => Prefix,
            Eq | NEq => Equality,
            Plus | Minus => Sum,
            Mod | Mult | Div => Product,
            Ident(_) | Int(_) | LParen | Bool(_) => Call,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token<'a>]) -> Self {
        Parser(Cursor::new(input))
    }

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut res = Vec::new();
        while let Some(v) = self.parse_stmt()? {
            res.push(v);
        }
        Ok(res)
    }

    fn parse_stmt(&mut self) -> Result<Option<Stmt>, Error> {
        let stmt = if let Some((n, e)) = self.parse_bind()? {
            Some(Stmt::Bind(n, e))
        } else {
            self.parse_expr(Prec::Lowest)?.map(Into::into)
        };

        // consume ';' if there
        if self.0.peek() == Some(&Token::Semicolon) {
            self.0.next();
        }

        Ok(stmt)
    }

    fn parse_bind(&mut self) -> Result<Option<(String, Expr)>, Error> {
        let name = match self.0.peek() {
            Some(Token::Ident(t)) => t,
            _ => return Ok(None),
        };
        if self.0.peek_n(1) != Some(&Token::Assign) {
            return Ok(None);
        }
        self.0.eat_n(2);
        let expr = self.parse_expr(Prec::Lowest)?.ok_or(Error::FnBody)?;
        Ok(Some((str::from_utf8(name).unwrap().to_string(), expr)))
    }

    fn parse_expr(&mut self, prec: Prec) -> Result<Option<Expr>, Error> {
        let mut lhs = match self.parse_prefix() {
            Ok(Some(v)) => v,
            o => return o,
        };

        while let Some(tok) = self.0.peek()
            && prec < Prec::token_prec(tok)
        {
            lhs = self.parse_infix(lhs)?;
        }

        Ok(Some(lhs))
    }

    fn parse_prefix(&mut self) -> Result<Option<Expr>, Error> {
        match self.0.next() {
            None => Ok(None),
            Some(Token::Int(i)) => Ok(Some(Expr::Int(*i))),
            Some(Token::Ident(i)) => Ok(Some(Expr::Var(str::from_utf8(i).unwrap().to_string()))),
            Some(Token::Bool(b)) => Ok(Some(Expr::Bool(*b))),
            Some(Token::LParen) => {
                let expr = self.parse_expr(Prec::Lowest);
                if self.0.next() != Some(&Token::RParen) {
                    return Err(Error::ClosingParen);
                }
                expr
            }
            Some(Token::If) => {
                let cond = self.parse_expr(Prec::Lowest)?.unwrap();
                if self.0.next() != Some(&Token::Then) {
                    return Err(Error::ClosingParen);
                }
                let a = self.parse_expr(Prec::Lowest)?.unwrap();
                if self.0.next() != Some(&Token::Else) {
                    return Err(Error::ClosingParen);
                }
                let b = self.parse_expr(Prec::Lowest)?.unwrap();
                Ok(Some(Expr::If(Box::new(cond), Box::new(a), Box::new(b))))
            }
            Some(Token::Minus) => Ok(self
                .parse_expr(Prec::Prefix)?
                .map(|expr| Expr::Prefix(Box::new(PrefixOp::Neg), Box::new(expr)))),
            Some(Token::Not) => Ok(self
                .parse_expr(Prec::Prefix)?
                .map(|expr| Expr::Prefix(Box::new(PrefixOp::Not), Box::new(expr)))),
            Some(t) => Err(Error::PrefixFn(t.to_string())),
        }
    }

    fn parse_infix(&mut self, lhs: Expr) -> Result<Expr, Error> {
        match self.0.peek().unwrap() {
            Token::Lte
            | Token::Gte
            | Token::Lt
            | Token::Exp
            | Token::Gt
            | Token::Mod
            | Token::Plus
            | Token::Minus
            | Token::Div
            | Token::And
            | Token::Or
            | Token::Mult
            | Token::Eq
            | Token::NEq => {
                let op = self.0.next().unwrap();
                let rhs = self
                    .parse_expr(Prec::token_prec(op))?
                    .ok_or(Error::InfixRhs)?;
                Ok(Expr::Infix(
                    Box::new(lhs),
                    op.try_into().unwrap(),
                    Box::new(rhs),
                ))
            }
            Token::Ident(..) | Token::Int(..) | Token::LParen | Token::Bool(..) => {
                Ok(Expr::Prefix(
                    Box::new(PrefixOp::Call(lhs)),
                    Box::new(self.parse_expr(Prec::Call)?.ok_or(Error::FnBody)?),
                ))
            }
            t => Err(Error::InfixFn(t.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::InfixOp, lexer::Lexer};

    #[test]
    fn test_parse_expr() {
        let tests = [
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
            ("2 ** -2", "(2 ** (- 2));"),
            ("2 ** 2 * 2", "((2 ** 2) * 2);"),
        ];

        for (input, expected) in tests {
            let lexer = Lexer::new(input.as_bytes())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            let parser = Parser::new(&lexer).parse_program().unwrap();
            assert_eq!(parser.len(), 1);
            assert_eq!(parser[0].to_string(), expected)
        }
    }

    #[test]
    fn test_parse_bind() {
        let input = "x = 1 + 1";
        let lexer = Lexer::new(input.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let parser = Parser::new(&lexer).parse_program().unwrap();
        assert_eq!(parser.len(), 1);
        assert_eq!(
            parser[0],
            Stmt::Bind(
                "x".to_string(),
                Expr::Infix(Box::new(Expr::Int(1)), InfixOp::Add, Box::new(Expr::Int(1)),)
            )
        );
    }
}
