use std::{iter::Peekable, str::Chars};

use crate::{
    error::{LoxError, LoxResult},
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    chars_iter: Peekable<Chars<'a>>,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            chars_iter: source.chars().peekable(),
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) {
        self.chars_iter.next();
        self.current += 1;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }

    pub fn scan_tokens(&mut self) -> LoxResult<(Vec<Token>, bool)> {
        let mut had_error: bool = false;

        while let Some(i) = self.chars_iter.next() {
            dbg!(i);
            self.start = self.current;
            self.scan_token(i).unwrap_or_else(|err| {
                eprintln!("{}", err);
                had_error = true
            });
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        Ok((self.tokens.clone(), had_error))
    }

    fn scan_token<'a>(&mut self, c: char) -> LoxResult<()> {
        self.current += 1;
        match c {
            '(' => {
                self.add_token(TokenType::LeftParen);
                self.match_next('=');
            }
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::Bangequal
                } else {
                    TokenType::Bang
                };

                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::Equalequal
                } else {
                    TokenType::Equal
                };

                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::Lessequal
                } else {
                    TokenType::Less
                };

                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::Greaterequal
                } else {
                    TokenType::Greater
                };

                self.add_token(token_type);
            }
            '/' => {
                if self.match_next('/') {
                    while let Some(next) = self.chars_iter.peek() {
                        match next {
                            '\n' => break,
                            _ => self.advance(),
                        };
                    }
                } else {
                    self.add_token(TokenType::Slash);
                };
            }
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
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
                self.advance();
                return true;
            };
        }

        false
    }

    fn string(&mut self) -> LoxResult<()> {
        let mut string = String::new();
        let mut end_string = false;

        while let Some(_) = self.chars_iter.peek() {
            // TODO refactor to use self.advance()
            let c = self.chars_iter.next().unwrap();
            self.current += 1;
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
            let literal = Literal::Str(string);
            self.add_token_with_literal(TokenType::String, Some(literal));
        } else {
            return Err(LoxError::SyntaxError {
                line: self.line,
                message: "Unterminated string.",
            });
        }
        Ok(())
    }

    fn number(&mut self) -> LoxResult<()> {
        while let Some(c) = self.chars_iter.peek() {
            match c {
                '0'..='9' => self.advance(),
                '.' => {
                    // TODO: Refactor
                    if let Some(v) = self.peek_next() {
                        if v.is_ascii_digit() {
                            self.advance();
                            if let Some(v2) = self.chars_iter.peek() {
                                if v2.is_ascii_digit() {
                                    self.advance();
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
                _ => break,
            };
        }
        let f = self.source[self.start..self.current].parse::<f64>();
        match f {
            Ok(num) => self.add_token_with_literal(TokenType::Number, Some(Literal::Num(num))),
            Err(_) => {
                return Err(LoxError::SyntaxError {
                    line: self.line,
                    message: "Not a valid Number",
                });
            }
        };
        Ok(())
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
}
