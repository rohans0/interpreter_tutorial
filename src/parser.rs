use crate::token::{Token, TokenType};

struct Expr {}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }
    fn expression(&self) -> Expr {
        self.equality()
    }
    fn equality(&self) -> Expr {
        let expr: Expr = comparison();
        while self.advance_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = previous();
            let right: Expr = comparison();
            //expr = new Expr.Binary(expr, operator, right);
        }
        expr
    }

    fn advance_match(&self, types: &[TokenType]) -> bool {
        for t in types {
            if check(t) {
                advance();
                return true;
            }
        }
        false
    }
}
