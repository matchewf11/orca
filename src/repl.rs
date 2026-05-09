use crate::{env::Env, evaluate};
use std::io::{self, Write};

// use this instead
// trait Repl<C> {}

pub fn start() {
    let env = Env::new_wrapped();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let input = {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input
        };
        evaluate(&input, env.clone());
    }
}
