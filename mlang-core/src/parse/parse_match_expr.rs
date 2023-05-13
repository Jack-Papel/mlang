use crate::prelude::*;
use super::parse_expr::{parse_next_expression, parse_single_token};
use super::parse_ast::{parse_block, parse_next_statement, find_end_of_block};
use crate::constructs::token::{TokenKind, Token, Tokens};
use crate::constructs::ast::*;
use crate::constructs::variable::*;

#[allow(unused_variables)]
pub fn parse_match_expression(tokens: &mut Tokens, current_indent: usize) -> Result<Expression, CompilationError> {
    let mut arms = vec![];

    loop {
        let end = find_end_of_block(&mut tokens.clone(), current_indent + 1);
        if end == 0 {
            break;
        }
        let pattern;

        if let Some(Token(TokenKind::Bar(indent), span)) = tokens.next() {
            if *indent != current_indent {
                return syntax_err!(Some(*span), "Expected match arm to be indented");
            }
        } else {
            return syntax_err!(tokens.peek().map(|tok| tok.1), "Tried to parse match arm without bar. Instead, got: {:?}", tokens.peek());
        }

        if let Some((idx, token)) = tokens.clone().enumerate().find(|(_, token)| {
            matches!(token, Token(TokenKind::Colon, ..))
        }) {
            if idx > end {
                return syntax_err!(None, "Expected colon after match pattern");
            }
            pattern = parse_pattern(&mut tokens.take(idx))?;
        } else {
            return syntax_err!(None, "Expected colon after match pattern");
        }

        tokens.next(); // Skip the COLON

        let block = if let Some(Token(TokenKind::Newline(_), ..)) = tokens.peek() {
            tokens.next();
            parse_block(tokens, current_indent + 1)?
        } else {
            Block{
                statements: vec!(parse_next_statement(tokens, current_indent)?)
            }
        };
        tokens.next(); // Skip the NEWLINE

        arms.push(MatchArm {
            pattern,
            block,
        });
    }
    
    Ok(Expression::Literal(Value::Function(Function::Match {
        arms,
    })))
}

fn parse_pattern(tokens: &mut Tokens) -> Result<Pattern, CompilationError> {
    if let Some((idx, _)) = tokens.clone().enumerate().find(|(_, token)| {
        matches!(token, Token(TokenKind::Tilde, ..))
    }) {
        let mut identifier = None;
        let mut typ = None;

        if idx != 0 {
            let pattern = parse_pattern(&mut tokens.take(idx))?;
            identifier = pattern.identifier;
            typ = pattern.typ;
        }

        tokens.next(); // Skip the TILDE

        Ok(Pattern {
            identifier,
            typ,
            guard: Some(parse_next_expression(tokens, 0)?),
        })
    } else {
        // FML!!!
        if let Some(first) = tokens.next() {
            if let Expression::Identifier(id) = parse_single_token(first)? {
                if let Some(second) = tokens.next() {
                    if let Expression::Identifier(ident) = parse_single_token(second)? {
                        Ok(Pattern {
                            identifier: Some(ident),
                            typ: Some(Type::from_id(&id.name).unwrap()),
                            guard: None
                        })
                    } else {
                        syntax_err!(Some(first.1.after()), "Expected identifier after type")
                    }
                } else {
                    Ok(Pattern {
                        identifier: Some(id),
                        typ: None,
                        guard: None
                    })
                }
            } else {
                syntax_err!(Some(first.1), "Expected identifier")
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
