use crate::prelude::*;
use super::ast::Function;
use super::token::span::Span;
use super::token::{Literal, LiteralKind};


#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Boolean,
    Match,
    Tuple(Vec<Type>),
    List(Box<Type>),
    Iter,
    Builtin,
    None,
}

impl Type {
    pub fn from_id(id: &str) -> Option<Type> {
        match id {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            "bool" => Some(Type::Boolean),
            "iter" => Some(Type::Iter),
            "match" => Some(Type::Match),
            "tuple" => Some(Type::Tuple(Vec::new())),
            "none" => Some(Type::None),
            s => {
                let mut chars = s.chars();
                if let Some(('[', ']')) = Option::zip(chars.next(), chars.last()) {
                    return Some(Type::List(Box::new(Type::from_id(&s[1..s.len() - 1])?)));
                }
                None
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(isize),
    Float(f64),
    String(String),
    Boolean(bool),
    IntRange(isize, isize),
    Function(Function),
    Tuple(Box<[Value]>),
    List(Vec<Value>),
    Filter(Box<Value>, Function),
    Map(Box<Value>, Function),
    None,
}

impl TryFrom<(Literal, Span)> for Value {
    type Error = MLGError;

    fn try_from((lit, span): (Literal, Span)) -> std::result::Result<Self, Self::Error> {
        let symbol = lit.symbol.get_str();

        Ok(match lit.kind {
            LiteralKind::String => Value::String(symbol.to_string()),
            LiteralKind::Int => Value::Int(symbol.parse().or(syntax_err!(Some(span), "Failed to parse int {}", symbol))?),
            LiteralKind::Float => Value::Float(symbol.parse().or(syntax_err!(Some(span), "Failed to parse float {}", symbol))?),
            LiteralKind::Bool => Value::Boolean(symbol.parse().or(syntax_err!(Some(span), "Failed to parse bool {}", symbol))?),
        })
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(ff) => write!(f, "{}", ff),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::IntRange(i1, i2) => write!(f, "{}..{}", i1, i2),
            Value::Function(Function::Match { arms: _ }) => write!(f, "<Match Statement>"),
            Value::Function(Function::Builtin(_)) => write!(f, "<Builtin Function>"),
            Value::Tuple(t) => {
                let mut s = String::new();
                for v in t.iter() {
                    s.push_str(&format!("{}, ", v));
                }
                s.pop();
                s.pop();
                write!(f, "({:?})", s)
            },
            Value::List(l) => {
                let mut s = String::new();
                for v in l.iter() {
                    s.push_str(&format!("{}, ", v));
                }
                s.pop();
                s.pop();
                write!(f, "[{}]", s)
            },
            Value::None => write!(f, "None"),
            Value::Filter(_, _) => write!(f, "<Filter>"),
            Value::Map(_, _) => write!(f, "<Map>"),
        }
    }
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::IntRange(_, _) => Type::Iter,
            Value::Function(_) => Type::Match,
            Value::Tuple(vals) => Type::Tuple(vals.iter().map(Value::get_type).collect()),
            Value::List(vals) => Type::List(Box::new(vals.get(0).unwrap_or(&Value::None).get_type())),
            Value::None => Type::None,
            Value::Filter(_, _) => Type::Iter,
            Value::Map(_, _) => Type::Iter,
        }
    }
}