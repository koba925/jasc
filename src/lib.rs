pub mod error;
mod parser;
mod scanner;
mod token;

use error::Error;
use parser::Parser;
use scanner::Scanner;

pub fn run(src: impl Into<String>) -> Result<f64, Vec<Error>> {
    Scanner::new(src.into())
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
}
