use crate::{ast::Program, value::Value};
use std::fmt;

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub struct Eval(Program);

impl Eval {
    pub fn new(prog: Program) -> Self {
        Self(prog)
    }

    pub fn eval(&self) -> Result<Option<Value>, Error> {
        Ok(None)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error")
    }
}
