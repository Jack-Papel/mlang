use crate::prelude::*;
use crate::constructs::ast::{BinaryOperator, UnaryOperator};

pub mod span;
use span::Span;
pub mod symbol;
use symbol::Symbol;
mod tokens;
pub use tokens::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Token (pub TokenKind, pub Span);

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen, RightParen, 
    LeftSqrBrace, RightSqrBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Percent,

    // Iteration/Match-related tokens
    Bar(/*index:*/ usize), Colon, Tilde, 
    Dollar, At, Hash, 
    TripleAmp, TripleBar,
    DotDot,

    // Reserved, but unused, tokens
    Amp, Caret, Question,

    // One or two character tokens.
    Exclamation, ExclamationEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    DoubleAmp, DoubleBar,
    ColonColon,

    // Literals.
    Identifier(Symbol), 
    Literal(Literal),

    // Whitespace.
    Newline(usize),

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
    pub fn as_binary_operator(&self, span: Option<Span>) -> Result<BinaryOperator> {
        match self {
            TokenKind::DotDot => Ok(BinaryOperator::Range),
            TokenKind::Dollar => Ok(BinaryOperator::ForEach),
            TokenKind::At => Ok(BinaryOperator::Map),
            TokenKind::Hash => Ok(BinaryOperator::Filter),
            TokenKind::TripleAmp => Ok(BinaryOperator::All),
            TokenKind::TripleBar => Ok(BinaryOperator::Any),
            TokenKind::ExclamationEqual => Ok(BinaryOperator::NotEqual),
            TokenKind::EqualEqual => Ok(BinaryOperator::Equal),
            TokenKind::Greater => Ok(BinaryOperator::Greater),
            TokenKind::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            TokenKind::Less => Ok(BinaryOperator::Less),
            TokenKind::LessEqual => Ok(BinaryOperator::LessEqual),
            TokenKind::DoubleAmp => Ok(BinaryOperator::And),
            TokenKind::DoubleBar => Ok(BinaryOperator::Or),
            TokenKind::Percent => Ok(BinaryOperator::Mod),
            TokenKind::Star => Ok(BinaryOperator::Mul),
            TokenKind::Slash => Ok(BinaryOperator::Div),
            TokenKind::Plus => Ok(BinaryOperator::Plus),
            TokenKind::Minus => Ok(BinaryOperator::Minus),
            _ => semantic_err!(span, "\"{self:?}\" is not a binary operator"),
        }
    }

    pub(in super::super) fn as_unary_operator(&self, span: Option<Span>) -> Result<UnaryOperator> {
        match self {
            TokenKind::Minus => Ok(UnaryOperator::Minus),
            TokenKind::Exclamation => Ok(UnaryOperator::Not),
            _ => semantic_err!(span, "\"{self:?}\" is not a unary operator"),
        }
    }
}