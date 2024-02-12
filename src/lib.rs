pub mod ast;
pub mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

use ast::{Stmt, Value};
use error::Error;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub fn run(src: impl Into<String>) -> Result<Value, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
        .and_then(|stmts| Interpreter::new().interpret(stmts))
}

pub fn parse(src: impl Into<String>) -> Result<Vec<Stmt>, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
}
