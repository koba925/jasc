use std::io;

mod error;
mod parser;
mod scanner;
mod token;

use error::Error;
use parser::Parser;
use scanner::Scanner;

fn run(src: String) -> Result<f64, Vec<Error>> {
    Scanner::new(src)
        .scan()
        .and_then(|tokens| Parser::new(tokens).parse())
}

fn main() {
    let src = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    match run(src) {
        Ok(value) => println!("{value}"),
        Err(errors) => {
            for e in errors {
                e.report();
            }
        }
    }
}
