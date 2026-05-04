use crate::{cursor::Cursor, token::Token};

pub struct Lexer<'a>(Cursor<'a>);

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer(Cursor::new(input))
    }

    fn read_number(&mut self) -> i64 {
        // Todo: fix this, to not eat the thing
        self.0
            .by_ref()
            .take_while(|x: &char| x.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap()
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
        match self.0.peek() {
            Some(' ') | Some('\n') => {
                self.0.next();
                self.next()
            }
            Some('(') => {
                self.0.next();
                Some(Token::LParen)
            }
            Some('/') => {
                self.0.next();
                Some(Token::Div)
            }
            Some(')') => {
                self.0.next();
                Some(Token::RParen)
            }
            Some('=') => {
                self.0.next();
                Some(Token::Assign)
            }
            Some('+') => {
                self.0.next();
                Some(Token::Plus)
            }
            Some(';') => {
                self.0.next();
                Some(Token::Semicolon)
            }
            Some('-') => {
                self.0.next();
                Some(Token::Minus)
            }
            Some('*') => {
                self.0.next();
                Some(Token::Mult)
            }
            Some(c) if c.is_numeric() => Some(Token::Int(self.read_number())),
            Some(c) if c.is_alphabetic() => Some(Token::Ident(self.read_ident())),
            Some(c) => todo!("{c}"),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "
        200 - 12* (foo/bar);
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
