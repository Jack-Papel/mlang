use mlang::constructs::variable::Value;
use mlang::constructs::ast::Function;

use crate::prelude::*;

use super::environment::Env;
use super::call_function;

pub trait MLGIter {
    fn next(&mut self, env: &mut Env) -> Result<Option<Value>>;
}

#[derive(Clone)]
pub struct RangeIter {
    pub current: isize,
    pub end: isize,
}

impl MLGIter for RangeIter {
    fn next(&mut self, _: &mut Env) -> Result<Option<Value>> {
        if self.current < self.end {
            self.current += 1;
            Ok(Some(Value::Int(self.current - 1)))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
pub struct CharIter {
    pub index: usize,
    pub string: String
}

impl MLGIter for CharIter {
    fn next(&mut self, _env: &mut Env) -> Result<Option<Value>> {
        if let Some(char) = self.string.chars().nth(self.index) {
            self.index += 1;
            Ok(Some(Value::String(char.to_string())))
        } else {
            Ok(None)
        }
    }
}

pub struct ListIter<'a> {
    pub(crate) index: usize,
    pub(crate) list: &'a Vec<Value>
}

impl MLGIter for ListIter<'_> {
    fn next(&mut self, _env: &mut Env) -> Result<Option<Value>> {
        if let Some(val) = self.list.get(self.index) {
            self.index += 1;
            Ok(Some(val.clone()))
        } else {
            Ok(None)
        }
    }
}

pub struct FilterIter<'a> {
    pub(crate) iter: Box<dyn MLGIter + 'a>,
    pub(crate) func: Function
}

impl MLGIter for FilterIter<'_> {
    fn next(&mut self, env: &mut Env) -> Result<Option<Value>> {
        while let Some(val) = self.iter.next(env)? {
            if let Value::Boolean(bl) = call_function(&val, &self.func, env)? {
                if bl {
                    return Ok(Some(val));
                } else {
                    continue;
                }
            } else {
                return exec_err!("Filter match must return a boolean");
            }
        }

        Ok(None)
    }

    // fn clone(&self) -> Box<dyn MLGIter> {
    //     Box::from(FilterIter {
    //         iter: self.iter.clone().as_ref(),
    //         mat: self.mat.clone()
    //     })
    // }
}

pub struct MapIter<'a> {
    pub(crate) iter: Box<dyn MLGIter + 'a>,
    pub(crate) func: Function
}

impl MLGIter for MapIter<'_> {
    fn next(&mut self, env: &mut Env) -> Result<Option<Value>> {
        if let Some(val) = self.iter.next(env)? {
            Ok(Some(call_function(&val, &self.func, env)?))
        } else {
            Ok(None)
        }
    }

    // fn clone(&self) -> Box<dyn MLGIter> {
    //     Box::from(MapIter {
    //         iter: self.iter.clone().as_ref(),
    //         mat: self.mat.clone()
    //     })
    // }
}