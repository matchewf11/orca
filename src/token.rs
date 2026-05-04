use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Ident(&'a str),
    Int(i64),
    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    Semicolon,
    LParen,
    RParen,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Ident(i) => write!(f, "{i}"),
            Int(i) => write!(f, "{i}"),
            Assign => write!(f, "="),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Mult => write!(f, "*"),
            Div => write!(f, "/"),
            Semicolon => write!(f, ";"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
        }
    }
}

pub struct Error;

impl TryFrom<char> for Token<'_> {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Token::LParen),
            '/' => Ok(Token::Div),
            ')' => Ok(Token::RParen),
            '=' => Ok(Token::Assign),
            '+' => Ok(Token::Plus),
            ';' => Ok(Token::Semicolon),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Mult),
            _ => Err(Error),
        }
    }
}
