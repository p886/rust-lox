use crate::scanner::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    // the enumerated type of token
    pub token_type: TokenType,
    // the value containted in the token if it has one (i.e. String or Number literals)
    pub literal: Option<Literal>,
    // the line where the token is found
    // pub line: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Numeric(f64),
    Identifier(String),
}
