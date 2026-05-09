use crate::token::Token::{self, *};

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

impl Prec {
    pub fn token_prec(token: &Token) -> Self {
        use Prec::*;
        match token {
            Arrow => Lambda,
            Exp => Exponent,
            Token::And => Prec::And,
            Token::Or => Prec::Or,
            Token::Pipe => Prec::Pipe,
            Lte | Gte | Lt | Gt => Relational,
            If | Then | Else | Semicolon | RParen | Assign => Lowest,
            Not => Prefix,
            Eq | NEq => Equality,
            Plus | Minus => Sum,
            Mod | Mult | Div => Product,
            Ident(_) | Int(_) | LParen | Bool(_) => Call,
            Dot => Compose,
            Dollar => Apply,
        }
    }
}

