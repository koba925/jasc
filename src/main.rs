use std::io;

mod token {
    #[derive(Debug, PartialEq)]
    pub enum TokenType {
        Semicolon,
        Number(f64),
        EOF,
    }

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub tt: TokenType,
        lexeme: String,
    }

    impl Token {
        pub fn new(tt: TokenType, lexeme: String) -> Token {
            Token { tt, lexeme }
        }
    }
}

mod scanner {
    use crate::token::{Token, TokenType};

    #[derive(Debug)]
    pub struct Scanner {
        src: Vec<char>,
        start: usize,
        current: usize,
        pub tokens: Vec<Token>,
        pub has_error: bool,
    }

    impl Scanner {
        pub fn new(src: String) -> Scanner {
            Scanner {
                src: src.chars().collect(),
                start: 0,
                current: 0,
                tokens: Vec::new(),
                has_error: false,
            }
        }

        pub fn scan(&mut self) {
            self.skip_whitespaces();
            while !self.at_end() {
                if self.current_char() == ';' {
                    self.advance();
                    self.add_token(TokenType::Semicolon);
                    self.advance();
                } else if self.current_char().is_ascii_digit() {
                    self.number();
                } else {
                    eprintln!("Error: Unexpected char");
                    self.has_error = true;
                    return;
                }
                self.skip_whitespaces();
            }
            self.add_token(TokenType::EOF);
        }

        fn skip_whitespaces(&mut self) {
            while !self.at_end() && self.current_char().is_whitespace() {
                self.advance()
            }
        }

        fn number(&mut self) {
            self.advance();
            while !self.at_end() && self.current_char().is_ascii_digit() {
                self.advance();
            }
            self.add_token(TokenType::Number(self.lexeme().parse().unwrap()));
        }

        fn add_token(&mut self, tt: TokenType) {
            self.tokens.push(Token::new(tt, self.lexeme()));
            self.start = self.current
        }

        // self.currentの手前の文字までを切り出すことに注意
        fn lexeme(&self) -> String {
            self.src[self.start..self.current].iter().collect()
        }

        fn advance(&mut self) {
            self.current += 1;
        }

        fn at_end(&self) -> bool {
            self.current >= self.src.len()
        }

        fn current_char(&self) -> char {
            self.src[self.current]
        }
    }
}

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
