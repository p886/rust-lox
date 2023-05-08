use crate::scanner::token;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        for character in self.source.chars() {
            match scan_token(character) {
                Ok(scanned_token) => self.tokens.push(scanned_token),
                Err(err) => {
                    println!("Error scanning: {}", err);
                    // don't `break` here, keep scanning instead to report as many errors as possible
                }
            }
        }
    }
}

fn scan_token(character: char) -> Result<Token, String> {
    let tok: Token = match character {
        '(' => token::new(TokenType::LeftParen, String::from(character), None, 1),
        ')' => token::new(TokenType::RightParen, String::from(character), None, 1),
        '{' => token::new(TokenType::LeftBrace, String::from(character), None, 1),
        '}' => token::new(TokenType::RightBrace, String::from(character), None, 1),
        ',' => token::new(TokenType::Comma, String::from(character), None, 1),
        '.' => token::new(TokenType::Dot, String::from(character), None, 1),
        '-' => token::new(TokenType::Minus, String::from(character), None, 1),
        '+' => token::new(TokenType::Plus, String::from(character), None, 1),
        ';' => token::new(TokenType::Semicolon, String::from(character), None, 1),
        '*' => token::new(TokenType::Star, String::from(character), None, 1),
        _ => return Err(format!("Unexpected character {:?}", character)),
    };

    Ok(tok)
}
