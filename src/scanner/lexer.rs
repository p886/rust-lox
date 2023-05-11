use crate::scanner::token::Literal;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub fn scan_tokens(source: String) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(char) = chars.next() {
        if char.is_whitespace() {
            continue;
        }
        let token_result: Result<Token, String> = match char {
            '(' => make_simple_token(TokenType::LeftParen),
            ')' => make_simple_token(TokenType::RightParen),
            '{' => make_simple_token(TokenType::LeftBrace),
            '}' => make_simple_token(TokenType::RightBrace),
            ',' => make_simple_token(TokenType::Comma),
            '.' => make_simple_token(TokenType::Dot),
            '-' => make_simple_token(TokenType::Minus),
            '+' => make_simple_token(TokenType::Plus),
            ';' => make_simple_token(TokenType::Semicolon),
            '*' => make_simple_token(TokenType::Star),
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
                        make_simple_token(TokenType::Comment)
                    }
                    _ => {
                        chars.next();
                        make_simple_token(TokenType::Slash)
                    }
                },
                None => make_simple_token(TokenType::Slash),
            },
            '"' => {
                let mut elements: Vec<String> = Vec::new();
                let mut terminated = false;
                for next_char in chars.by_ref() {
                    if next_char == '"' {
                        terminated = true;
                        break;
                    }
                    elements.push(next_char.to_string());
                }
                if !terminated {
                    return Err(String::from("Unterminated string"));
                }
                let joined = elements.join("");
                Ok(Token {
                    token_type: TokenType::String,
                    literal: Some(Literal::String(joined)),
                })
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut num_parts: Vec<String> = Vec::new();
                let mut contains_dot = false;

                num_parts.push(char.to_string());
                for c in chars.by_ref() {
                    if c == '.' && !contains_dot {
                        contains_dot = true;
                    }
                    if c != '.' && (c.is_alphabetic() || c.is_whitespace()) {
                        break;
                    }
                    num_parts.push(c.to_string());
                }

                let lit = num_parts.join("").to_string().parse::<f64>().unwrap();
                Ok(Token {
                    token_type: TokenType::Number,
                    literal: Some(Literal::Numeric(lit)),
                })
            }
            'A'..='Z' | 'a'..='z' | '_' => {
                let mut parts: Vec<String> = Vec::new();

                parts.push(char.to_string());

                for c in chars.by_ref() {
                    if !(c.is_alphabetic()) && c != '_' {
                        break;
                    }
                    parts.push(c.to_string());
                }
                let raw_token = parts.join("");

                let typ = match raw_token.as_str() {
                    "and" => TokenType::And,
                    "class" => TokenType::Class,
                    "else" => TokenType::Else,
                    "fun" => TokenType::Fun,
                    "for" => TokenType::For,
                    "if" => TokenType::If,
                    "nil" => TokenType::Nil,
                    "or" => TokenType::Or,
                    "print" => TokenType::Print,
                    "return" => TokenType::Return,
                    "super" => TokenType::Super,
                    "this" => TokenType::This,
                    "true" => TokenType::True,
                    "var" => TokenType::Var,
                    "while" => TokenType::While,
                    &_ => TokenType::Identifier,
                };

                let lit = if typ == TokenType::Identifier {
                    Some(Literal::Identifier(raw_token))
                } else {
                    None
                };

                Ok(Token {
                    token_type: typ,
                    literal: lit,
                })
            }
            _ => Err(format!("unrecognized character {:?}", char)),
        };

        match token_result {
            Ok(t) => tokens.push(t),
            Err(msg) => return Err(msg),
        }
    }
    Ok(tokens)
}

fn double_lexeme(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    single_type: TokenType,
    double_type: TokenType,
) -> Result<Token, String> {
    let tt = match chars.peek() {
        Some('=') => {
            chars.next();
            double_type
        }
        None => single_type,
        _ => single_type,
    };
    Ok(Token {
        token_type: tt,
        literal: None,
    })
}

fn make_simple_token(tt: TokenType) -> Result<Token, String> {
    Ok(Token {
        token_type: tt,
        literal: None,
    })
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    fn make_test_token(tt: TokenType) -> Token {
        Token {
            token_type: tt,
            literal: None,
        }
    }

    #[test]
    fn test_scan_tokens_all_success() {
        let tokens = match scan_tokens(String::from(
            "   !,.- + != <= >=\n\n\n\n ==\t !\r<>}{()   / //\n;\"fo\no\" 1 6.78",
        )) {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

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
            make_test_token(TokenType::Slash),
            make_test_token(TokenType::Comment),
            make_test_token(TokenType::Semicolon),
            Token {
                token_type: TokenType::String,
                literal: Some(Literal::String(String::from("fo\no"))),
            },
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(1.0)),
            },
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(6.78)),
            },
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
                assert!(tokens.is_empty())
            }
            Err(err) => {
                assert_eq!("unrecognized character '?'", err)
            }
        };
    }

    #[test]
    fn test_scan_tokens_comments() {
        let tokens = match scan_tokens(String::from("// a comment \n+")) {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

        let expected_tokens = vec![
            make_test_token(TokenType::Comment),
            make_test_token(TokenType::Plus),
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, _) in tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, expected_tokens[i].token_type);
        }
    }

    #[test]
    fn test_scan_tokens_literals() {
        let tokens = match scan_tokens(String::from("\"helloworld\"")) {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

        let expected_tokens = vec![Token {
            token_type: TokenType::String,
            literal: Some(Literal::String(String::from("helloworld"))),
        }];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, _) in tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, expected_tokens[i].token_type);
        }
    }

    #[test]
    fn test_scan_tokens_unterminated_string() {
        match scan_tokens(String::from("\"helloworld")) {
            Ok(tokens) => assert!(tokens.is_empty()),
            Err(err) => {
                assert_eq!(err, "Unterminated string")
            }
        };
    }

    #[test]
    fn test_scan_tokens_multiline_literals() {
        let tokens = match scan_tokens(String::from("\"hello\nworld\"")) {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

        let expected_tokens = vec![Token {
            token_type: TokenType::String,
            literal: Some(Literal::String(String::from("hello\nworld"))),
        }];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, _) in tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, expected_tokens[i].token_type);
        }
    }

    #[test]
    fn test_scan_tokens_number_literals() {
        let tokens = match scan_tokens(String::from("1.0 23433434.1233456 1 542 2348923409")) {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

        let expected_tokens = vec![
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(1.0)),
            },
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(23433434.123456)),
            },
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(1.0)),
            },
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(542.0)),
            },
            Token {
                token_type: TokenType::Number,
                literal: Some(Literal::Numeric(2348923409.0)),
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, _) in tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, expected_tokens[i].token_type);
        }
    }

    #[test]
    fn test_scan_tokens_identifiers_keywords() {
        let tokens = match scan_tokens(String::from(
            "test and class else fun for if nil or print return super this true var while _my_variable_",
        )) {
            Ok(tokens) => tokens,
            Err(err) => panic!("Unexpected error in test: {}", err),
        };

        let expected_tokens = vec![
            Token {
                token_type: TokenType::Identifier,
                literal: Some(Literal::Identifier(String::from("test"))),
            },
            Token {
                token_type: TokenType::And,
                literal: None,
            },
            Token {
                token_type: TokenType::Class,
                literal: None,
            },
            Token {
                token_type: TokenType::Else,
                literal: None,
            },
            Token {
                token_type: TokenType::Fun,
                literal: None,
            },
            Token {
                token_type: TokenType::For,
                literal: None,
            },
            Token {
                token_type: TokenType::If,
                literal: None,
            },
            Token {
                token_type: TokenType::Nil,
                literal: None,
            },
            Token {
                token_type: TokenType::Or,
                literal: None,
            },
            Token {
                token_type: TokenType::Print,
                literal: None,
            },
            Token {
                token_type: TokenType::Return,
                literal: None,
            },
            Token {
                token_type: TokenType::Super,
                literal: None,
            },
            Token {
                token_type: TokenType::This,
                literal: None,
            },
            Token {
                token_type: TokenType::True,
                literal: None,
            },
            Token {
                token_type: TokenType::Var,
                literal: None,
            },
            Token {
                token_type: TokenType::While,
                literal: None,
            },
            Token {
                token_type: TokenType::Identifier,
                literal: Some(Literal::Identifier(String::from("_my_variable_"))),
            },
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, _) in tokens.iter().enumerate() {
            assert_eq!(tokens[i].token_type, expected_tokens[i].token_type);
            assert_eq!(tokens[i].literal, expected_tokens[i].literal);
        }
    }
}
