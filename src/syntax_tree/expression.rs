use std::vec;

use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Literal(Token),
    Grouping(Box<Expression>),
}

pub struct Parser {
    pub current: usize,
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparsion();
        while self.is_matching(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparsion();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn comparsion(&mut self) -> Expression {
        let mut expr = self.term();
        while self.is_matching(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.is_matching(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();
        while self.is_matching(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        if self.is_matching(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expression::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expression {
        if self.is_matching(vec![TokenType::False]) {
            return Expression::Literal(Token {
                token_type: TokenType::False,
                literal: None,
            });
        }
        if self.is_matching(vec![TokenType::True]) {
            return Expression::Literal(Token {
                token_type: TokenType::True,
                literal: None,
            });
        }
        if self.is_matching(vec![TokenType::Nil]) {
            return Expression::Literal(Token {
                token_type: TokenType::Nil,
                literal: None,
            });
        }

        if self.is_matching(vec![TokenType::Number, TokenType::String]) {
            return Expression::Literal(self.previous());
        }

        if self.is_matching(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            Expression::Grouping(Box::new(expr))
        } else {
            panic!("incomplete expression")
        }
    }

    fn consume(&mut self, tt: TokenType, msg: &str) {
        if self.check(tt) {
            self.advance();
            return;
        }
        panic!("{}", msg)
    }

    fn is_matching(&mut self, t_types: Vec<TokenType>) -> bool {
        for tt in t_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == tt
    }

    fn advance(&mut self) -> Token {
        if !(self.is_at_end()) {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        match self.tokens.get(self.current) {
            Some(token) => token,
            None => panic!("no token found!"),
        }
    }

    fn previous(&self) -> Token {
        match self.tokens.get(self.current - 1) {
            Some(token) => token.clone(),
            None => panic!("no token found!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::token::Literal;

    #[test]
    fn test_build_tree() {
        // 1 + 2

        let one = Token {
            token_type: TokenType::Number,
            literal: Some(Literal::Numeric(1.0)),
        };
        let plus = Token {
            token_type: TokenType::Plus,
            literal: None,
        };
        let two = Token {
            token_type: TokenType::Number,
            literal: Some(Literal::Numeric(2.0)),
        };
        let eof = Token {
            token_type: TokenType::Eof,
            literal: None,
        };

        let tokens: Vec<Token> = vec![one.clone(), plus.clone(), two.clone(), eof];

        let expected_tree = Expression::Binary {
            left: Box::new(Expression::Literal(one)),
            operator: plus,
            right: Box::new(Expression::Literal(two)),
        };

        let mut parser = Parser { current: 0, tokens };
        let tree = parser.expression();

        assert_eq!(tree, expected_tree);
    }
}
