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
        let expr: Expr = self.comparison();
        while self.advance_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: &Token = self.previous();
            let right: Expr = self.comparison();
            //expr = new Expr.Binary(expr, operator, right);
        }
        expr
    }

    fn comparison(&self) -> Expr {
        let expr: Expr = self.term();

        while self.advance_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right: Expr = self.term();
            expr = Expr::new(/*expr, operator, right*/)
        }
        expr
    }

    fn term(&self) -> Expr {
        let expr : Expr = self.factor();

        while self.advance_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator: &Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::new(/*expr, operator, right*/)
        }

        expr
    }

    fn factor(&self) -> Expr {
        let expr : Expr = self.unary();

        while self.advance_match(&[TokenType::Slash, TokenType::Star]) {
            let operator: &Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::new(/*expr, operator, right*/)
        }

        expr
    }


    fn unary(&self) -> Expr {
        if self.advance_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator: &Token = self.previous();
            let right: Expr = self.unary();
            return Expr::new(/*operator, right*/)
        }

        self.primary()
    }

    fn primary(&self) -> Expr {
        match self.advance().tokentype {
            TokenType::False => Expr::new(),
            TokenType::True => Expr::new(),
            TokenType::Nil => Expr::new(),
            TokenType::Number, TokenType::String => {

            }
            TokenType::LeftParen => {
                let expr:Expr = self.expression();
              self.consume(RIGHT_PAREN, "Expect ')' after expression.");
              return new Expr.Grouping(expr);
            }
            _=>todo!()
        }

    }

    fn advance_match(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tokentype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tokentype == tokentype
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().tokentype == TokenType::Eof
    }
    fn peek(&self) -> &Token<'a> {
        self.tokens.get(self.current).unwrap() //FIX:
    }

    fn previous(&self) -> &Token<'a> {
        self.tokens.get(self.current - 1).unwrap() //FIX:
    }
}
