use std::io;

mod error;
mod scanner;
mod token;

use error::Error;
use scanner::Scanner;
use token::TokenType;

fn run(src: String) -> Result<f64, Vec<Error>> {
    let mut scanner = Scanner::new(src);
    let tokens = scanner.scan()?;

    let value = match tokens[0].tt {
        TokenType::Number(value) => value,
        _ => {
            return Err(vec![Error::GenericError {
                msg: "Error: number expected.".to_string(),
            }])
        }
    };

    match tokens[1].tt {
        TokenType::Semicolon => (),
        _ => {
            return Err(vec![Error::GenericError {
                msg: "Error: semicolon expected.".to_string(),
            }])
        }
    }

    Ok(value)
}

fn main() {
    let src = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    match run(src) {
        Ok(value) => println!("{value}"),
        Err(errors) => {
            for e in errors {
                eprintln!("{}", e)
            }
        }
    }
}
