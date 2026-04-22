use crate::error;
use crate::token::Token;
use crate::token::TokenType;

pub struct Scanner<'a> {
    pub source: &'a str,
    tokens: Vec<Token<'a>>,

    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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

    fn add_token(&mut self, tokentype: TokenType, literal: Option<u8>) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(tokentype, text, literal.unwrap(), self.line));//FIX:
    }

    fn scan_token(&mut self) -> _ {
        self.add_token(
            match self.advance() {
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
                _ => {
                    error(self.line, "Unexpected character.");
                    return;
                }
            },
            None,
        );
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", 0, self.line));
        self.tokens
    }
}
