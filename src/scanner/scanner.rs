use crate::token::token::Token;
use crate::token::token_type::TokenType;
use std::any::Any;
use std::process;

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

    fn scan_token(&mut self) {
        let c: char = if let Some(c) = self.advance() {
            c
        } else {
            println!(
                "Something went wrong, there is no character at {}",
                self.current
            );
            return;
        };
        match c {
            '(' => self.add_token(TokenType::LeftParen),
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
                let token_type = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                };
            }
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            // TODO: Add error handling -- might have to change how Lox is frameworked
            //          - This means that I might need an error reporting crate
            _ => println!("Not a valid token!"),
        }
    }

    fn advance(&self) -> Option<char> {
        self.source.chars().nth(self.current as usize)
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_and_literal(token_type, None::<char>);
    }

    fn add_token_and_literal(&mut self, token_type: TokenType, literal: impl Any) {
        let text = &self.source[self.start as usize..self.current as usize];
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(cur_char) = self.source.chars().nth(self.current as usize) {
            if cur_char != expected {
                return false;
            }
        } else {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let c;
        if let Some(next_char) = self.source.chars().nth(self.current as usize) {
            c = next_char;
        } else {
            //TODO: Have to fix error handling here!
            eprintln!(
                "Could not find a character at: {} in line {}.",
                self.current, self.line
            );
            process::exit(64);
        }
        c
    }

    fn string(&self) {}
}
