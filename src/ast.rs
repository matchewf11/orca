use crate::token::Token;
use std::fmt;

pub type Program = Vec<Stmt>;
type Args = Vec<String>;
type Name = String;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Bind(Name, Args, Expr),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Var(Name),
    Infix(Box<Expr>, InfixOp, Box<Expr>),
    Call(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Error {
    InvalidToken,
}

impl TryFrom<&Token<'_>> for InfixOp {
    type Error = Error;
    fn try_from(t: &Token<'_>) -> Result<Self, Self::Error> {
        Ok(match t {
            Token::Plus => InfixOp::Add,
            Token::Minus => InfixOp::Sub,
            Token::Mult => InfixOp::Mul,
            Token::Div => InfixOp::Div,
            _ => return Err(Error::InvalidToken),
        })
    }
}

impl From<Expr> for Stmt {
    fn from(e: Expr) -> Self {
        Stmt::Expr(e)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;
        match self {
            Int(i) => write!(f, "{i}"),
            Bool(b) => write!(f, "{b}"),
            Var(s) => write!(f, "{s}"),
            Infix(l, o, r) => write!(f, "({l} {o} {r})"),
            Call(fun, a) => write!(f, "({fun} {a})"),
        }
    }
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InfixOp::*;
        let s = match self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;
        match self {
            Expr(e) => write!(f, "{e};"),
            Bind(n, a, b) => write!(f, "{n} [{}] = {b}", a.join(",")),
        }
    }
}
