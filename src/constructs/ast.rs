use super::variable::{Builtin, Value, Type};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    RANGE,
    FOR_EACH,
    MAP,
    FILTER,
    ALL,
    ANY,
    NOT_EQUAL,
    EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    AND,
    OR,
    MOD,
    MUL,
    DIV,
    PLUS,
    MINUS,
}

impl BinaryOperator {
    pub fn get_precedence_map() -> [Vec<BinaryOperator>; 10] {
        [
            vec![BinaryOperator::RANGE],
            vec![BinaryOperator::FOR_EACH, BinaryOperator::MAP, BinaryOperator::FILTER],
            vec![BinaryOperator::ALL, BinaryOperator::ANY],
            vec![BinaryOperator::MOD],
            vec![BinaryOperator::MUL, BinaryOperator::DIV],
            vec![BinaryOperator::PLUS, BinaryOperator::MINUS],
            vec![BinaryOperator::NOT_EQUAL, BinaryOperator::EQUAL],
            vec![BinaryOperator::GREATER, BinaryOperator::GREATER_EQUAL, BinaryOperator::LESS, BinaryOperator::LESS_EQUAL],
            vec![BinaryOperator::AND],
            vec![BinaryOperator::OR],
        ]
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    MINUS,
    NOT,
}

impl UnaryOperator {
    pub const VALUES : [UnaryOperator; 2] = [
        UnaryOperator::MINUS, 
        UnaryOperator::NOT
    ];
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Let(Identifier, Expression),
    Set(Identifier, Expression),
    Return(Expression),
    Break(Option<Expression>),
    Continue,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub identifier: Option<Identifier>,
    pub typ: Option<Type>,
    pub guard: Option<Expression>,
}

#[derive(Debug, Clone)]
pub enum Function {
    Match {
        arms: Vec<MatchArm>,
    },
    Builtin(Builtin)
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Value),
    Identifier(Identifier),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Call(Box<Expression>, Box<Expression>),
    // Grouping(Box<Expression>),
    // Call(Box<Expression>, Vec<Expression>),
    // If(Box<Expression>, Box<Block>, Option<Box<Block>>),
    // While(Box<Expression>, Box<Block>),
    // For(Identifier, Box<Expression>, Box<Expression>, Box<Block>),
}