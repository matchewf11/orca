use crate::{
    ast::{Expr, InfixOp, PrefixOp, Program, Stmt},
    cursor::Cursor,
    prec::Prec,
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
    ExprStart,
    If,
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
            ExprStart => "expr start",
            If => "if",
        };
        write!(f, "{s}")
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
        self.0.eat_if_eq(&Token::Semicolon);
        Ok(stmt)
    }

    fn parse_bind(&mut self) -> Result<Option<(String, Expr)>, Error> {
        let name = match self.0.peek() {
            Some(Token::Ident(t)) => t,
            _ => return Ok(None),
        };

        let mut args = Vec::new();

        for i in 1.. {
            match self.0.peek_n(i) {
                Some(Token::Ident(v)) => args.push(v),
                Some(Token::Assign) => break,
                _ => return Ok(None),
            }
        }

        self.0.eat_n(2 + args.len());

        let body = args.into_iter().rfold(
            self.parse_expr(Prec::Lowest)?.ok_or(Error::FnBody)?,
            |acc, arg| Expr::new_infix(Expr::new_var(arg), InfixOp::Arrow, acc),
        );

        Ok(Some((str::from_utf8(name).unwrap().to_string(), body)))
    }

    fn parse_expr(&mut self, prec: Prec) -> Result<Option<Expr>, Error> {
        let mut lhs = match self.parse_prefix() {
            Ok(Some(v)) => v,
            o => return o,
        };

        while let Some(tok) = self.0.peek()
            && if Prec::is_right_assoc(tok) {
                prec <= Prec::token_prec_infix(tok).ok_or(Error::ExprStart)?
            } else {
                prec < Prec::token_prec_infix(tok).ok_or(Error::ExprStart)?
            }
        {
            lhs = self.parse_infix(lhs)?;
        }

        Ok(Some(lhs))
    }

    fn parse_prefix(&mut self) -> Result<Option<Expr>, Error> {
        let val = match self.0.next() {
            None => return Ok(None),
            Some(v) => v,
        };
        Ok(match val {
            Token::Int(i) => Some(Expr::Int(*i)),
            Token::Null => Some(Expr::Null),
            Token::Bool(b) => Some(Expr::Bool(*b)),
            Token::Ident(i) => Some(Expr::Var(str::from_utf8(i).unwrap().to_string())),
            Token::LParen => {
                let expr = self.parse_expr(Prec::Lowest);
                self.0.expect_or(&Token::RParen, Error::ClosingParen)?;
                expr?
            }
            Token::If => {
                let cond = self.parse_expr(Prec::Lowest)?.ok_or(Error::If)?;
                self.0.expect_or(&Token::Then, Error::ClosingParen)?;
                let a = self.parse_expr(Prec::Lowest)?.ok_or(Error::If)?;
                self.0.expect_or(&Token::Else, Error::ClosingParen)?;
                let b = self.parse_expr(Prec::Lowest)?.ok_or(Error::If)?;
                Some(Expr::new_if(cond, a, b))
            }
            t => {
                let prefix_op: PrefixOp =
                    t.try_into().map_err(|_| Error::PrefixFn(t.to_string()))?;
                self.parse_expr(Prec::Prefix)?
                    .map(|expr| Expr::new_prefix(prefix_op, expr))
            }
        })
    }

    fn parse_infix(&mut self, lhs: Expr) -> Result<Expr, Error> {
        use Token::*;
        let t = self.0.peek().ok_or(Error::InfixRhs)?;
        if let Ok(op) = t.try_into()
            && let Some(pr) = Prec::token_prec_infix(t)
        {
            self.0.next();
            let rhs = self.parse_expr(pr)?.ok_or(Error::InfixRhs)?;
            Ok(Expr::new_infix(lhs, op, rhs))
        } else if Prec::token_prec_infix(t) == Some(Prec::Call) {
            Ok(Expr::new_prefix(
                PrefixOp::Call(lhs),
                self.parse_expr(Prec::Call)?.ok_or(Error::FnBody)?,
            ))
        } else {
            Err(Error::InfixFn(t.to_string()))
        }
    }
}

#[cfg(test)]
mod tests;
