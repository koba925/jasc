#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Skip,
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,
    Identifier(String),
    Number(f64),
    Print,
    EOF,
}

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::Skip => panic!("Invalid Token {:?}", TokenValue::Skip),
            TokenValue::LeftParen => write!(f, "("),
            TokenValue::RightParen => write!(f, ")"),
            TokenValue::Plus => write!(f, "+"),
            TokenValue::Minus => write!(f, "-"),
            TokenValue::Star => write!(f, "*"),
            TokenValue::Slash => write!(f, "/"),
            TokenValue::Semicolon => write!(f, ";"),
            TokenValue::Identifier(name) => write!(f, "(id {})", name),
            TokenValue::Number(n) => write!(f, "{}", n),
            TokenValue::Print => write!(f, "print"),
            TokenValue::EOF => write!(f, "end"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub val: TokenValue,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(val: TokenValue, lexeme: String, line: usize) -> Token {
        Token { val, lexeme, line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.val, self.lexeme, self.line)
    }
}
