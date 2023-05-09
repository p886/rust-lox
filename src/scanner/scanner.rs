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
                '!' => match double_lexeme(&mut chars, TokenType::Bang, TokenType::BangEqual) {
                    Some(value) => value,
                    None => break,
                },
                '=' => match double_lexeme(&mut chars, TokenType::Equal, TokenType::EqualEqual) {
                    Some(value) => value,
                    None => break,
                },
                '<' => match double_lexeme(&mut chars, TokenType::Less, TokenType::LessEqual) {
                    Some(value) => value,
                    None => break,
                },
                '>' => match double_lexeme(&mut chars, TokenType::Greater, TokenType::GreaterEqual)
                {
                    Some(value) => value,
                    None => break,
                },
                '/' => match chars.peek() {
                    Some(peeked_char) => match peeked_char {
                        '/' => {
                            // comments
                            while let Some(next_char) = chars.next() {
                                // consume all characters after the comment until the newline
                                if next_char == '\n' {
                                    break;
                                }
                            }
                            Ok(TokenType::Comment)
                        }
                        _ => {
                            chars.next();
                            Ok(TokenType::Slash)
                        }
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
                Err(msg) => println!("{}", msg),
            }
        }
    }
}

fn double_lexeme(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    single_type: TokenType,
    double_type: TokenType,
) -> Option<Result<TokenType, String>> {
    match chars.peek() {
        Some(next_char) => match next_char {
            '=' => {
                chars.next();
                Some(Ok(double_type))
            }
            _ => Some(Ok(single_type)),
        },
        None => None,
    }
}
