use crate::{cursor::Cursor, token::Token};

pub struct Lexer<'a>(Cursor<'a>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer(Cursor::new(input))
    }

    fn read_number(&mut self) -> i64 {
        self.0.take_while_slice(|x| x.is_numeric()).parse().unwrap()
    }

    fn read_ident(&mut self) -> &'a str {
        self.0.take_while_slice(Self::is_ident_char)
    }

    fn is_ident_char(c: &char) -> bool {
        c.is_alphabetic() || *c == '_'
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.peek().and_then(|c: char| match c {
            ' ' | '\n' => {
                self.0.next();
                self.next()
            }
            c if TryInto::<Token>::try_into(c).is_ok() => {
                self.0.next();
                c.try_into().ok()
            }
            c if c.is_numeric() => Some(Token::Int(self.read_number())),
            c if c.is_alphabetic() => Some(Token::Ident(self.read_ident())),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "
        200-12 * (foo / bar);
        add_two x = x + 2;
        ";

        assert_eq!(
            &Lexer::new(input).collect::<Vec<_>>(),
            &[
                Token::Int(200),
                Token::Minus,
                Token::Int(12),
                Token::Mult,
                Token::LParen,
                Token::Ident("foo"),
                Token::Div,
                Token::Ident("bar"),
                Token::RParen,
                Token::Semicolon,
                Token::Ident("add_two"),
                Token::Ident("x"),
                Token::Assign,
                Token::Ident("x"),
                Token::Plus,
                Token::Int(2),
                Token::Semicolon,
            ],
        )
    }
}
