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

#[derive(Debug)]
pub struct Error(u8);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find anything for token: {}", self.0)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.lookup_token() {
            return Some(Ok(val));
        }

        match self.0.peek()? {
            b' ' | b'\n' => {
                self.0.next();
                self.next()
            }
            b'#' => {
                while &b'\n' != self.0.next()? {}
                self.next()
            }
            c if c.is_ascii_digit() => Some(Ok(self.read_number().map(Token::Int).unwrap())),
            c if c.is_ascii_alphabetic() => Some(Ok(Token::Ident(self.read_ident()))),
            c => Some(Err(Error(*c))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = b"
        # foo
        200-12 * (foo / bar);
        add_two x = x + 2;
        foo = true;
        bar = false; # foo
        # foo
        -1;
        1 == 1;
        1 != 1;
        !true;
        1 % 1;
        1 > 1;
        1 < 1;
        1 <= 1;
        1 >= 1;
        1 || 1;
        1 && 1;
        1 ** 1;
        x => 1;
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
                Token::Int(1),
                Token::Lt,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::Lte,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::Gte,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::Or,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::And,
                Token::Int(1),
                Token::Semicolon,
                Token::Int(1),
                Token::Exp,
                Token::Int(1),
                Token::Semicolon,
                Token::Ident(b"x"),
                Token::Arrow,
                Token::Int(1),
                Token::Semicolon,
            ],
        )
    }
}
