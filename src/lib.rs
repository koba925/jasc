pub mod ast;
pub mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

use ast::{Expr, Value};
use error::Error;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub fn run(src: impl Into<String>) -> Result<Value, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
        .and_then(|expr| Interpreter::new().interpret(Box::new(expr)))
}

pub fn parse(src: impl Into<String>) -> Result<Expr, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
}
