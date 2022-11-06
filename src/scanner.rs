use crate::token::Token;

pub struct Scanner {
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let mut v = Vec::new();
        for item in source.lines() {
            v.push(Token::new(item))
        }
        Scanner { tokens: v }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        vec![Token::new("TOKEN HERE")]
    }
}
