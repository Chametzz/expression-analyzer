use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Operator(Operator),
    LeftParen,
    RightParen,
}