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
    String(String),
    Infix(Box<Expr>, InfixOp, Box<Expr>),
    Prefix(Box<PrefixOp>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Null,
}

impl Expr {
    pub fn new_prefix(op: PrefixOp, expr: Expr) -> Self {
        Self::Prefix(Box::new(op), Box::new(expr))
    }
    pub fn new_infix(lhs: Expr, op: InfixOp, rhs: Expr) -> Self {
        Self::Infix(Box::new(lhs), op, Box::new(rhs))
    }
    pub fn new_var(name: &[u8]) -> Self {
        Self::Var(str::from_utf8(name).unwrap().to_string())
    }
    pub fn new_if(cond: Expr, a: Expr, b: Expr) -> Self {
        Self::If(Box::new(cond), Box::new(a), Box::new(b))
    }
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

impl<'a> TryFrom<&'a Token<'a>> for PrefixOp {
    type Error = Error<'a>;

    fn try_from(t: &'a Token) -> Result<Self, Self::Error> {
        Ok(match t {
            Token::Minus => PrefixOp::Neg,
            Token::Not => PrefixOp::Not,

            Token::Plus
            | Token::Mult
            | Token::Div
            | Token::Eq
            | Token::NEq
            | Token::Mod
            | Token::Gt
            | Token::Lt
            | Token::Null
            | Token::Gte
            | Token::Lte
            | Token::And
            | Token::Or
            | Token::Arrow
            | Token::Exp
            | Token::Dollar
            | Token::Pipe
            | Token::Dot
            | Token::Ident(..)
            | Token::String(..)
            | Token::Int(..)
            | Token::Bool(..)
            | Token::Assign
            | Token::Semicolon
            | Token::LParen
            | Token::RParen
            | Token::If
            | Token::Then
            | Token::Else => return Err(Error(t)),
        })
    }
}

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
            Token::Ident(..)
            | Token::Int(..)
            | Token::String(..)
            | Token::Bool(..)
            | Token::Assign
            | Token::Semicolon
            | Token::LParen
            | Token::RParen
            | Token::Not
            | Token::Null
            | Token::If
            | Token::Then
            | Token::Else => return Err(Error(t)),
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
            Null => write!(f, "null"),
            Bool(b) => write!(f, "{b}"),
            String(b) => write!(f, "\"{b}\""),
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
