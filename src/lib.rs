pub mod ast;
pub mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

use ast::Value;
use error::Error;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub fn run(src: impl Into<String>) -> Result<Value, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
        .and_then(|expr| Interpreter::new(expr).interpret())
}
