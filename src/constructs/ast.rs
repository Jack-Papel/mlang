use super::{variable::{Builtin, Value, Type}};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Range,
    ForEach,
    Map,
    Filter,
    All,
    Any,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Mod,
    Mul,
    Div,
    Plus,
    Minus,
}

impl BinaryOperator {
    pub fn get_precedence_map() -> [Vec<BinaryOperator>; 10] {
        [
            vec![BinaryOperator::Range],
            vec![BinaryOperator::ForEach, BinaryOperator::Map, BinaryOperator::Filter],
            vec![BinaryOperator::All, BinaryOperator::Any],
            vec![BinaryOperator::Mod],
            vec![BinaryOperator::Mul, BinaryOperator::Div],
            vec![BinaryOperator::Plus, BinaryOperator::Minus],
            vec![BinaryOperator::NotEqual, BinaryOperator::Equal],
            vec![BinaryOperator::Greater, BinaryOperator::GreaterEqual, BinaryOperator::Less, BinaryOperator::LessEqual],
            vec![BinaryOperator::And],
            vec![BinaryOperator::Or],
        ]
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus,
    Not,
}

impl UnaryOperator {
    pub const VALUES : [UnaryOperator; 2] = [
        UnaryOperator::Minus, 
        UnaryOperator::Not
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