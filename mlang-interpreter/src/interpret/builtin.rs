use mlang::constructs::token::symbol::{builtin_symbols, Symbol};
use mlang::constructs::variable::Value;

use super::environment::Env;
use super::iter::{MLGIter, CharIter, RangeIter, ListIter, FilterIter, MapIter};

use crate::prelude::*;

/// Builtin functions
#[derive(Debug, Clone)]
pub enum Builtin {
    Print,
    Println,
    Assert
}

impl Builtin {
    pub fn from(symbol: &Symbol) -> Result<Builtin> {
        if *symbol == *builtin_symbols::PRINT {
            Ok(Builtin::Print)
        } else if *symbol == *builtin_symbols::PRINTLN {
            Ok(Builtin::Println)
        } else if *symbol == *builtin_symbols::ASSERT {
            Ok(Builtin::Assert)
        } else {
            exec_err!("Symbol {} is not a builtin function!", symbol)
        }
    }

    pub fn execute(&self, value: Value, env: &mut Env) -> Result<Value> {
        match self {
            Self::Print => {
                env.print(format!("{}", value))?;
                Ok(Value::None)
            }
            Self::Println => {
                env.print(format!("{}\n", value))?;
                Ok(Value::None)
            }
            Self::Assert => {
                match value {
                    Value::Boolean(b) if b => {}
                    _ => {
                        return Err(ExecutionError("Assertion failed!".to_string()))
                    }
                }
                Ok(Value::None)
            }
        }
    }
}

pub trait Iterable {
    fn iter<'a>(&'a self) -> Option<Box<dyn MLGIter + 'a>>;
}

impl Iterable for Value {
    fn iter<'a>(&'a self) -> Option<Box<dyn MLGIter + 'a>> {
        match self {
            Value::String(s) => Some(Box::from(CharIter {
                index: 0,
                string: s.clone()
            })),
            Value::IntRange(b, e) => Some(Box::from(RangeIter {
                current: *b,
                end: *e
            })),
            Value::List(l) => Some(Box::from(ListIter {
                index: 0,
                list: l
            })),
            Value::Filter(val, mat) => {
                val.iter().map(|iter| {
                    Box::from(FilterIter {
                        iter,
                        func: mat.clone()
                    }) as Box<dyn MLGIter>
                })
            }
            Value::Map(val, mat) => {
                val.iter().map(|iter| {
                    Box::from(MapIter {
                        iter,
                        func: mat.clone()
                    }) as Box<dyn MLGIter>
                })
            }
            _ => None,
        }
    }
}
