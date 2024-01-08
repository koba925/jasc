use std::io;

mod scanner;
mod token;

use scanner::Scanner;
use token::TokenType;

fn run(src: String) {
    let mut scanner = Scanner::new(src);
    scanner.scan();
    if scanner.has_error {
        return;
    }

    match scanner.tokens[0].tt {
        TokenType::Number(_) => (),
        _ => {
            eprintln!("Error: number expected.");
            return;
        }
    }
    match scanner.tokens[1].tt {
        TokenType::Semicolon => (),
        _ => {
            eprintln!("Error: semicolon expected.");
            return;
        }
    }

    match scanner.tokens[0].tt {
        TokenType::Number(value) => {
            println!("{}", value);
        }
        _ => (),
    }
}

fn main() {
    let src = io::read_to_string(io::stdin()).expect("Error: failed to read the code.");
    run(src);
}
