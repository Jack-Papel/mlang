use std::collections::HashMap;

use crate::mlang::variable::{Value, Builtin};
use crate::mlang::ast::*;
use crate::prelude::*;

pub struct Env<'a> {
    ident_map: HashMap<String, Value>,
    parent: Option<&'a Env<'a>>,
    break_flag: bool,
    output: Option<String>
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        let mut ident_map = HashMap::new();
        // Special values
        ident_map.insert("print".to_string(), Value::Function(Function::Builtin(Builtin::Print)));
        ident_map.insert("assert".to_string(), Value::Function(Function::Builtin(Builtin::Assert)));
        Env {
            ident_map,
            parent: None,
            break_flag: false,
            output: Some(String::new())
        }
    }

    fn set_break(&mut self) {
        self.break_flag = true;
    }

    fn reset_break(&mut self) {
        self.break_flag = false;
    }

    fn new_child(&'a self) -> Env<'a> {
        Env {
            ident_map: HashMap::new(),
            parent: Some(self),
            break_flag: false,
            output: None
        }
    }

    fn get_ident(&self, name: String) -> Result<&Value> {
        match self.ident_map.get(&name) {
            Some(value) => Ok(value),
            None => match self.parent {
                Some(parent) => parent.get_ident(name),
                None => exec_err!("Identifier {} not found", name)
            }
        }
    }

    fn set_ident(&mut self, name: String, value: Value) {
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
        } else {
            if let Some(parent) = self.parent {
                unsafe {
                    let ptr = parent as *const Env as *mut Env;
                    (*ptr).print(text)?;
                }
            } else {
                return exec_err!("No output stream found");
            }
        }
        Ok(())
    }

    pub fn write_to_string(&self, output: &mut String) {
        if let Some(output_str) = &self.output {
            output.push_str(output_str);
        } else {
            if let Some(parent) = self.parent {
                parent.write_to_string(output);
            }
        }
    }

    fn has_ident(&self, name: &str) -> bool {
        self.ident_map.contains_key(name) || match self.parent {
            Some(parent) => parent.has_ident(name),
            None => false,
        }
    }
}

pub trait Executable {
    fn execute(&self, env: &mut Env) -> Result<Value>;
}

impl Executable for Expression {
    fn execute(&self, env: &mut Env) -> Result<Value> {
        match self {
            Expression::Literal(literal) => {
                if let Value::Function(Function::Match { arms }) = literal {
                    if arms.iter().all(|arm| {
                        arm.pattern.identifier.is_none()
                    }) {
                        for arm in arms {
                            if matches(&Value::None, &arm, env)? {
                                return arm.block.execute(env);
                            }
                        }
        
                        return Ok(Value::None);
                    }
                }
                Ok(literal.clone())
            },
            Expression::Identifier(identifier) => {
                env.get_ident(identifier.name.clone()).map(|v| v.clone())
            }
            Expression::Binary(left, operator, right) => {
                let left = left.execute(env)?.clone();
                let right = right.execute(env)?.clone();

                match operator {
                    BinaryOperator::PLUS => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::String(left.to_owned() + right)),
                            _ => exec_err!("Cannot add {} and {}", left, right),
                        }
                    }
                    BinaryOperator::MINUS => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
                            _ => exec_err!("Cannot subtract {} and {}", left, right),
                        }
                    }
                    BinaryOperator::MUL => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
                            _ => exec_err!("Cannot multiply {} and {}", left, right),
                        }
                    }
                    BinaryOperator::DIV => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left / right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
                            _ => exec_err!("Cannot divide {} and {}", left, right),
                        }
                    }
                    BinaryOperator::RANGE => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::IntRange(*left, *right)),
                            _ => exec_err!("Cannot create range {} and {}", left, right),
                        }
                    }
                    BinaryOperator::FOR_EACH => {
                        if let func @ Value::Function(_) = right {
                            if let Some(mut iter) = left.iter()? {
                                while let Some(val) = iter.next(env)? {
                                    Expression::Call(
                                        Box::from(Expression::Literal(val)), 
                                        Box::from(Expression::Literal(func.clone()))
                                    ).execute(env)?;

                                    if env.break_flag {
                                        env.reset_break();
                                        break;
                                    }
                                }
                            }

                            Ok(Value::None)
                        } else {
                            exec_err!("Cannot iterate over {} with {}", left, right)
                        }
                    }
                    
                    BinaryOperator::MAP => {
                        if let Value::Function(mat) = right {
                            Ok(Value::Map(Box::from(left), mat))
                        } else {
                            exec_err!("Cannot iterate over {} with {}", left, right)
                        }
                    }
                    BinaryOperator::FILTER => {
                        if let Value::Function(mat) = right {
                            Ok(Value::Filter(Box::from(left), mat))
                        } else {
                            exec_err!("Cannot filter over {} with {}", left, right)
                        }
                    }
                    BinaryOperator::MOD => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left % right)),
                            _ => exec_err!("Cannot modulo {} and {}", left, right),
                        }
                    }
                    BinaryOperator::NOT_EQUAL => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left != right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left != right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left != right)),
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left != right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::EQUAL => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left == right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left == right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left == right)),
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left == right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::GREATER => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left > right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left > right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left > right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::GREATER_EQUAL => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left >= right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left >= right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left >= right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::LESS => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left < right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left < right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left < right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::LESS_EQUAL => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left <= right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left <= right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left <= right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::AND => {
                        match (&left, &right) {
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(*left && *right)),
                            _ => exec_err!("Cannot AND {} and {}", left, right),
                        }
                    }
                    BinaryOperator::OR => {
                        match (&left, &right) {
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(*left || *right)),
                            _ => exec_err!("Cannot OR {} and {}", left, right),
                        }
                    }
                    BinaryOperator::ALL => {
                        if let Some(mut iter) = left.iter()? {
                            while let Some(val) = iter.next(env)? {
                                if let Value::Boolean(bl) = Expression::Call(
                                    Box::from(Expression::Literal(val.clone())), 
                                    Box::from(Expression::Literal(right.clone()))
                                ).execute(env)? {
                                    if !bl {
                                        return Ok(Value::Boolean(false));
                                    }
                                } else {
                                    return exec_err!("Filter must return a boolean!");
                                }
                            }
                            Ok(Value::Boolean(true))
                        } else {
                            exec_err!("Cannot &&& over {} with {}", left, right)
                        }
                    }
                    BinaryOperator::ANY => {
                        if let Some(mut iter) = left.iter()? {
                            while let Some(val) = iter.next(env)? {
                                if let Value::Boolean(bl) = Expression::Call(
                                    Box::from(Expression::Literal(val.clone())), 
                                    Box::from(Expression::Literal(right.clone()))
                                ).execute(env)? {
                                    if bl {
                                        return Ok(Value::Boolean(true));
                                    }
                                } else {
                                    return exec_err!("Filter must return a boolean!");
                                }
                            }
                            Ok(Value::Boolean(false))
                        } else {
                            exec_err!("Cannot ||| over {} with {}", left, right)
                        }
                    }
                }
            }
            Expression::Unary(operator, expression) => {
                let expression = expression.execute(env)?;

                match operator {
                    UnaryOperator::MINUS => {
                        match expression {
                            Value::Int(value) => Ok(Value::Int(-value)),
                            Value::Float(value) => Ok(Value::Float(-value)),
                            _ => exec_err!("Cannot negate {}", expression),
                        }
                    }
                    UnaryOperator::NOT => {
                        match expression {
                            Value::Boolean(value) => Ok(Value::Boolean(!value)),
                            _ => exec_err!("Cannot negate {}", expression),
                        }
                    }
                }
            }
            Expression::Call(expression, callee) => {
                match callee.execute(env)? {
                    Value::Function(func) => call_function(&expression.execute(env)?, &func, env),
                    unknown => {
                        exec_err!("Cannot call {:?}", unknown)
                    }
                }
            }
        }
    }
}

pub fn call_function(value: &Value, function: &Function, env: &mut Env) -> Result<Value> {
    match function {
        Function::Builtin(b) => b.execute(value.clone(), env),
        Function::Match { arms } => {
            get_result_from_match(value, arms, env)
        }
    }
}

fn get_result_from_match(value: &Value, arms: &Vec<MatchArm>, env: &mut Env) -> Result<Value> {
    for arm in arms {
        let mut inner_env = env.new_child();

        if matches(value, &arm, &mut inner_env)? {
            let result = arm.block.execute(&mut inner_env);
            // This is kinda hacky, but it works. Probably.
            if inner_env.break_flag {
                env.break_flag = true;
            }
            return result;
        }
    }

    Ok(Value::None)
}

fn matches(value: &Value, arm: &MatchArm, inner_env: &mut Env) -> Result<bool> {
    // Match pattern
    if let Some(ref ident) = arm.pattern.identifier {
        if let Some(ref typ) = arm.pattern.typ {
            if *typ != value.get_type() {
                return Ok(false);
            }
        }

        inner_env.ident_map.insert(ident.name.clone(), value.clone());
    }

    // Match guard
    if let Some(ref guard) = arm.pattern.guard {
        match guard.execute(inner_env)? {
            Value::Boolean(b) => return Ok(b),
            o => return exec_err!("Guard must return a boolean. Got: {}", o),
        }
    }

    return Ok(true);
}

impl Executable for Block {
    fn execute(&self, env: &mut Env) -> Result<Value> {
        for (idx, statement) in self.statements.iter().enumerate() {
            if idx == self.statements.len() - 1 {
                return statement.execute(env);
            }

            match statement {
                Statement::Break(expr) => {
                    env.set_break();
                    if let Some(result) = expr {
                        return result.execute(env);
                    } else {
                        return Ok(Value::None);
                    }
                }
                Statement::Continue => {
                    return Ok(Value::None);
                }
                Statement::Return(_) => {
                    return statement.execute(env);
                }
                statement => {
                    statement.execute(env)?;
                }
            }
        }

        unreachable!()
    }
}

impl Executable for Statement {
    fn execute(&self, env: &mut Env) -> Result<Value> {
        match self {
            Statement::Expression(expression) => expression.execute(env),
            Statement::Let(identifier, expression) => {
                if env.has_ident(&identifier.name) {
                    exec_err!("Identifier {} already exists", identifier.name)
                } else {
                    let value = expression.execute(env)?;
                    
                    env.set_ident(identifier.name.clone(), value);
                    Ok(Value::None)
                }
            }
            Statement::Return(expression) => expression.execute(env),
            Statement::Set(identifier, expression) => {
                if !env.has_ident(&identifier.name) {
                    exec_err!("Identifier {} does not exist", identifier.name)
                } else {
                    let value = expression.execute(env)?;
                    
                    env.set_ident(identifier.name.clone(), value);
                    Ok(Value::None)
                }
            }
            _ => unreachable!("Breaks and continues should be handled in the block execute function"),
        }
    }
}