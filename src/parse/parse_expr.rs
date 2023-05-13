use crate::constructs::token::span::Span;
use crate::prelude::*;
use super::parse_ast::*;
use super::parse_match_expr::parse_match_expression;
use crate::constructs::token::{TokenKind, Token, Tokens};
use crate::constructs::ast::{Expression, BinaryOperator, Identifier};
use crate::constructs::variable::Value;

#[derive(Debug)]
enum ExpressionFragment {
    Parsed(Expression, Span),
    Unparsed(Token),
}

impl ExpressionFragment {
    fn span(&self) -> Span {
        match self {
            ExpressionFragment::Parsed(.., span) |
            ExpressionFragment::Unparsed(Token(.., span)) => *span
        }
    }
}

pub fn parse_next_expression(tokens: &mut Tokens, current_indent: usize) -> Result<Expression, CompilationError> {
    let current_indent = if let Some(Token(TokenKind::Newline(indent), ..)) = tokens.peek() {
        let indent = *indent;
        tokens.next();
        indent
    } else {
        current_indent
    };
    
    let end = find_end_of_expression(&mut tokens.clone(), current_indent);

    let mut atoms: Vec<ExpressionFragment> = tokens.take(end)
        .map(|token| ExpressionFragment::Unparsed(token.clone()))
        .collect();
        
    evaluate_fragments_in_parentheses(&mut atoms, current_indent)?;

    evaluate_fragments_on_match_statements(&mut atoms)?;

    // Remove whitespace - it messes up the parsing
    atoms.retain(|atom| !matches!(atom, ExpressionFragment::Unparsed(Token(TokenKind::Newline(_), ..))));
    // Try to parse any singular parsable tokens.
    // This will make our life easier when doing calls
    for atom in atoms.iter_mut() {
        if let ExpressionFragment::Unparsed(token) = atom {
            if let Ok(expr) = parse_single_token(token) {
                *atom = ExpressionFragment::Parsed(expr, token.1);
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
            ExpressionFragment::Parsed(expr, ..) => Ok(expr.clone()),
            ExpressionFragment::Unparsed(token) => parse_single_token(token),
        }
    } else {
        // Get span
        // This is a big old mess
        if let Some(first_frag) = atoms.get(0) {
            if let Some(second_frag) = atoms.last() {
                syntax_err!(Some(first_frag.span() + second_frag.span()), "Failed to parse into singular expression. Got: {:?}", atoms)
            } else {
                unreachable!("atoms is guaranteed to have at least one element!")
            }
        } else {
            syntax_err!(None, "Failed to parse into singular expression. Got: {:?}", atoms)
        }
    }
}

fn find_end_of_expression(tokens: &mut Tokens, env_indent: usize) -> usize {
    if let Some(Token(TokenKind::Bar(match_indent), ..)) = tokens.peek() {
        let match_indent = *match_indent;
        // We need to find the end of the match statement
        let mut length = 0;
        while let Some(Token(TokenKind::Bar(indent), ..)) = tokens.peek() {
            if *indent != match_indent {
                // We are no longer continuing to parse the match statement
                break;
            }
            let block_length = find_end_of_block(&mut tokens.clone(), match_indent + 1);
            length += block_length;
            tokens.skip(block_length);
            // Skip newline
            tokens.next();
            length += 1;
        }

        // Remove the last newline we accounted for
        length - 1
    } else {
        find_end_of_block(tokens, env_indent + 1)
    }
}

fn evaluate_fragments_on_match_statements(atoms: &mut Vec<ExpressionFragment>) -> Result<(), CompilationError> {
    // First, find if there is a BAR token, and if so, evaluate the rest of the tokens into a match expression
    if let Some((idx, ExpressionFragment::Unparsed(Token(TokenKind::Bar(indent), span)))) = atoms.iter().enumerate().find(|(_, partial)|
        matches!(partial, ExpressionFragment::Unparsed(Token(TokenKind::Bar(_), ..)))
    ) {
        let match_block = parse_match_expression(&mut Tokens::new(
            &atoms.iter().skip(idx).map(|partial| {
                match partial {
                    ExpressionFragment::Unparsed(token) => token.clone(),
                    _ => panic!("Expected unparsed token"),
                }
            }).collect()
        ), *indent)?;

        let span = span.clone();

        atoms.truncate(idx);

        atoms.push(ExpressionFragment::Parsed(match_block, span));
    }

    Ok(())
}

fn evaluate_fragments_in_parentheses(atoms: &mut Vec<ExpressionFragment>, current_indent: usize) -> Result<(), CompilationError> {
    let mut depth = 0;
    while let Some((left_idx, token)) = atoms.iter().enumerate().find(|(_, partial)| {
        match partial {
            ExpressionFragment::Unparsed(Token(TokenKind::LeftParen, ..)) => {
                depth += 1;
                true
            },
            ExpressionFragment::Unparsed(Token(TokenKind::RightParen, ..)) => {
                depth -= 1;
                false
            },
            ExpressionFragment::Unparsed(Token(TokenKind::Bar(_), ..)) => {
                depth == 0
            },
            _ => false,
        }
    }) {
        if let ExpressionFragment::Unparsed(Token(TokenKind::Bar(_), ..)) = token {
            // Don't evaluate parentheses inside match blocks
            break;
        }

        let mut depth = 0;
        if let Some((len_idx, _)) = atoms.iter().skip(left_idx).enumerate().find(|(_, partial)| {
            match partial {
                ExpressionFragment::Unparsed(Token(TokenKind::LeftParen, ..)) => {
                    depth += 1;
                    false
                },
                ExpressionFragment::Unparsed(Token(TokenKind::RightParen, ..)) => {
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

            let mut subqueue = Tokens::new(&subexpression_tokens);
            
            // Get span
            // This is a big old mess
            let span = if let Some(Token(.., first_span)) = subexpression_tokens.get(0) {
                if let Some(Token(.., second_span)) = subexpression_tokens.last() {
                    *first_span + *second_span
                } else {
                    unreachable!("subexpression_tokens is guaranteed to have at least one element!")
                }
            } else {
                return syntax_err!(Some(token.span().after()), "Expected expression inside parentheses");
            };

            let expr = parse_next_expression(&mut subqueue, current_indent)?;

            atoms[left_idx] = ExpressionFragment::Parsed(expr, span);
            atoms.drain((left_idx + 1)..(left_idx + len_idx + 1));

        } else {
            return syntax_err!(Some(token.span()), "Expected closing parenthesis");
        }
    }

    Ok(())
}

fn apply_unary_operator_if_present(atoms: &mut Vec<ExpressionFragment>) -> Result<(), CompilationError> {
    if let Some(ExpressionFragment::Unparsed(token)) = atoms.get(0) {
        if let Ok(op) = token.0.as_unary_operator(Some(token.1)) {
            let first_span = token.1.clone();
            atoms.remove(0);
            if let Some(ExpressionFragment::Parsed(expr, span)) = atoms.get(0) {
                atoms[0] = ExpressionFragment::Parsed(Expression::Unary(
                    op, 
                    Box::new(expr.clone())
                ), first_span + *span);
            } else {
                return syntax_err!(Some(first_span.after()), "Expected parsed expression after unary operator. Got: {:?}", atoms.get(0));
            }
        }
    }

    Ok(())
} 

fn evaluate_fragments_on_binary_operators(atoms: &mut Vec<ExpressionFragment>) -> Result<(), CompilationError> {
    for ops in BinaryOperator::get_precedence_map() {
        while let Some((op_idx, ExpressionFragment::Unparsed(token))) = atoms.iter().enumerate().find(|(_, partial)| {
            if let ExpressionFragment::Unparsed(token) = partial {
                if let Ok(encountered_op) = token.0.as_binary_operator(Some(token.1)) {
                    if ops.iter().any(|op| op.clone() == encountered_op) {
                        return true;
                    }
                }
            }
            false
        }) {
            let (left, right) = get_binary_operands(atoms, op_idx)?;
            let op = token.0.as_binary_operator(Some(token.1))?;

            let expr = Expression::Binary(Box::new(left), op, Box::new(right));

            atoms[op_idx - 1] = ExpressionFragment::Parsed(expr, atoms[op_idx - 1].span() + atoms[op_idx + 1].span());
            atoms.remove(op_idx);
            atoms.remove(op_idx);
        }
    }

    Ok(())
}


fn get_binary_operands(atoms: &[ExpressionFragment], idx: usize) -> Result<(Expression, Expression), CompilationError> {
    let left = match &atoms.get(idx - 1) {
        Some(ExpressionFragment::Parsed(expr, ..)) => expr.clone(),
        Some(ExpressionFragment::Unparsed(token)) => parse_single_token(token)?,
        None => return syntax_err!(Some(atoms[idx].span().after()), "Expected expression before binary operator {:?}", atoms[idx]),
    };

    let right = match &atoms.get(idx + 1) {
        Some(ExpressionFragment::Parsed(expr, ..)) => expr.clone(),
        Some(ExpressionFragment::Unparsed(token)) => parse_single_token(token)?,
        None => return syntax_err!(Some(atoms[idx].span().after()), "Expected expression after binary operator {:?}", atoms[idx]),
    };

    Ok((left, right))
}

#[allow(unused_variables)]
fn evaluate_fragments_on_calls(atoms: &mut Vec<ExpressionFragment>) -> Result<(), CompilationError> {
    while let Some((idx, [
            ExpressionFragment::Parsed(expr1, span1), 
            ExpressionFragment::Parsed(expr2, span2)
        ])) = atoms.windows(2).enumerate().find(|(_, frags)| {
        matches!((&frags[0], &frags[1]), (ExpressionFragment::Parsed(..), ExpressionFragment::Parsed(..)))
    }) {
        let call = Expression::Call(Box::from(expr1.clone()), Box::from(expr2.clone()));
        atoms[idx] = ExpressionFragment::Parsed(call, *span1 + *span2);
        atoms.remove(idx + 1);
    }

    Ok(())
}

pub fn parse_single_token(token: &Token) -> Result<Expression, CompilationError> {
    match token.0 {
        TokenKind::Identifier(ident) => Ok(Expression::Identifier(Identifier { name: ident.get_str().to_string() })),
        TokenKind::Literal(lit) => Ok(Expression::Literal(Value::try_from((lit, token.1))?)),
        _ => syntax_err!(Some(token.1), "Unexpected token"),
    }
}
