use crate::scanner::token::{Literal, Token};
use crate::scanner::token_type::TokenType;
use crate::syntax_tree::expression::Expression;

pub fn interpret(tree: &Expression) -> Expression {
    match tree {
        Expression::Unary { operator, right } => {
            let expr = interpret(&*right);
            match operator.token_type {
                TokenType::Minus => match expr {
                    Expression::Literal(lit) => {
                        let new_val = match lit.literal {
                            Some(val) => match val {
                                Literal::String(_) => {
                                    panic!("Cannot use `-` on a string")
                                }
                                Literal::Numeric(num) => -num,
                                Literal::Identifier(_) => panic!("Cannot use `-` on an identifier"),
                            },
                            None => todo!(),
                        };
                        return Expression::Literal(Token {
                            token_type: TokenType::Number,
                            literal: Some(Literal::Numeric(new_val)),
                        });
                    }
                    _ => interpret(&expr),
                },
                TokenType::Bang => Expression::Literal(Token {
                    token_type: TokenType::Bang,
                    literal: None,
                }),
                _ => Expression::Literal(Token {
                    token_type: TokenType::Bang,
                    literal: None,
                }),
            };
            expr
        }
        Expression::Binary {
            left,
            operator,
            right,
        } => todo!(),
        Expression::Literal(lit) => Expression::Literal(lit.clone()),
        Expression::Grouping(group) => interpret(&*group),
    }
}
