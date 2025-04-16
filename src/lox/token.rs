use core::fmt;
use std::fmt::Display;

use crate::lox::token_type::TokenType;

pub struct Token<T> {
    nature: TokenType,
    lexeme: String,
    literal: T,
    line: u8
}

impl<T> Token<T> {
    fn new(nature: TokenType, lexeme: String, literal: T, line: u8) -> Self {
        Token { nature: nature, lexeme: lexeme, literal: literal, line: line }
    }
}

impl<T> fmt::Display for Token<T> 
where T: Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(nature: {} - lexeme {} w/ literal-> {})", self.nature, self.lexeme, self.literal)
    }

}

