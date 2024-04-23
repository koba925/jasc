pub mod ast;
mod env;
pub mod error;
mod interpreter;
mod parser;
mod scanner;
pub mod token;

use ast::{Stmt, Value};
use error::Error;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use token::Token;

pub fn run(src: impl Into<String>) -> Result<Value, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(&tokens).parse())
        .and_then(|stmts| Interpreter::new().interpret(stmts))
}

pub fn scan(src: impl Into<String>) -> Result<Vec<Token>, Vec<Error>> {
    Scanner::new(src.into()).scan()
}

pub fn parse(src: impl Into<String>) -> Result<Vec<Stmt>, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(&tokens).parse())
}
