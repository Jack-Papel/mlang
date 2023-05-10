use crate::prelude::*;
use crate::constructs::ast::{BinaryOperator, UnaryOperator};

mod span;
use span::Span;
pub mod symbol;
use symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, 
    LEFT_SQR_BRACE, RIGHT_SQR_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    PERCENT,

    // Iteration/Match-related tokens
    BAR(/*index:*/ usize), COLON, TILDE, 
    DOLLAR, AT, HASH, 
    TRIPLE_AMP, TRIPLE_BAR,
    DOT_DOT,

    // Reserved, but unused, tokens
    AMP, CARET, QUESTION,

    // One or two character tokens.
    EXCLAMATION, EXCLAMATION_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    DOUBLE_AMP, DOUBLE_BAR,
    COLON_COLON,

    // Literals.
    IDENTIFIER(Symbol), 
    Literal(Literal),

    // Whitespace.
    NEWLINE(usize),

    // Keywords.
    Keyword(Symbol),
}

pub enum Delimiter {
    Paren,
    Brace,
    Bracket
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralKind {
    Bool,
    String,
    Int,
    Float
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Literal {
    pub kind: LiteralKind,
    pub symbol: Symbol
}

impl TokenKind {
    pub fn as_binary_operator(&self) -> Result<BinaryOperator> {
        match self {
            TokenKind::DOT_DOT => Ok(BinaryOperator::Range),
            TokenKind::DOLLAR => Ok(BinaryOperator::ForEach),
            TokenKind::AT => Ok(BinaryOperator::Map),
            TokenKind::HASH => Ok(BinaryOperator::Filter),
            TokenKind::TRIPLE_AMP => Ok(BinaryOperator::All),
            TokenKind::TRIPLE_BAR => Ok(BinaryOperator::Any),
            TokenKind::EXCLAMATION_EQUAL => Ok(BinaryOperator::NotEqual),
            TokenKind::EQUAL_EQUAL => Ok(BinaryOperator::Equal),
            TokenKind::GREATER => Ok(BinaryOperator::Greater),
            TokenKind::GREATER_EQUAL => Ok(BinaryOperator::GreaterEqual),
            TokenKind::LESS => Ok(BinaryOperator::Less),
            TokenKind::LESS_EQUAL => Ok(BinaryOperator::LessEqual),
            TokenKind::DOUBLE_AMP => Ok(BinaryOperator::And),
            TokenKind::DOUBLE_BAR => Ok(BinaryOperator::Or),
            TokenKind::PERCENT => Ok(BinaryOperator::Mod),
            TokenKind::STAR => Ok(BinaryOperator::Mul),
            TokenKind::SLASH => Ok(BinaryOperator::Div),
            TokenKind::PLUS => Ok(BinaryOperator::Plus),
            TokenKind::MINUS => Ok(BinaryOperator::Minus),
            _ => parse_err!("\"{self:?}\" is not a binary operator"),
        }
    }

    pub(in super::super) fn as_unary_operator(&self) -> Result<UnaryOperator> {
        match self {
            TokenKind::MINUS => Ok(UnaryOperator::Minus),
            TokenKind::EXCLAMATION => Ok(UnaryOperator::Not),
            _ => parse_err!("\"{self:?}\" is not a unary operator"),
        }
    }
}