use crate::lexer::Lexer;
use std::io;

pub fn start() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", Lexer::new(&input).collect::<Vec<_>>());
    }
}
