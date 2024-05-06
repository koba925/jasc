// TODO: コメントの処理がダサいのでなんとかする

use crate::error::Error;
use crate::token::{Token, TokenValue};

#[derive(Debug)]
pub struct Scanner {
    src: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

// TODO: multipeekを使う
impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            src: src.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = vec![];
        let mut errors = vec![];

        self.skip_whitespaces();
        while !self.is_at_end() {
            match self.scan_token() {
                Ok(token) if token.val == TokenValue::Skip => (),
                Ok(token) => tokens.push(token),
                Err(error) => errors.push(error),
            }
            self.skip_whitespaces();
        }

        self.start = self.current;
        tokens.push(self.make_token(TokenValue::EOF));

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn scan_token(&mut self) -> Result<Token, Error> {
        self.start = self.current;

        match self.advance() {
            '(' => Ok(self.make_token(TokenValue::LeftParen)),
            ')' => Ok(self.make_token(TokenValue::RightParen)),
            '{' => Ok(self.make_token(TokenValue::LeftBrace)),
            '}' => Ok(self.make_token(TokenValue::RightBrace)),
            '*' => Ok(self.make_token(TokenValue::Star)),
            '/' => {
                if self.peek() == '/' {
                    self.advance();
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    Ok(self.make_token(TokenValue::Skip))
                } else {
                    Ok(self.make_token(TokenValue::Slash))
                }
            }
            '+' => Ok(self.make_token(TokenValue::Plus)),
            '-' => Ok(self.make_token(TokenValue::Minus)),
            '?' => Ok(self.make_token(TokenValue::Question)),
            ':' => Ok(self.make_token(TokenValue::Colon)),
            ';' => Ok(self.make_token(TokenValue::Semicolon)),
            '=' => Ok(self.make_token(TokenValue::Equal)),
            ',' => Ok(self.make_token(TokenValue::Comma)),
            '&' => self.two_letter_op_or_error('&', TokenValue::And),
            '|' => self.two_letter_op_or_error('|', TokenValue::Or),
            c if c.is_ascii_digit() => Ok(self.number()),
            c if Self::is_identifier_char(c) => Ok(self.identifier()),
            c => Err(self.error(&c.to_string(), "Unexpected character.")),
        }
    }

    fn skip_whitespaces(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
    }

    fn two_letter_op_or_error(&mut self, second: char, op: TokenValue) -> Result<Token, Error> {
        if self.peek() == second {
            self.advance();
            Ok(self.make_token(op))
        } else {
            Err(self.error(
                &self.peek().to_string(),
                &format!("Expected '{}'.", &second),
            ))
        }
    }

    fn number(&mut self) -> Token {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }
        self.make_token(TokenValue::Number(self.lexeme().parse().unwrap()))
    }

    fn identifier(&mut self) -> Token {
        while !self.is_at_end() && Self::is_identifier_char(self.peek()) {
            self.advance();
        }
        let lexeme = self.lexeme();
        match lexeme.as_str() {
            "function" => self.make_token(TokenValue::Function),
            "if" => self.make_token(TokenValue::If),
            "else" => self.make_token(TokenValue::Else),
            "let" => self.make_token(TokenValue::Let),
            "print" => self.make_token(TokenValue::Print),
            "return" => self.make_token(TokenValue::Return),
            "while" => self.make_token(TokenValue::While),
            "true" => self.make_token(TokenValue::True),
            "false" => self.make_token(TokenValue::False),
            "break" => self.make_token(TokenValue::Break),
            _ => self.make_token(TokenValue::Identifier),
        }
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn make_token(&self, val: TokenValue) -> Token {
        Token::new(val, self.lexeme(), self.line)
    }

    // self.currentの手前の文字までを切り出すことに注意
    fn lexeme(&self) -> String {
        self.src[self.start..self.current].iter().collect()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.src[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    // TODO: Optionを使うかmultipeekに置き換える
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.src[self.current]
        }
    }

    fn error(&self, location: &str, msg: &str) -> Error {
        Error::new(self.line, if location == "\0" { "" } else { location }, msg)
    }
}

#[cfg(test)]
mod test {
    use super::Scanner;
    use crate::error::Error;
    use crate::token::{Token, TokenValue};

    #[test]
    fn test_scanner() {
        let src =
            "function (a, b) { let a = (-1 + 2) / 3 * 4;\nif else return print while break a ? true : false && || ; }";
        let result = Scanner::new(src.to_string()).scan();
        let expected = vec![
            Token::new(TokenValue::Function, "function".to_string(), 1),
            Token::new(TokenValue::LeftParen, "(".to_string(), 1),
            Token::new(TokenValue::Identifier, "a".to_string(), 1),
            Token::new(TokenValue::Comma, ",".to_string(), 1),
            Token::new(TokenValue::Identifier, "b".to_string(), 1),
            Token::new(TokenValue::RightParen, ")".to_string(), 1),
            Token::new(TokenValue::LeftBrace, "{".to_string(), 1),
            Token::new(TokenValue::Let, "let".to_string(), 1),
            Token::new(TokenValue::Identifier, "a".to_string(), 1),
            Token::new(TokenValue::Equal, "=".to_string(), 1),
            Token::new(TokenValue::LeftParen, "(".to_string(), 1),
            Token::new(TokenValue::Minus, "-".to_string(), 1),
            Token::new(TokenValue::Number(1.0), "1".to_string(), 1),
            Token::new(TokenValue::Plus, "+".to_string(), 1),
            Token::new(TokenValue::Number(2.0), "2".to_string(), 1),
            Token::new(TokenValue::RightParen, ")".to_string(), 1),
            Token::new(TokenValue::Slash, "/".to_string(), 1),
            Token::new(TokenValue::Number(3.0), "3".to_string(), 1),
            Token::new(TokenValue::Star, "*".to_string(), 1),
            Token::new(TokenValue::Number(4.0), "4".to_string(), 1),
            Token::new(TokenValue::Semicolon, ";".to_string(), 1),
            Token::new(TokenValue::If, "if".to_string(), 2),
            Token::new(TokenValue::Else, "else".to_string(), 2),
            Token::new(TokenValue::Return, "return".to_string(), 2),
            Token::new(TokenValue::Print, "print".to_string(), 2),
            Token::new(TokenValue::While, "while".to_string(), 2),
            Token::new(TokenValue::Break, "break".to_string(), 2),
            Token::new(TokenValue::Identifier, "a".to_string(), 2),
            Token::new(TokenValue::Question, "?".to_string(), 2),
            Token::new(TokenValue::True, "true".to_string(), 2),
            Token::new(TokenValue::Colon, ":".to_string(), 2),
            Token::new(TokenValue::False, "false".to_string(), 2),
            Token::new(TokenValue::And, "&&".to_string(), 2),
            Token::new(TokenValue::Or, "||".to_string(), 2),
            Token::new(TokenValue::Semicolon, ";".to_string(), 2),
            Token::new(TokenValue::RightBrace, "}".to_string(), 2),
        ];
        match result {
            Ok(tokens) => {
                for (token, exp) in tokens.iter().zip(&expected) {
                    assert_eq!(token, exp);
                }
            }
            Err(_) => panic!("Failed - result: {:?}", result),
        }
    }

    #[test]
    fn test_and() {
        assert_eq!(
            Scanner::new("&&".to_string()).scan(),
            Ok(vec![
                Token::new(TokenValue::And, "&&".to_string(), 1),
                Token::new(TokenValue::EOF, "".to_string(), 1),
            ])
        );
        assert_eq!(
            Scanner::new("&a".to_string()).scan(),
            Err(vec![Error::new(1, "a", "Expected '&'.",)])
        );
        assert_eq!(
            Scanner::new("&".to_string()).scan(),
            Err(vec![Error::new(1, "", "Expected '&'.",)])
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            Scanner::new("||".to_string()).scan(),
            Ok(vec![
                Token::new(TokenValue::Or, "||".to_string(), 1),
                Token::new(TokenValue::EOF, "".to_string(), 1),
            ])
        );
        assert_eq!(
            Scanner::new("|a".to_string()).scan(),
            Err(vec![Error::new(1, "a", "Expected '|'.",)])
        );
        assert_eq!(
            Scanner::new("|".to_string()).scan(),
            Err(vec![Error::new(1, "", "Expected '|'.",)])
        );
    }
}
