use crate::prelude::*;
use super::parse_ast::*;
use super::token_queue::TokenQueue;
use super::parse_match_expr::parse_match_expression;
use crate::constructs::token::Token;
use crate::constructs::ast::{Expression, BinaryOperator, Identifier};
use crate::constructs::variable::{Value};

#[derive(Debug)]
enum ExpressionFragment {
    Parsed(Expression),
    Unparsed(Token),
}

pub fn parse_next_expression(token_queue: &mut TokenQueue, current_indent: usize) -> Result<Expression> {
    let current_indent = if let Some(Token::NEWLINE(indent)) = token_queue.peek() {
        let indent = *indent;
        token_queue.next();
        indent
    } else {
        current_indent
    };
    
    let end = find_end_of_expression(&mut token_queue.clone(), current_indent);

    let mut atoms: Vec<ExpressionFragment> = token_queue.take(end)
        .map(|token| ExpressionFragment::Unparsed(token.clone()))
        .collect();
        
    evaluate_fragments_in_parentheses(&mut atoms, current_indent)?;

    evaluate_fragments_on_match_statements(&mut atoms)?;

    // Remove whitespace - it messes up the parsing
    atoms.retain(|atom| !matches!(atom, ExpressionFragment::Unparsed(Token::NEWLINE(_))));
    // Try to parse any singular parsable tokens.
    // This will make our life easier when doing calls
    for atom in atoms.iter_mut() {
        if let ExpressionFragment::Unparsed(token) = atom {
            if let Ok(expr) = parse_single_token(token) {
                *atom = ExpressionFragment::Parsed(expr);
            }
        }
    }

    apply_unary_operator_if_present(&mut atoms)?;

    evaluate_fragments_on_calls(&mut atoms)?;

    evaluate_fragments_on_binary_operators(&mut atoms)?;

    if atoms.is_empty() {
        // () = none. This will make sense once tuples are added
        Ok(Expression::Literal(Value::None))
    } else if atoms.len() == 1 {
        match &atoms[0] {
            ExpressionFragment::Parsed(expr) => Ok(expr.clone()),
            ExpressionFragment::Unparsed(token) => parse_single_token(token),
        }
    } else {
        parse_err!("Failed to parse into singular expression. Got: {:?}", atoms)
    }
}

fn find_end_of_expression(token_queue: &mut TokenQueue, env_indent: usize) -> usize {
    if let Some(Token::BAR(match_indent)) = token_queue.peek() {
        let match_indent = *match_indent;
        // We need to find the end of the match statement
        let mut length = 0;
        while let Some(Token::BAR(indent)) = token_queue.peek() {
            if *indent != match_indent {
                // We are no longer continuing to parse the match statement
                break;
            }
            let block_length = find_end_of_block(&mut token_queue.clone(), match_indent + 1);
            length += block_length;
            token_queue.skip(block_length);
            // Skip newline
            token_queue.next();
            length += 1;
        }

        // Remove the last newline we accounted for
        length - 1
    } else {
        find_end_of_block(token_queue, env_indent + 1)
    }
}

fn evaluate_fragments_on_match_statements(atoms: &mut Vec<ExpressionFragment>) -> Result<()> {
    // First, find if there is a BAR token, and if so, evaluate the rest of the tokens into a match expression
    if let Some((idx, ExpressionFragment::Unparsed(Token::BAR(indent)))) = atoms.iter().enumerate().find(|(_, partial)|
        matches!(partial, ExpressionFragment::Unparsed(Token::BAR(_)))
    ) {
        let match_block = parse_match_expression(&mut TokenQueue::new( 
            &atoms.iter().skip(idx).map(|partial| {
                match partial {
                    ExpressionFragment::Unparsed(token) => token.clone(),
                    _ => panic!("Expected unparsed token"),
                }
            }).collect()
        ), *indent)?;

        atoms.truncate(idx);

        atoms.push(ExpressionFragment::Parsed(match_block));
    }

    Ok(())
}

fn evaluate_fragments_in_parentheses(atoms: &mut Vec<ExpressionFragment>, current_indent: usize) -> Result<()> {
    let mut depth = 0;
    while let Some((left_idx, token)) = atoms.iter().enumerate().find(|(_, partial)| {
        match partial {
            ExpressionFragment::Unparsed(Token::LEFT_PAREN) => {
                depth += 1;
                true
            },
            ExpressionFragment::Unparsed(Token::RIGHT_PAREN) => {
                depth -= 1;
                false
            },
            ExpressionFragment::Unparsed(Token::BAR(_)) => {
                depth == 0
            },
            _ => false,
        }
    }) {
        if let ExpressionFragment::Unparsed(Token::BAR(_)) = token {
            // Don't evaluate parentheses inside match blocks
            break;
        }

        let mut depth = 0;
        if let Some((len_idx, _)) = atoms.iter().skip(left_idx).enumerate().find(|(_, partial)| {
            match partial {
                ExpressionFragment::Unparsed(Token::LEFT_PAREN) => {
                    depth += 1;
                    false
                },
                ExpressionFragment::Unparsed(Token::RIGHT_PAREN) => {
                    depth -= 1;
                    depth == 0
                },
                _ => false,
            }
        }) {
            let subexpression_tokens = atoms.iter().skip(left_idx + 1).take(len_idx - 1).map(|partial| {
                match partial {
                    ExpressionFragment::Unparsed(token) => token.clone(),
                    _ => panic!("Expected unparsed token"),
                }
            }).collect();

            let mut subqueue = TokenQueue::new(&subexpression_tokens);

            let expr = parse_next_expression(&mut subqueue, current_indent)?;

            atoms[left_idx] = ExpressionFragment::Parsed(expr);
            atoms.drain((left_idx + 1)..(left_idx + len_idx + 1));

        } else {
            return parse_err!("Expected closing parenthesis");
        }
    }

    Ok(())
}

fn apply_unary_operator_if_present(atoms: &mut Vec<ExpressionFragment>) -> Result<()> {
    if let Some(ExpressionFragment::Unparsed(token)) = atoms.get(0) {
        if let Ok(op) = token.as_unary_operator() {
            atoms.remove(0);
            if let Some(ExpressionFragment::Parsed(expr)) = atoms.get(0) {
                atoms[0] = ExpressionFragment::Parsed(Expression::Unary(
                    op, 
                    Box::new(expr.clone())
                ));
            } else {
                return parse_err!("Expected parsed expression after unary operator. Got: {:?}", atoms.get(0));
            }
        }
    }

    Ok(())
} 

fn evaluate_fragments_on_binary_operators(atoms: &mut Vec<ExpressionFragment>) -> Result<()> {
    for ops in BinaryOperator::get_precedence_map() {
        while let Some((op_idx, ExpressionFragment::Unparsed(token))) = atoms.iter().enumerate().find(|(_, partial)| {
            if let ExpressionFragment::Unparsed(token) = partial {
                if let Ok(encountered_op) = token.as_binary_operator() {
                    if ops.iter().any(|op| op.clone() == encountered_op) {
                        return true;
                    }
                }
            }
            false
        }) {
            let (left, right) = get_binary_operands(atoms, op_idx)?;
            let op = token.as_binary_operator()?;

            let expr = Expression::Binary(Box::new(left), op, Box::new(right));

            atoms[op_idx - 1] = ExpressionFragment::Parsed(expr);
            atoms.remove(op_idx);
            atoms.remove(op_idx);
        }
    }

    Ok(())
}


fn get_binary_operands(atoms: &[ExpressionFragment], idx: usize) -> Result<(Expression, Expression)> {
    let left = match &atoms.get(idx - 1) {
        Some(ExpressionFragment::Parsed(expr)) => expr.clone(),
        Some(ExpressionFragment::Unparsed(token)) => parse_single_token(token)?,
        None => return parse_err!("Expected expression before binary operator {:?}", atoms[idx]),
    };

    let right = match &atoms.get(idx + 1) {
        Some(ExpressionFragment::Parsed(expr)) => expr.clone(),
        Some(ExpressionFragment::Unparsed(token)) => parse_single_token(token)?,
        None => return parse_err!("Expected expression after binary operator {:?}", atoms[idx]),
    };

    Ok((left, right))
}

#[allow(unused_variables)]
fn evaluate_fragments_on_calls(atoms: &mut Vec<ExpressionFragment>) -> Result<()> {
    while let Some((idx, [
                ExpressionFragment::Parsed(expr1), 
                ExpressionFragment::Parsed(expr2)
        ])) = atoms.windows(2).enumerate().find(|(_, frags)| {
        matches!((&frags[0], &frags[1]), (ExpressionFragment::Parsed(_), ExpressionFragment::Parsed(_)))
    }) {
        let call = Expression::Call(Box::from(expr1.clone()), Box::from(expr2.clone()));
        atoms[idx] = ExpressionFragment::Parsed(call);
        atoms.remove(idx + 1);
    }

    Ok(())
}

pub fn parse_single_token(token: &Token) -> Result<Expression> {
    match token {
        Token::IDENTIFIER(ident) => Ok(Expression::Identifier(Identifier { name: ident.to_string() })),
        Token::STRING(str) => Ok(Expression::Literal(Value::String(str.to_string()))),
        Token::INT(num) => Ok(Expression::Literal(Value::Int(*num))),
        Token::FLOAT(num) => Ok(Expression::Literal(Value::Float(*num))),
        Token::TRUE => Ok(Expression::Literal(Value::Boolean(true))),
        Token::FALSE => Ok(Expression::Literal(Value::Boolean(false))),
        _ => parse_err!("Unexpected token: {:?}", token),
    }
}
