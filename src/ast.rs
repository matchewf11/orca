use crate::token::Token;
use std::fmt;

type Args = Vec<String>;
type Name = String;

#[derive(Debug)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Bind(Name, Args, Expr),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Var(Name),
    Infix(Box<Expr>, InfixOp, Box<Expr>),
    Call(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Error {
    InvalidToken(String),
}

impl TryFrom<&Token<'_>> for InfixOp {
    type Error = Error;
    fn try_from(t: &Token<'_>) -> Result<Self, Self::Error> {
        Ok(match t {
            Token::Plus => InfixOp::Add,
            Token::Minus => InfixOp::Sub,
            Token::Mult => InfixOp::Mul,
            Token::Div => InfixOp::Div,
            t => return Err(Error::InvalidToken(format!("{t}"))),
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
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;
        match self {
            Expr(e) => write!(f, "{e};"),
            e => todo!("write out function call: {e}"),
        }
    }
}
