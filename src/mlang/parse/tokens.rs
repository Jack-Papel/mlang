use crate::mlang::ast::{BinaryOperator, UnaryOperator};
use crate::prelude::*;

#[allow(non_camel_case_types, clippy::upper_case_acronyms, unused)]

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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
    IDENTIFIER(String), STRING(String), INT(isize), FLOAT(f64),

    // Whitespace.
    NEWLINE(usize),

    // Keywords.
    STRUCT, IMPL, FALSE,
    YIELD, TRUE, LET, RETURN,
}

impl Token {
    pub fn as_binary_operator(&self) -> Result<BinaryOperator> {
        match self {
            Token::DOT_DOT => Ok(BinaryOperator::RANGE),
            Token::DOLLAR => Ok(BinaryOperator::FOR_EACH),
            Token::AT => Ok(BinaryOperator::MAP),
            Token::HASH => Ok(BinaryOperator::FILTER),
            Token::TRIPLE_AMP => Ok(BinaryOperator::ALL),
            Token::TRIPLE_BAR => Ok(BinaryOperator::ANY),
            Token::EXCLAMATION_EQUAL => Ok(BinaryOperator::NOT_EQUAL),
            Token::EQUAL_EQUAL => Ok(BinaryOperator::EQUAL),
            Token::GREATER => Ok(BinaryOperator::GREATER),
            Token::GREATER_EQUAL => Ok(BinaryOperator::GREATER_EQUAL),
            Token::LESS => Ok(BinaryOperator::LESS),
            Token::LESS_EQUAL => Ok(BinaryOperator::LESS_EQUAL),
            Token::DOUBLE_AMP => Ok(BinaryOperator::AND),
            Token::DOUBLE_BAR => Ok(BinaryOperator::OR),
            Token::PERCENT => Ok(BinaryOperator::MOD),
            Token::STAR => Ok(BinaryOperator::MUL),
            Token::SLASH => Ok(BinaryOperator::DIV),
            Token::PLUS => Ok(BinaryOperator::PLUS),
            Token::MINUS => Ok(BinaryOperator::MINUS),
            _ => parse_err!("\"{self:?}\" is not a binary operator"),
        }
    }

    pub(in super::super) fn as_unary_operator(&self) -> Result<UnaryOperator> {
        match self {
            Token::MINUS => Ok(UnaryOperator::MINUS),
            Token::EXCLAMATION => Ok(UnaryOperator::NOT),
            _ => parse_err!("\"{self:?}\" is not a unary operator"),
        }
    }
}