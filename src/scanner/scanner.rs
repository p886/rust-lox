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
                '!' => double_lexeme(&mut chars, TokenType::Bang, TokenType::BangEqual),
                '=' => double_lexeme(&mut chars, TokenType::Equal, TokenType::EqualEqual),
                '<' => double_lexeme(&mut chars, TokenType::Less, TokenType::LessEqual),
                '>' => double_lexeme(&mut chars, TokenType::Greater, TokenType::GreaterEqual),
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
                    None => Ok(TokenType::Slash),
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
) -> Result<TokenType, String> {
    match chars.peek() {
        Some(next_char) => match next_char {
            '=' => {
                chars.next();
                Ok(double_type)
            }
            _ => Ok(single_type),
        },
        None => Ok(single_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens_all_success() {
        let mut scnr = Scanner {
            source: String::from("   !,.- + != <= >=\n\n\n\n ==\t !\r<>}{()   "),
            tokens: Vec::new(),
        };
        scnr.scan_tokens();

        fn make_test_token(tt: TokenType) -> Token {
            Token {
                token_type: tt,
                lexeme: String::from("todo"),
                literal: None,
                line: 1,
            }
        }

        let expected_tokens = vec![
            make_test_token(TokenType::Bang),
            make_test_token(TokenType::Comma),
            make_test_token(TokenType::Dot),
            make_test_token(TokenType::Minus),
            make_test_token(TokenType::Plus),
            make_test_token(TokenType::BangEqual),
            make_test_token(TokenType::LessEqual),
            make_test_token(TokenType::GreaterEqual),
            make_test_token(TokenType::EqualEqual),
            make_test_token(TokenType::Bang),
            make_test_token(TokenType::Less),
            make_test_token(TokenType::Greater),
            make_test_token(TokenType::RightBrace),
            make_test_token(TokenType::LeftBrace),
            make_test_token(TokenType::LeftParen),
            make_test_token(TokenType::RightParen),
        ];

        assert_eq!(scnr.tokens.len(), expected_tokens.len());

        for (i, _) in scnr.tokens.iter().enumerate() {
            assert_eq!(scnr.tokens[i].token_type, expected_tokens[i].token_type);
        }
    }
}
