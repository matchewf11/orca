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
    Arrow,
    // .
    // $
    // |>
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
            Arrow => "=>",
        };
        write!(f, "{s}")
    }
}
