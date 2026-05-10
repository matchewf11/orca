use crate::token::Token::{self, *};

impl Prec {
    pub fn token_prec_infix(token: &Token) -> Option<Self> {
        use Prec::*;
        Some(match token {
            Token::And => Prec::And,
            Token::Or => Prec::Or,
            Token::Pipe => Prec::Pipe,
            Token::String(_) | Null | Ident(_) | Int(_) | LParen | Bool(_) => Call,
            Then | Else | Semicolon | RParen => Lowest,
            Dollar => Apply,
            Arrow => Lambda,
            Exp => Exponent,
            Lte | Gte | Lt | Gt => Relational,
            Not => Prefix,
            Eq | NEq => Equality,
            Mod | Mult | Div => Product,
            Dot => Compose,
            Plus | Minus => Sum,
            Token::Assign | Token::If => return None,
        })
    }

    pub fn is_right_assoc(tok: &Token) -> bool {
        matches!(tok, Token::Dot | Token::Dollar | Token::Exp | Token::Arrow)
    }
}

#[derive(PartialOrd, PartialEq)]
pub enum Prec {
    Lowest,

    // a $ (b |> c)
    Apply,

    // ((a => a) |> f)
    Pipe,

    // a => (b || c)
    Lambda,

    // ((a && b) || a)
    Or,

    // (true && (false == true))
    And,

    // ((1 > 1) == true)
    Equality,

    // ((1 + 1) > 1)
    Relational,

    // (1 + (1 * 2))
    Sum,

    // ((-1) * 1)
    Product,

    // -(2 ** 2)
    Prefix,

    // ((f g) ** 2)
    Exponent,

    // ((f g) . x)
    Compose,

    // HIGHEST
    Call,
}
