use super::token_type::TokenType;
use std::any::Any;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Any>,
    line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: impl Any, line: u64) -> Token {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal: Box::new(literal),
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
