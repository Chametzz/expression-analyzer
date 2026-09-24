use super::token::Token;

#[derive(Debug, Clone)]
pub struct Expression {
    raw: String,
    tokens: Vec<Token>,
}

impl Expression {
    pub fn new(raw: String, tokens: Vec<Token>) -> Self {
        Self { raw, tokens }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}