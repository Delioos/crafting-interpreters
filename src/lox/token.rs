use core::fmt;
use std::fmt::Display;

use crate::lox::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
    True,
    False,
    None
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Identifier(s) => write!(f, "{}", s),
            Literal::Str(s) => write!(f, "\"{}\"", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::True => write!(f, "True"),
            Literal::False => write!(f, "False"),
            Literal::None => write!(f, "None"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    nature: TokenType,
    lexeme: String,
    literal: Literal,
    line: u8,
}

impl Token {
    pub fn new(nature: TokenType, lexeme: String, literal: Literal, line: u8) -> Self {
        Token {
            nature: nature,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(nature: {} - lexeme {} w/ literal-> {})",
            self.nature, self.lexeme, self.literal
        )
    }
}
