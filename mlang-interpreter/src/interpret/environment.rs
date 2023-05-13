use std::collections::HashMap;

use mlang::constructs::token::symbol::builtin_symbols;
use mlang::constructs::variable::Value; 
use mlang::constructs::ast::Function;

use crate::prelude::*;

pub struct Env<'a> {
    ident_map: HashMap<String, Value>,
    parent: Option<&'a Env<'a>>,
    pub(super) break_flag: bool,
    output: Option<String>
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        let mut ident_map = HashMap::new();
        // Special values
        ident_map.insert("print".to_string(), Value::Function(Function::Builtin(*builtin_symbols::PRINT)));
        ident_map.insert("println".to_string(), Value::Function(Function::Builtin(*builtin_symbols::PRINTLN)));
        ident_map.insert("assert".to_string(), Value::Function(Function::Builtin(*builtin_symbols::ASSERT)));
        Env {
            ident_map,
            parent: None,
            break_flag: false,
            output: Some(String::new())
        }
    }

    pub(super) fn set_break(&mut self) {
        self.break_flag = true;
    }

    pub(super) fn reset_break(&mut self) {
        self.break_flag = false;
    }

    pub(super) fn new_child(&'a self) -> Env<'a> {
        Env {
            ident_map: HashMap::new(),
            parent: Some(self),
            break_flag: false,
            output: None
        }
    }

    pub(super) fn get_ident(&self, name: String) -> Result<&Value> {
        match self.ident_map.get(&name) {
            Some(value) => Ok(value),
            None => match self.parent {
                Some(parent) => parent.get_ident(name),
                None => exec_err!("Identifier {} not found", name)
            }
        }
    }

    pub(super) fn create_ident(&mut self, name: String, value: Value) {
        self.ident_map.insert(name, value);
    }

    pub(super) fn set_ident(&mut self, name: String, value: Value) {
        if let Some(parent) = self.parent {
            if parent.has_ident(&name) {
                // You have my permission to cry about this
                unsafe {
                    let ptr = parent as *const Env as *mut Env;
                    (*ptr).set_ident(name, value);
                }
                return;
            }
        }
        self.ident_map.insert(name, value);
    }

    pub fn print(&mut self, text: String) -> Result<()> {
        if let Some(output) = &mut self.output {
            output.push_str(&text);
        } else if let Some(parent) = self.parent {
            unsafe {
                let ptr = parent as *const Env as *mut Env;
                (*ptr).print(text)?;
            }
        } else {
            return exec_err!("No output stream found");
        }
        Ok(())
    }

    pub fn write_to_string(&self, output: &mut String) {
        if let Some(output_str) = &self.output {
            output.push_str(output_str);
        } else if let Some(parent) = self.parent {
            parent.write_to_string(output);
        }
    }

    pub(super) fn has_ident(&self, name: &str) -> bool {
        self.ident_map.contains_key(name) || match self.parent {
            Some(parent) => parent.has_ident(name),
            None => false,
        }
    }
}