use std::{
    char,
    collections::{self, HashMap},
};

use super::{
    token::{Literal, Token},
    ScanningError, TokenType,
};
use tracing::error;

#[derive(thiserror::Error, Debug)]
enum TokenError {
    #[error("[line {0}] Error: Unexpected character \"{1}\".")]
    InvalidToken(u64, char),
    #[error("[Line {0}] : Unterminated string")]
    UnterminatedString(u64),
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
    keywords: collections::HashMap<String, TokenType>,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
        }
    }

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                          helpers                           */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    fn is_finished(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn peek_next(&self) -> char {
        let curr = (self.current + 1) as usize;
        if curr >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(curr).expect("should be a char");
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

    /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
    /*                 consume lexeme related shi                 */
    /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

    fn advance(&self) -> char {
        // Should handle cleaner than with a unwrap
        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }

    fn string(&mut self) -> Result<(), TokenError> {
        while self.peek() != '"' && !self.is_finished() {
            if self.peek() == '\n' {
                self.advance();
            }
        }

        if self.is_finished() {
            return Err(TokenError::UnterminatedString(self.line));
        }

        self.advance();

        // Trim the surrounding quotes
        let string_len = self.source.len();
        let value: String = self
            .source
            .chars()
            .skip(1)
            .take(string_len - 2 as usize)
            .collect();
        self.add_token_with_literal(TokenType::String, Literal::Str(value));
        Ok(())
    }

    fn number(&mut self) -> Result<(), TokenError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let string_len = self.source.len();
        let raw: String = self
            .source
            .chars()
            .skip(1)
            .take(string_len - 2 as usize)
            .collect();
        let num = raw.parse::<f64>().expect("should be a float");
        self.add_token_with_literal(TokenType::Number, Literal::Number(num))
    }

    fn identifier(&mut self) -> TokenType {
        // TODO : understand quertos implementation
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let string_len = self.source.len();
        let raw: String = self
            .source
            .chars()
            .skip(1)
            .take(string_len - 2 as usize)
            .collect();

        match self.keywords.get(&raw) {
            Some(nature) => nature.clone(),
            None => TokenType::Identifier,
        }
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
            ' ' | '\r' | '\t' => return Ok(()),
            '\n' => {
                self.line += 1;
                return Ok(());
            }

            /*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
            /*                          Strings                           */
            /*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/
            '"' => {
                let _ = self.string();
                return Ok(());
            }

            _ => {
                if c.is_ascii_digit() {
                    let _ = self.number();
                    return Ok(());
                } else if self.is_alpha(c) {
                    self.identifier();
                }
                return Err(TokenError::InvalidToken(self.line, c));
            }
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
