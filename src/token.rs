use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(value: &str) -> Token {
        Token {
            token_type: TokenType::RightBrace,
            lexeme: "}".to_string(),
            line: 4,
        }
    }
}
