use crate::token::Token;
use std::fmt;

pub type Program = Vec<Stmt>;
type Name = String;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Bind(Name, Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Var(Name),
    Infix(Box<Expr>, InfixOp, Box<Expr>),
    Prefix(Box<PrefixOp>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOp {
    Call(Expr),
    Neg,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NEq,
    Mod,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    Exp,
    Arrow,
    Dot,
    Dollar,
    Pipe,
}

#[derive(Debug)]
pub struct Error<'a>(&'a Token<'a>);

impl<'a> TryFrom<&'a Token<'a>> for InfixOp {
    type Error = Error<'a>;

    fn try_from(t: &'a Token) -> Result<Self, Self::Error> {
        Ok(match t {
            Token::Plus => InfixOp::Add,
            Token::Minus => InfixOp::Sub,
            Token::Mult => InfixOp::Mul,
            Token::Div => InfixOp::Div,
            Token::Eq => InfixOp::Eq,
            Token::NEq => InfixOp::NEq,
            Token::Mod => InfixOp::Mod,
            Token::Gt => InfixOp::Gt,
            Token::Lt => InfixOp::Lt,
            Token::Gte => InfixOp::Gte,
            Token::Lte => InfixOp::Lte,
            Token::And => InfixOp::And,
            Token::Or => InfixOp::Or,
            Token::Arrow => InfixOp::Arrow,
            Token::Exp => InfixOp::Exp,
            Token::Dollar => InfixOp::Dollar,
            Token::Pipe => InfixOp::Pipe,
            Token::Dot => InfixOp::Dot,
            t => return Err(Error(t)),
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
            Prefix(o, a) => write!(f, "({o} {a})"),
            If(c, a, b) => write!(f, "if {c} then {a} else {b}"),
        }
    }
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrefixOp::*;
        match self {
            Call(e) => write!(f, "{e}"),
            Neg => write!(f, "-"),
            Not => write!(f, "!"),
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
            Eq => "==",
            NEq => "!=",
            Mod => "%",
            Gt => ">",
            Lt => "<",
            Gte => ">=",
            Lte => "<=",
            And => "&&",
            Or => "||",
            Exp => "**",
            Arrow => "=>",
            Dollar => "$",
            Pipe => "|>",
            Dot => ".",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;
        match self {
            Expr(e) => write!(f, "{e};"),
            Bind(n, e) => write!(f, "{n} = {e};"),
        }
    }
}
