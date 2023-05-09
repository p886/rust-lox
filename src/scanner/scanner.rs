use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub fn scan_tokens(source: String) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

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
                        // it's a comment...
                        for next_char in chars.by_ref() {
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
            Ok(tt) => tokens.push(Token {
                token_type: tt,
                lexeme: String::from("todo"),
                literal: None,
                line: 1,
            }),
            Err(msg) => return Err(msg),
        }
    }
    Ok(tokens)
}

fn double_lexeme(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    single_type: TokenType,
    double_type: TokenType,
) -> Result<TokenType, String> {
    match chars.peek() {
        Some('=') => {
            chars.next();
            Ok(double_type)
        }
        None => Ok(single_type),
        _ => Ok(single_type),
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_scan_tokens_all_success() {
        let tokens = match scan_tokens(String::from("   !,.- + != <= >=\n\n\n\n ==\t !\r<>}{()   "))
        {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

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

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, _) in tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, expected_tokens[i].token_type);
        }
    }

    #[test]
    fn test_scan_tokens_unexpected_character() {
        match scan_tokens(String::from("?")) {
            Ok(tokens) => {
                assert!(tokens.len() == 0)
            }
            Err(err) => {
                assert_eq!("unrecognized character '?'", err)
            }
        };
    }
}
