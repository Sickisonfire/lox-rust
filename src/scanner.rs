use crate::{error::LoxError, token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    line: usize,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // split source string into tokens

        let chars_iter = self.source.chars();
        for i in chars_iter {
            self.scan_token(i);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", "OBJECT?", 2));
        self.tokens.clone()
    }

    fn add_token(&self, token_type: TokenType, literal: &str) {}

    fn scan_token<'a>(&self, c: char) {
        match c {
            '(' => self.add_token(TokenType::LeftParen, "test"),
            ')' => self.add_token(TokenType::RightParen, "test"),
            '{' => self.add_token(TokenType::LeftBrace, "test"),
            '}' => self.add_token(TokenType::RightBrace, "test"),
            ',' => self.add_token(TokenType::Comma, "test"),
            '.' => self.add_token(TokenType::Dot, "test"),
            '-' => self.add_token(TokenType::Minus, "test"),
            '+' => self.add_token(TokenType::Plus, "test"),
            ';' => self.add_token(TokenType::Semicolon, "test"),
            '*' => self.add_token(TokenType::Star, "test"),
            _ => eprintln!(
                "{}",
                LoxError::SyntaxError {
                    line: self.line,
                    message: "Unexpected character",
                }
            ),
        };
    }
}
