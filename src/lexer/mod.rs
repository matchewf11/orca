use crate::{cursor::Cursor, token::Token};
use std::{fmt, num::ParseIntError};

pub struct Lexer<'a>(Cursor<'a, u8>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Lexer(Cursor::new(input))
    }

    fn lookup_token(&mut self) -> Option<Token<'static>> {
        let matches = [
            ("false", Token::Bool(false)),
            ("true", Token::Bool(true)),
            ("then", Token::Then),
            ("null", Token::Null),
            ("else", Token::Else),
            ("if", Token::If),
            ("||", Token::Or),
            ("&&", Token::And),
            ("<=", Token::Lte),
            (">=", Token::Gte),
            ("!=", Token::NEq),
            ("=>", Token::Arrow),
            ("**", Token::Exp),
            (">", Token::Gt),
            ("<", Token::Lt),
            ("+", Token::Plus),
            ("-", Token::Minus),
            ("=", Token::Assign),
            (";", Token::Semicolon),
            ("*", Token::Mult),
            ("(", Token::LParen),
            (")", Token::RParen),
            ("/", Token::Div),
            ("!", Token::Not),
            ("%", Token::Mod),
            ("==", Token::Eq),
            (".", Token::Dot),
            ("$", Token::Dollar),
            ("|>", Token::Pipe),
        ]
        .into_iter()
        .filter(|(ident, _)| self.0.is_prefix(ident.as_bytes()));

        matches.max_by_key(|(i, _)| i.len()).map(|(i, v)| {
            self.0.eat_n(i.len());
            v
        })
    }

    fn read_number(&mut self) -> Result<i64, ParseIntError> {
        str::from_utf8(self.0.eat_while(|x| x.is_ascii_digit()))
            .unwrap()
            .parse()
    }

    fn read_ident(&mut self) -> &'a [u8] {
        self.0.eat_while(|b| b.is_ascii_alphabetic() || *b == b'_')
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.lookup_token() {
            return Some(Ok(val));
        }

        Some(match self.0.peek()? {
            b' ' | b'\n' => {
                self.0.next();
                self.next()?
            }
            b'#' => {
                while &b'\n' != self.0.next()? {}
                self.next()?
            }
            c if c.is_ascii_digit() => self
                .read_number()
                .map(Token::Int)
                .map_err(Error::InvalidInt),
            c if c.is_ascii_alphabetic() => Ok(Token::Ident(self.read_ident())),
            c => Err(Error::NotFound(*c)),
        })
    }
}

#[derive(Debug)]
pub enum Error {
    NotFound(u8),
    InvalidInt(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            NotFound(b) => write!(f, "Could not find anything for token: {b}"),
            InvalidInt(e) => write!(f, "Could not lex int: {e}"),
        }
    }
}

#[cfg(test)]
mod tests;
