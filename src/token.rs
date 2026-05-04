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
