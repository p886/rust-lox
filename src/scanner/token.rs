use crate::scanner::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    // the enumerated type of token
    pub token_type: TokenType,
    // the actual string representation of that token
    pub lexeme: String,
    // the value containted in the token if it has one (i.e. String or Number literals)
    pub literal: Option<String>,
    // the line where the token is found
    pub line: i32,
}
