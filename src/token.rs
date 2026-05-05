use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Ident(&'a [u8]),
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

        let s = match self {
            Ident(i) => return write!(f, "{}", str::from_utf8(i).unwrap()),
            Int(i) => return write!(f, "{i}"),
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Mult => "*",
            Div => "/",
            Semicolon => ";",
            LParen => "(",
            RParen => ")",
        };

        write!(f, "{s}")
    }
}

pub enum Error {
    UnknownSymbol,
}

impl TryFrom<u8> for Token<'static> {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            b'(' => Token::LParen,
            b'/' => Token::Div,
            b')' => Token::RParen,
            b'=' => Token::Assign,
            b'+' => Token::Plus,
            b';' => Token::Semicolon,
            b'-' => Token::Minus,
            b'*' => Token::Mult,
            b => return Err(Error::UnknownSymbol),
        })
    }
}
