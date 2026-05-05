use crate::{
    ast::{Expr, Program, Stmt},
    cursor::Cursor,
    token::Token,
};
use std::fmt;

pub struct Parser<'a>(Cursor<'a, Token<'a>>);

#[derive(Debug)]
pub enum Error {
    FnBody,
    PrefixFn,
    InfixFn,
    ClosingParen,
    InfixRhs,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let s = match self {
            FnBody => "function has no body",
            PrefixFn => "no prefix function found",
            InfixFn => "no infix function found",
            ClosingParen => "parenthesis are not closed",
            InfixRhs => "no rhs to the infix",
        };
        write!(f, "{s}")
    }
}

#[derive(PartialOrd, PartialEq)]
enum Prec {
    Lowest,
    Sum,
    Product,
    Call,
}

impl Prec {
    fn token_prec(token: &Token) -> Self {
        use Prec::*;
        use Token::*;
        match token {
            Semicolon | RParen | Assign => Lowest,
            Plus | Minus => Sum,
            Mult | Div => Product,
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
        let stmt = if let Some((n, a, e)) = self.parse_bind()? {
            Some(Stmt::Bind(n, a, e))
        } else {
            self.parse_expr(Prec::Lowest)?.map(Into::into)
        };

        // consume ';' if there
        if self.0.peek() == Some(&Token::Semicolon) {
            self.0.next();
        }

        Ok(stmt)
    }

    fn parse_bind(&mut self) -> Result<Option<(String, Vec<String>, Expr)>, Error> {
        let idents = self.0.peek_while_map(|i| match i {
            Token::Ident(i) => Some(str::from_utf8(i).unwrap().to_string()),
            _ => None,
        });

        if idents.is_empty() || self.0.peek_n(idents.len()) != Some(&Token::Assign) {
            return Ok(None);
        }

        // plus 1 for the assign token
        self.0.eat_n(idents.len() + 1);

        let expr = self.parse_expr(Prec::Lowest)?.ok_or(Error::FnBody)?;
        Ok(Some((idents[0].clone(), idents[1..].to_vec(), expr)))
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
            Some(Token::LParen) => {
                let expr = self.parse_expr(Prec::Lowest);
                if self.0.next() != Some(&Token::RParen) {
                    return Err(Error::ClosingParen);
                }
                expr
            }
            Some(Token::Bool(b)) => Ok(Some(Expr::Bool(*b))),
            Some(Token::Assign)
            | Some(Token::Plus)
            | Some(Token::Minus)
            | Some(Token::Mult)
            | Some(Token::Semicolon)
            | Some(Token::RParen)
            | Some(Token::Div) => Err(Error::PrefixFn),
        }
    }

    fn parse_infix(&mut self, lhs: Expr) -> Result<Expr, Error> {
        match self.0.peek().unwrap() {
            Token::Plus | Token::Minus | Token::Div | Token::Mult => {
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
            Token::Ident(..) | Token::Int(..) | Token::LParen | Token::Bool(..) => Ok(Expr::Call(
                Box::new(lhs),
                Box::new(self.parse_expr(Prec::Call)?.ok_or(Error::FnBody)?),
            )),
            Token::Assign | Token::Semicolon | Token::RParen => Err(Error::InfixFn),
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
    fn test_parse() {
        let input = "x + y ; add x y = x + y;";
        let lexer = Lexer::new(input.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(
            Parser::new(&lexer).parse_program().unwrap(),
            vec![
                Stmt::Expr(Expr::Infix(
                    Box::new(Expr::Var("x".to_string())),
                    InfixOp::Add,
                    Box::new(Expr::Var("y".to_string())),
                )),
                Stmt::Bind(
                    "add".to_string(),
                    vec!["x".to_string(), "y".to_string()],
                    Expr::Infix(
                        Box::new(Expr::Var("x".to_string())),
                        InfixOp::Add,
                        Box::new(Expr::Var("y".to_string())),
                    )
                ),
            ],
        );
    }
}
