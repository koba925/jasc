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
