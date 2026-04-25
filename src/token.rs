#[derive(Debug,Clone,Copy,PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub enum Value<'a> {
    Number(f64),
    String(&'a str),
    None
}

pub struct Token<'a> {
    pub tokentype: TokenType,
    lexeme: &'a str,
    literal: Value<'a>,
    line: u32,
}

impl<'a> Token<'a> {
    pub fn new(tokentype: TokenType, lexeme: &'a str, literal: Value<'a>, line: u32) -> Self {
        Self {
            tokentype,
            lexeme,
            literal,
            line,
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.tokentype, self.lexeme, self.literal)
    }
}
