use crate::{cursor::Cursor, token::Token};
use std::num::ParseIntError;

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
        self.0.eat_while(|c| c.is_ascii_alphabetic() || *c == b'_')
    }
}

#[derive(Debug)]
pub struct Error;

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.peek().and_then(|c| match c {
            b' ' | b'\n' => {
                self.0.next();
                self.next()
            }
            c if c.is_ascii_digit() => Some(Ok(Token::Int(self.read_number().unwrap()))),
            c if c.is_ascii_alphabetic() => Some(Ok(Token::lookup_keyword(self.read_ident()))),
            &c => {
                self.0.next();
                c.try_into().ok().map(Ok)
            }
        })
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
            ],
        )
    }
}
