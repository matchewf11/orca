use crate::{env::Env, evaluate, repl};
use std::{env, fs, io::Error};

pub fn start() -> Result<(), Error> {
    match env::args().nth(1) {
        None => Ok(repl::start()),
        Some(file_name) => {
            let input = fs::read_to_string(file_name)?;
            Ok(evaluate(&input, Env::new_wrapped()))
        }
    }
}
