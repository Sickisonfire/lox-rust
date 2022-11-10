use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: &str, line: usize) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: "TODO".to_string(),
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        // type + ' ' + lexeme + ' ' + literal
        format!("{:?} {} ", self.token_type, self.lexeme)
    }
}
