use super::{token::{Literal, Token}, ScanningError, TokenType};
use tracing::error;

#[derive(thiserror::Error, Debug)]
enum TokenError {
    #[error("[line {0}] Error: Unexpected character \"{1}\".")]
    InvalidToken(u64, char),
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_finished(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn add_token(&mut self, nature: TokenType) -> Result<(), TokenError>{
        self.add_token_with_literal(nature, Literal::None);
        Ok(())
    }

    fn add_token_with_literal(&mut self, nature: TokenType, literal: Literal) -> Result<(), TokenError> {
        let text: String = self.source.chars().skip(self.start as usize).take(self.current as usize).collect();
        Ok(self.tokens.push(Token::new(nature,text,literal,self.line)))
    }

    fn advance(&self) -> char {
        // Should handle cleaner than with a unwrap 
        self.source.chars().nth((self.current + 1) as usize).unwrap()

    }

    fn scan_token(&mut self)-> Result<(), TokenError> {
        let c = self.advance();
        let token = match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => Err(TokenError::InvalidToken(self.line, c ))
        };

        Ok(())
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_finished() {
            self.start = self.current;
            let _ = self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::new(), super::Literal::None, self.line.try_into().unwrap()));
        return self.tokens.clone();
    }

}
