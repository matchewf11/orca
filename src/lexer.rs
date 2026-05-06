use crate::{cursor::Cursor, token::Token};
use std::{fmt, num::ParseIntError};

pub struct Lexer<'a>(Cursor<'a, u8>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Lexer(Cursor::new(input))
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

#[derive(Debug)]
pub enum Error {
    Number(ParseIntError),
    InvalidByte(u8),
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Number(e) => write!(f, "failed to parse number: {e}"),
            InvalidByte(b) => write!(f, "invalid byte: {b}"),
            Empty => write!(f, "nothing"),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.peek() {
            None => None,
            Some(c) => match c {
                b' ' | b'\n' => {
                    self.0.next();
                    self.next()
                }
                c if c.is_ascii_digit() => {
                    Some(self.read_number().map(Token::Int).map_err(Error::Number))
                }
                c if c.is_ascii_alphabetic() => Some(Ok(Token::lookup_keyword(self.read_ident()))),
                b'!' => {
                    self.0.next();
                    match self.0.peek() {
                        Some(b'=') => {
                            self.0.next();
                            Some(Ok(Token::NEq))
                        },
                        _ => {
                            Some(Ok(Token::Not))
                        }
                    }
                }
                b'=' => {
                    self.0.next();
                    if self.0.peek() == Some(&b'=') {
                        self.0.next();
                        Some(Ok(Token::Eq))
                    } else {
                        Some(Ok(Token::Assign))
                    }
                }
                &c => {
                    self.0.next();
                    c.try_into().ok().map(Ok)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = b"
        200-12 * (foo / bar);
        add_two x = x + 2;
        foo = true;
        bar = false;
        -1;
        1 == 1;
        1 != 1;
        !true;
        1 % 1;
        1 > 1;
        ";

        assert_eq!(
            &Lexer::new(input).collect::<Result<Vec<_>, _>>().unwrap(),
            &[
                Token::Int(200),
                Token::Minus,
                Token::Int(12),
                Token::Mult,
                Token::LParen,
                Token::Ident(b"foo"),
                Token::Div,
                Token::Ident(b"bar"),
                Token::RParen,
                Token::Semicolon,
                Token::Ident(b"add_two"),
                Token::Ident(b"x"),
                Token::Assign,
                Token::Ident(b"x"),
                Token::Plus,
                Token::Int(2),
                Token::Semicolon,
                Token::Ident(b"foo"),
                Token::Assign,
                Token::Bool(true),
                Token::Semicolon,
                Token::Ident(b"bar"),
                Token::Assign,
                Token::Bool(false),
                Token::Semicolon,
                Token::Minus,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::Eq,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::NEq,
                Token::Int(1),
                Token::Semicolon,
                Token::Not,
                Token::Bool(true),
                Token::Semicolon,
                Token::Int(1),
                Token::Mod,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::Gt,
                Token::Int(1),
                Token::Semicolon,
            ],
        )
    }
}
