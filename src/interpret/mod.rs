use crate::prelude::*;
use crate::constructs::variable::Value;
use crate::constructs::ast::*;

pub mod environment;
use environment::Env;

pub trait Executable {
    fn execute(&self, env: &mut Env) -> Result<Value, ExecutionError>;
}

impl Executable for Expression {
    fn execute(&self, env: &mut Env) -> Result<Value, ExecutionError> {
        match self {
            Expression::Literal(literal) => {
                if let Value::Function(Function::Match { arms }) = literal {
                    if arms.iter().all(|arm| {
                        arm.pattern.identifier.is_none()
                    }) {
                        for arm in arms {
                            if matches(&Value::None, arm, env)? {
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
                let left = left.execute(env)?;
                let right = right.execute(env)?;

                match operator {
                    BinaryOperator::Plus => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left + right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left + right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::String(left.to_owned() + right)),
                            _ => exec_err!("Cannot add {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Minus => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left - right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left - right)),
                            _ => exec_err!("Cannot subtract {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Mul => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left * right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left * right)),
                            _ => exec_err!("Cannot multiply {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Div => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left / right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Float(left / right)),
                            _ => exec_err!("Cannot divide {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Range => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::IntRange(*left, *right)),
                            _ => exec_err!("Cannot create range {} and {}", left, right),
                        }
                    }
                    BinaryOperator::ForEach => {
                        if let func @ Value::Function(_) = right {
                            if let Some(mut iter) = left.iter() {
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
                    
                    BinaryOperator::Map => {
                        if let Value::Function(mat) = right {
                            Ok(Value::Map(Box::from(left), mat))
                        } else {
                            exec_err!("Cannot iterate over {} with {}", left, right)
                        }
                    }
                    BinaryOperator::Filter => {
                        if let Value::Function(mat) = right {
                            Ok(Value::Filter(Box::from(left), mat))
                        } else {
                            exec_err!("Cannot filter over {} with {}", left, right)
                        }
                    }
                    BinaryOperator::Mod => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Int(left % right)),
                            _ => exec_err!("Cannot modulo {} and {}", left, right),
                        }
                    }
                    BinaryOperator::NotEqual => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left != right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left != right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left != right)),
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left != right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Equal => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left == right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left == right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left == right)),
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(left == right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Greater => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left > right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left > right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left > right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::GreaterEqual => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left >= right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left >= right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left >= right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Less => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left < right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left < right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left < right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::LessEqual => {
                        match (&left, &right) {
                            (Value::Int(left), Value::Int(right)) => Ok(Value::Boolean(left <= right)),
                            (Value::Float(left), Value::Float(right)) => Ok(Value::Boolean(left <= right)),
                            (Value::String(left), Value::String(right)) => Ok(Value::Boolean(left <= right)),
                            _ => exec_err!("Cannot compare {} and {}", left, right),
                        }
                    }
                    BinaryOperator::And => {
                        match (&left, &right) {
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(*left && *right)),
                            _ => exec_err!("Cannot AND {} and {}", left, right),
                        }
                    }
                    BinaryOperator::Or => {
                        match (&left, &right) {
                            (Value::Boolean(left), Value::Boolean(right)) => Ok(Value::Boolean(*left || *right)),
                            _ => exec_err!("Cannot OR {} and {}", left, right),
                        }
                    }
                    BinaryOperator::All => {
                        if let Some(mut iter) = left.iter() {
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
                    BinaryOperator::Any => {
                        if let Some(mut iter) = left.iter() {
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
                    UnaryOperator::Minus => {
                        match expression {
                            Value::Int(value) => Ok(Value::Int(-value)),
                            Value::Float(value) => Ok(Value::Float(-value)),
                            _ => exec_err!("Cannot negate {}", expression),
                        }
                    }
                    UnaryOperator::Not => {
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

pub fn call_function(value: &Value, function: &Function, env: &mut Env) -> Result<Value, ExecutionError> {
    match function {
        Function::Builtin(b) => b.execute(value.clone(), env),
        Function::Match { arms } => {
            get_result_from_match(value, arms, env)
        }
    }
}

fn get_result_from_match(value: &Value, arms: &Vec<MatchArm>, env: &mut Env) -> Result<Value, ExecutionError> {
    for arm in arms {
        let mut inner_env = env.new_child();

        if matches(value, arm, &mut inner_env)? {
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

fn matches(value: &Value, arm: &MatchArm, inner_env: &mut Env) -> Result<bool, ExecutionError> {
    // Match pattern
    if let Some(ident) = &arm.pattern.identifier {
        if let Some(typ) = &arm.pattern.typ {
            if *typ != value.get_type() {
                return Ok(false);
            }
        }

        inner_env.create_ident(ident.name.clone(), value.clone());
    }

    // Match guard
    if let Some(guard) = &arm.pattern.guard {
        match guard.execute(inner_env)? {
            Value::Boolean(b) => return Ok(b),
            o => return exec_err!("Guard must return a boolean. Got: {}", o),
        }
    }

    Ok(true)
}

impl Executable for Block {
    fn execute(&self, env: &mut Env) -> Result<Value, ExecutionError> {
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
    fn execute(&self, env: &mut Env) -> Result<Value, ExecutionError> {
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