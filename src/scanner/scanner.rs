use crate::token::token::Token;
use crate::token::token_type::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: String::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None::<String>, self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn scan_token(&self) {}
}
