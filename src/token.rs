use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Ident(&'a [u8]),
    Int(i64),
    Bool(bool),
    Assign,
    Eq,
    NEq,
    Plus,
    Minus,
    Mult,
    Div,
    Semicolon,
    LParen,
    RParen,
    Not,
    Mod,
    Gt,
    Lt,
    Lte,
    Gte,
    And,
    Or,
    Exp,
    If,
    Then,
    Else,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        let s = match self {
            Ident(i) => return write!(f, "{}", str::from_utf8(i).unwrap()),
            Int(i) => return write!(f, "{i}"),
            Bool(b) => return write!(f, "{b}"),
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Mult => "*",
            Div => "/",
            Semicolon => ";",
            LParen => "(",
            RParen => ")",
            Eq => "==",
            NEq => "!=",
            Not => "!",
            Mod => "%",
            Gt => ">",
            Lt => ">",
            Gte => ">=",
            Lte => "<=",
            And => "&&",
            Or => "||",
            Exp => "**",
            If => "if",
            Then => "then",
            Else => "else",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub struct UnknownSymbolError;

impl TryFrom<u8> for Token<'static> {
    type Error = UnknownSymbolError;

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
            b'!' => Token::Not,
            b'%' => Token::Mod,
            b'>' => Token::Gt,
            b'<' => Token::Lt,
            _ => return Err(UnknownSymbolError),
        })
    }
}

impl<'a> Token<'a> {
    pub fn lookup_keyword(bytes: &'a [u8]) -> Self {
        match bytes {
            b"true" => Token::Bool(true),
            b"false" => Token::Bool(false),
            b"if" => Token::If,
            b"then" => Token::Then,
            b"else" => Token::Else,
            _ => Token::Ident(bytes),
        }
    }
}
