use super::{
    token::{Literal, Token},
    ScanningError, TokenType,
};
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

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn add_token(&mut self, nature: TokenType) -> Result<(), TokenError> {
        self.add_token_with_literal(nature, Literal::None);
        Ok(())
    }

    fn add_token_with_literal(
        &mut self,
        nature: TokenType,
        literal: Literal,
    ) -> Result<(), TokenError> {
        let text: String = self
            .source
            .chars()
            .skip(self.start as usize)
            .take(self.current as usize)
            .collect();
        Ok(self
            .tokens
            .push(Token::new(nature, text, literal, self.line)))
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_finished() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn advance(&self) -> char {
        // Should handle cleaner than with a unwrap
        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }

    fn string(&self) -> TokenType {
        todo!()
    }

    fn scan_token(&mut self) -> Result<(), TokenError> {
        let c = self.advance();
        let token = match c {
            /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
            /*                        single char                         */
            /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
            /*                         Operators                          */
            /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
            '!' => {
                if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
            /*                       longer lexemes                       */
            /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
            '/' => {
                if self.match_next('/') {
                    // A comment oes until the end of the line.
                    loop {
                        if self.peek() != '\n' && !self.is_finished() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    // idk how to handle this tbh TODO: check this implementation effect
                    return Ok(());
                } else {
                    TokenType::Slash
                }
            }
            /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
            /*                       useless chars                        */
            /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
            ' ' => return Ok(()),
            '\r' => return Ok(()),
            '\t' => return Ok(()),
            '\n' => {
                self.line += 1;
                return Ok(());
            }

            /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
            /*                          Strings                           */
            /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
            '"' => self.string(),

            _ => return Err(TokenError::InvalidToken(self.line, c)),
        };

        let _ = self.add_token(token);

        Ok(())
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_finished() {
            self.start = self.current;
            let _ = self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            super::Literal::None,
            self.line.try_into().unwrap(),
        ));
        return self.tokens.clone();
    }
}
