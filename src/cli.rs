use crate::{env::Env, evaluate, repl};
use std::{env, fs, io::Error};

pub fn start() -> Result<(), Error> {
    match env::args().nth(1) {
        None => {
            repl::start();
            Ok(())
        }
        Some(file_name) => {
            let input = fs::read_to_string(file_name)?;
            evaluate(&input, Env::new_wrapped());
            Ok(())
        }
    }
}
