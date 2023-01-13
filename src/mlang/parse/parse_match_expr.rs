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
            matches!(token, Token::COLON)
        }) {
            if idx > end {
                return parse_err!("Expected colon after match pattern");
            }
            pattern = parse_pattern(&mut token_queue.take(idx))?;
        } else {
            return parse_err!("Expected colon after match pattern");
        }

        token_queue.next(); // Skip the COLON

        let block = if let Some(Token::NEWLINE(_)) = token_queue.peek() {
            token_queue.next();
            parse_block(token_queue, current_indent + 1)?
        } else {
            Block{
                statements: vec!(parse_next_statement(token_queue, current_indent)?)
            }
        };
        token_queue.next(); // Skip the NEWLINE

        arms.push(MatchArm {
            pattern,
            block,
        });
    }
    
    Ok(Expression::Literal(Value::Function(Function::Match {
        arms,
    })))
}

fn parse_pattern(token_queue: &mut TokenQueue) -> Result<Pattern> {
    if let Some((idx, _)) = token_queue.clone().enumerate().find(|(_, token)| {
        matches!(token, Token::TILDE)
    }) {
        let mut identifier = None;
        let mut typ = None;

        if idx != 0 {
            let pattern = parse_pattern(&mut token_queue.take(idx))?;
            identifier = pattern.identifier;
            typ = pattern.typ;
        }

        token_queue.next(); // Skip the TILDE

        Ok(Pattern {
            identifier,
            typ,
            guard: Some(parse_next_expression(token_queue, 0)?),
        })
    } else {
        // FML!!!
        if let Some(first) = token_queue.next() {
            if let Expression::Identifier(id) = parse_single_token(first)? {
                if let Some(second) = token_queue.next() {
                    if let Expression::Identifier(ident) = parse_single_token(second)? {
                        Ok(Pattern {
                            identifier: Some(ident),
                            typ: Some(Type::from_id(&id.name).unwrap()),
                            guard: None
                        })
                    } else {
                        parse_err!("Expected identifier after type")
                    }
                } else {
                    Ok(Pattern {
                        identifier: Some(id),
                        typ: None,
                        guard: None
                    })
                }
            } else {
                parse_err!("Expected identifier")
            }
        } else {
            Ok(
                Pattern {
                    identifier: None,
                    typ: None,
                    guard: None
                }
            )
        }
    }
}
