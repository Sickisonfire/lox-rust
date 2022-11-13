use std::{iter::Peekable, str::Chars};

use crate::{
    error::{LoxError, LoxResult},
    token::Token,
    token_type::TokenType,
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    line: usize,
    chars_iter: Peekable<Chars<'a>>,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            line: 1,
            chars_iter: source.chars().peekable(),
        }
    }

    pub fn scan_tokens(&mut self) -> LoxResult<(Vec<Token>, bool)> {
        let mut had_error: bool = false;

        while let Some(i) = self.chars_iter.next() {
            dbg!(i);
            self.scan_token(i).unwrap_or_else(|err| {
                eprintln!("{}", err);
                had_error = true
            });
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", "OBJECT?", 2));
        Ok((self.tokens.clone(), had_error))
    }

    fn add_token(&self, token_type: TokenType, literal: &str) {
        println!("{}", literal);
    }

    fn scan_token<'a>(&mut self, c: char) -> LoxResult<()> {
        match c {
            '(' => {
                self.add_token(TokenType::LeftParen, "test");
                self.match_next('=');
            }
            ')' => self.add_token(TokenType::RightParen, "test"),
            '{' => self.add_token(TokenType::LeftBrace, "test"),
            '}' => self.add_token(TokenType::RightBrace, "test"),
            ',' => self.add_token(TokenType::Comma, "test"),
            '.' => self.add_token(TokenType::Dot, "test"),
            '-' => self.add_token(TokenType::Minus, "test"),
            '+' => self.add_token(TokenType::Plus, "test"),
            ';' => self.add_token(TokenType::Semicolon, "test"),
            '*' => self.add_token(TokenType::Star, "test"),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::Bangequal
                } else {
                    TokenType::Bang
                };

                self.add_token(token_type, "test");
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::Equalequal
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type, "test");
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::Lessequal
                } else {
                    TokenType::Less
                };

                self.add_token(token_type, "test");
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::Greaterequal
                } else {
                    TokenType::Greater
                };

                self.add_token(token_type, "test");
            }
            '/' => {
                if self.match_next('/') {
                    while let Some(next) = self.chars_iter.peek() {
                        match next {
                            '\n' => break,
                            _ => self.chars_iter.next(),
                        };
                    }
                } else {
                    self.add_token(TokenType::Slash, "test");
                };
            }
            '"' => self.string()?,
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => {
                return Err(LoxError::SyntaxError {
                    line: self.line,
                    message: "Unexpected character",
                })
            }
        };
        Ok(())
    }

    fn match_next(&mut self, expected: char) -> bool {
        if let Some(&next) = self.chars_iter.peek() {
            if next == expected {
                self.chars_iter.next();
                return true;
            };
        }

        false
    }

    fn string(&mut self) -> LoxResult<()> {
        let mut string = String::new();
        let mut end_string = false;

        while let Some(_) = self.chars_iter.peek() {
            let c = self.chars_iter.next().unwrap();
            match c {
                '"' => {
                    end_string = true;
                    break;
                }
                '\n' => {
                    string.push(c);
                    self.line += 1;
                }
                _ => string.push(c),
            }
        }
        if end_string {
            self.add_token(TokenType::String, &string);
        } else {
            return Err(LoxError::SyntaxError {
                line: self.line,
                message: "Unterminated string.",
            });
        }
        Ok(())
    }
}
