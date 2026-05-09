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
    x |> x $ y . f;
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
            Token::Ident(b"x"),
            Token::Pipe,
            Token::Ident(b"x"),
            Token::Dollar,
            Token::Ident(b"y"),
            Token::Dot,
            Token::Ident(b"f"),
            Token::Semicolon,
        ],
    )
}
