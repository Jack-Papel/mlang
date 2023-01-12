use super::util::TokenQueue;
use crate::prelude::*;
use crate::mlang::ast::*;
use super::tokens::Token;
use super::parse_expr::*;
use super::parse_ast::{parse_block, parse_next_statement, find_end_of_block};
use crate::mlang::variable::*;

#[allow(unused_variables)]
pub fn parse_match_expression(token_queue: &mut TokenQueue, current_indent: usize) -> Result<Expression> {
    let mut arms = vec![];

    loop {
        let end = find_end_of_block(&mut token_queue.clone(), current_indent + 1);
        if end == 0 {
            break;
        }
        let pattern;

        token_queue.next(); // Skip the BAR
        if let Some((idx, token)) = token_queue.clone().enumerate().find(|(_, token)| {
            match token {
                Token::COLON => true,
                _ => false,
            }
        }) {
            if idx > end {
                return parse_err!("Expected colon after match pattern");
            }
            pattern = parse_pattern(&mut token_queue.take(idx))?;
        } else {
            return parse_err!("Expected colon after match pattern");
        }

        token_queue.next(); // Skip the COLON

        let body = if let Some(Token::NEWLINE(_)) = token_queue.peek() {
            token_queue.next();
            let result = parse_block(token_queue, current_indent + 1)?;
            result
        } else {
            Block{
                statements: vec!(parse_next_statement(token_queue, current_indent)?)
            }
        };
        token_queue.next(); // Skip the NEWLINE

        arms.push(MatchArm {
            pattern: pattern,
            block: body,
        });
    }
    
    Ok(Expression::Literal(Value::Function(Function::Match {
        arms: arms,
    })))
}

fn parse_pattern(token_queue: &mut TokenQueue) -> Result<Pattern> {
    if let Some((idx, _)) = token_queue.clone().enumerate().find(|(_, token)| {
        match token {
            Token::TILDE => true,
            _ => false,
        }
    }) {
        let mut ident = None;
        let mut typ = None;

        if idx != 0 {
            let pattern = parse_pattern(&mut token_queue.take(idx))?;
            ident = pattern.identifier;
            typ = pattern.typ;
        }

        token_queue.next(); // Skip the TILDE

        Ok(Pattern {
            identifier: ident,
            typ: typ,
            guard: Some(parse_next_expression(token_queue, 0)?),
        })
    } else {
        // FML!!!
        if let Some(first) = token_queue.next() {
            if let Expression::Identifier(id) = parse_single_token(first)? {
                if let Some(second) = token_queue.next() {
                    if let Expression::Identifier(ident) = parse_single_token(second)? {
                        return Ok(Pattern {
                            identifier: Some(ident),
                            typ: Some(Type::from_id(&id.name).unwrap()),
                            guard: None
                        });
                    } else {
                        return parse_err!("Expected identifier after type");
                    }
                } else {
                    return Ok(Pattern {
                        identifier: Some(id),
                        typ: None,
                        guard: None
                    });
                }
            } else {
                return parse_err!("Expected identifier");
            }
        } else {
            return Ok(
                Pattern {
                    identifier: None,
                    typ: None,
                    guard: None
                }
            );
        }
    }
}
