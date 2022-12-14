use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        // type + ' ' + lexeme + ' ' + literal
        format!(
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            match &self.literal {
                Some(l) => l.to_string(),
                None => String::new(),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Num(f64),
    Str(String),
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Str(x) => format!("{}  ", x),
            Literal::Num(x) => format!("{}  ", x),
        }
    }
}
