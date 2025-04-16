use super::{token::Token, TokenType};

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u128,
    current: u128,
    line: u128,
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
        self.current >= self.source.len() as u128
    }

    fn scan_token(&self) {
        todo!()
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_finished() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::new(), super::Literal::None, self.line.try_into().unwrap()));
        return self.tokens.clone();
    }

}
