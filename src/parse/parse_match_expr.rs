use crate::prelude::*;
use super::token_queue::TokenQueue;
use super::parse_expr::{parse_next_expression, parse_single_token};
use super::parse_ast::{parse_block, parse_next_statement, find_end_of_block};
use crate::constructs::token::{TokenKind, Token};
use crate::constructs::ast::*;
use crate::constructs::variable::*;

#[allow(unused_variables)]
pub fn parse_match_expression(token_queue: &mut TokenQueue, current_indent: usize) -> Result<Expression> {
    let mut arms = vec![];

    loop {
        let end = find_end_of_block(&mut token_queue.clone(), current_indent + 1);
        if end == 0 {
            break;
        }
        let pattern;

        if let Some(Token(TokenKind::Bar(indent), span)) = token_queue.next() {
            if *indent != current_indent {
                return parse_err!(Some(*span), "Expected match arm to be indented");
            }
        } else {
            return parse_err!(token_queue.peek().map(|tok| tok.1), "Tried to parse match arm without bar. Instead, got: {:?}", token_queue.peek());
        }

        if let Some((idx, token)) = token_queue.clone().enumerate().find(|(_, token)| {
            matches!(token, Token(TokenKind::Colon, ..))
        }) {
            if idx > end {
                return parse_err!(None, "Expected colon after match pattern");
            }
            pattern = parse_pattern(&mut token_queue.take(idx))?;
        } else {
            return parse_err!(None, "Expected colon after match pattern");
        }

        token_queue.next(); // Skip the COLON

        let block = if let Some(Token(TokenKind::Newline(_), ..)) = token_queue.peek() {
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
        matches!(token, Token(TokenKind::Tilde, ..))
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
                        parse_err!(Some(first.1.after()), "Expected identifier after type")
                    }
                } else {
                    Ok(Pattern {
                        identifier: Some(id),
                        typ: None,
                        guard: None
                    })
                }
            } else {
                parse_err!(Some(first.1), "Expected identifier")
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
