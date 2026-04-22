use std::collections::HashMap;

use crate::error;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::Value;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,

    start: usize,
    current: usize,
    line: u32,

    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let keywords = HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ]);

        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap(); // FIX:
        self.current += 1;
        c
    }

    fn advance_check(&mut self, expected: char) -> bool {
        match self.is_at_end() {
            true => false,
            false => {
                let c = self.source.chars().nth(self.current).unwrap(); // FIX:
                if c == expected {
                    self.current += 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn peek(&self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.source.chars().nth(self.current).unwrap(), // FIX:
        }
    }

    fn peek_next(&self) -> char {
        match self.current + 1 >= self.source.len() {
            true => '\0',
            false => self.source.chars().nth(self.current + 1).unwrap(), // FIX:
        }
    }

    fn add_token(&mut self, tokentype: TokenType, literal: Value<'a>) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(tokentype, text, literal, self.line)); //FIX:
    }

    fn scan_token(&mut self) {
        let tokentype = match self.advance() {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => match self.advance_check('=') {
                true => TokenType::BangEqual,
                false => TokenType::Bang,
            },
            '=' => match self.advance_check('=') {
                true => TokenType::EqualEqual,
                false => TokenType::Equal,
            },
            '>' => match self.advance_check('=') {
                true => TokenType::GreaterEqual,
                false => TokenType::Greater,
            },
            '<' => match self.advance_check('=') {
                true => TokenType::LessEqual,
                false => TokenType::Less,
            },
            '/' => match self.peek() { // COMMENTS
                '/' => {
                    loop {
                        match self.advance() {
                            '\n' | '\0' => break,
                            _=>{}
                        };
                    }
                    return;
                }
                '*' => {
                    loop {
                        match self.advance() {
                            '\0' => break,
                            '*' => {
                                if self.peek() == '/' {
                                    self.advance();
                                    break;
                                };
                            },
                            _=>{}
                        };
                    }
                    return;
                }
                _ => TokenType::Slash,
            },
            ' ' | '\r' | '\t' => return,
            '\n' => {
                self.line += 1;
                return;
            }
            '"' => {
                self.string();
                return;
            }
            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphanumeric() {
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character.");
                }
                return;
            }
        };
        self.add_token(tokentype, Value::None);
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start + 1..self.current - 1];
        let tokentype = self
            .keywords
            .get(text)
            .copied()
            .unwrap_or(TokenType::Identifier);
        self.add_token(tokentype, Value::None);
    }

    fn number(&mut self) {
        loop {
            match self.peek() {
                '.' => {
                    if !self.peek_next().is_ascii_digit() {
                        break;
                    }
                }
                c => {
                    if !c.is_ascii_digit() {
                        break;
                    }
                }
            };
            self.advance();
        }

        let s = &self.source[self.start + 1..self.current - 1];
        self.add_token(
            TokenType::Number,
            Value::Number(s.parse::<f64>().unwrap()), //FIX:
        );
    }

    fn string(&mut self) {
        loop {
            match self.peek() {
                '\n' => self.line += 1,
                '\0' => {
                    error(self.line, "Unterminated string.");
                    return;
                }
                '"' => {
                    self.advance();
                    self.add_token(
                        TokenType::String,
                        Value::String(&self.source[self.start + 1..self.current - 1]),
                    );
                    return;
                }
                _ => {}
            };
            self.advance();
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", Value::None, self.line));
        std::mem::take(&mut self.tokens)
    }
}
