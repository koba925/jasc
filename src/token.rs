#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Skip,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Question,
    Colon,
    Semicolon,
    Equal,
    Comma,
    Identifier,
    Number(f64),
    Function,
    If,
    Let,
    Print,
    EOF,
}

impl std::fmt::Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::Skip => panic!("Invalid Token {:?}", TokenValue::Skip),
            TokenValue::LeftParen => write!(f, "("),
            TokenValue::RightParen => write!(f, ")"),
            TokenValue::LeftBrace => write!(f, "{{"),
            TokenValue::RightBrace => write!(f, "}}"),
            TokenValue::Plus => write!(f, "+"),
            TokenValue::Minus => write!(f, "-"),
            TokenValue::Star => write!(f, "*"),
            TokenValue::Slash => write!(f, "/"),
            TokenValue::Question => write!(f, "?"),
            TokenValue::Colon => write!(f, ":"),
            TokenValue::Semicolon => write!(f, ";"),
            TokenValue::Equal => write!(f, "="),
            TokenValue::Comma => write!(f, ","),
            TokenValue::Identifier => write!(f, "id"),
            TokenValue::Number(n) => write!(f, "{}", n),
            TokenValue::Function => write!(f, "function"),
            TokenValue::If => write!(f, "if"),
            TokenValue::Let => write!(f, "let"),
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
        write!(f, "({:?} '{}' {:?})", self.val, self.lexeme, self.line)
    }
}

pub fn stringify_tokens(tokens: &Vec<Token>) -> String {
    let mut result = String::new();

    for token in tokens {
        result = format!("{}{}\n", result, token)
    }

    result
}
