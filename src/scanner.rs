use crate::{token::Token, token_type::TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    done: bool,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            done: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // split source string into tokens

        let c = self.source.split("");
        self.scan_token(c);

        self.start = self.current;

        self.tokens
            .push(Token::new(TokenType::Eof, "", "OBJECT?", 2));
        vec![]
    }

    fn add_token(&self, token_type: TokenType, literal: String) {}

    fn scan_token<'a, I>(&self, mut source_iter: I)
    where
        I: Iterator<Item = &'a str>,
    {
        match source_iter.next() {
            Some(c) => match c {
                "(" => self.add_token(TokenType::LeftParen, "test".to_string()),
                _ => self.add_token(TokenType::LeftParen, "test".to_string()),
            },
            None => (),
        }
    }
}
