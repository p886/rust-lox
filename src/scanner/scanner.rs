use crate::scanner::token;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        let mut chars = self.source.chars().peekable();

        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                continue;
            }
            let token_result: Result<TokenType, String> = match char {
                '(' => Ok(TokenType::LeftParen),
                ')' => Ok(TokenType::RightParen),
                '{' => Ok(TokenType::LeftBrace),
                '}' => Ok(TokenType::RightBrace),
                ',' => Ok(TokenType::Comma),
                '.' => Ok(TokenType::Dot),
                '-' => Ok(TokenType::Minus),
                '+' => Ok(TokenType::Plus),
                ';' => Ok(TokenType::Semicolon),
                '*' => Ok(TokenType::Star),
                '!' => match chars.peek() {
                    Some(next_char) => match next_char {
                        '=' => {
                            chars.next();
                            Ok(TokenType::BangEqual)
                        }
                        _ => Ok(TokenType::Bang),
                    },
                    None => break,
                },
                '=' => match chars.peek() {
                    Some(next_char) => match next_char {
                        '=' => {
                            chars.next();
                            Ok(TokenType::EqualEqual)
                        }
                        _ => Ok(TokenType::Equal),
                    },
                    None => break,
                },
                '<' => match chars.peek() {
                    Some(next_char) => match next_char {
                        '=' => {
                            chars.next();
                            Ok(TokenType::LessEqual)
                        }
                        _ => Ok(TokenType::Less),
                    },
                    None => break,
                },
                '>' => match chars.peek() {
                    Some(next_char) => match next_char {
                        '=' => {
                            chars.next();
                            Ok(TokenType::GreaterEqual)
                        }
                        _ => Ok(TokenType::Greater),
                    },
                    None => break,
                },
                _ => Err(format!("unrecognized character {:?}", char)),
            };

            match token_result {
                Ok(tt) => self.tokens.push(Token {
                    token_type: tt,
                    lexeme: String::from("todo"),
                    literal: None,
                    line: 1,
                }),
                Err(msg) => {
                    println!("{}", msg);
                }
            }
        }
    }
}
