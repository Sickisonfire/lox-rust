use crate::token_type::TokenType;

#[derive(Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: &str, line: usize) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            literal: "TODO",
            line,
        }
    }
}

impl ToString for Token<'_> {
    fn to_string(&self) -> String {
        // type + ' ' + lexeme + ' ' + literal
        format!("{:?} {} ", self.token_type, self.lexeme)
    }
}
