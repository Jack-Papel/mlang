use crate::constructs::token::span::Span;
use crate::constructs::token::symbol::builtin_symbols;
use crate::prelude::*;
use crate::constructs::ast::*;
use crate::constructs::program::UnverifiedProgram;
use crate::constructs::token::{TokenKind, Token, Tokens};
use super::parse_expr::parse_next_expression;

pub fn to_ast(tokens: &mut Tokens) -> Result<UnverifiedProgram> {
    Ok(UnverifiedProgram { 
        block: parse_block(tokens, 0)? 
    })
}

pub fn find_end_of_block(tokens: &mut Tokens, block_indent: usize) -> usize {
    let mut length = 0;
    // An block ends when the next line has a lower indentation than the current line
    loop {
        match tokens.next() {
            Some(Token(TokenKind::Newline(indent), ..)) => {
                if *indent < block_indent {
                    return length;
                }
            },
            None => return length,
            _ => {}
        }
        length += 1;
    }
}

pub fn parse_block(tokens: &mut Tokens, block_indent: usize) -> Result<Block> {
    let mut statements = Vec::new();
    let end = find_end_of_block(&mut tokens.clone(), block_indent);
    let mut tokens = tokens.take(end);

    while let Some(token) = tokens.peek() {
        if let Token(TokenKind::Newline(_), ..) = token {
            tokens.next();
            continue;
        }

        statements.push(parse_next_statement(&mut tokens, block_indent)?);
    }

    if statements.is_empty() {
        if let Some(Token(.., Span { index, len })) = tokens.last() {
            return syntax_err!(Some(Span { 
                index: index + *len as u32, 
                len: 1
            }), "Expected block");
        } else {
            return syntax_err!(None, "Expected block");
        }
    }

    Ok(Block { statements })
}

pub fn parse_next_statement(tokens: &mut Tokens, current_indent: usize) -> Result<Statement> {
    match tokens.peek() {
        Some(Token(TokenKind::Keyword(symbol), ..)) if *symbol == *builtin_symbols::LET => {
            let ident = match tokens.peek_n(1) {
                Some(Token(TokenKind::Identifier(ident), ..)) => ident.get_str().to_string(),
                Some(Token(.., span)) => return syntax_err!(Some(*span), "Expected identifier after let"),
                None => return syntax_err!(None, "Unexpected end of assignment expression")
            };

            tokens.skip(3);
            let expression = parse_next_expression(tokens, current_indent)?;
            Ok(Statement::Let(Identifier { name: ident }, expression))
        },
        Some(Token(TokenKind::Keyword(symbol), ..)) if *symbol == *builtin_symbols::RETURN => {
            tokens.skip(1);
            let expression = parse_next_expression(tokens, current_indent)?;
            Ok(Statement::Return(expression))
        },
        Some(Token(TokenKind::Identifier(ident), ..)) => {
            let ident = ident.get_str().to_string();
            if let Some(Token(TokenKind::Equal, ..)) = tokens.peek_n(1) {
                tokens.skip(2);
                let expression = parse_next_expression(tokens, current_indent)?;
                Ok(Statement::Set(Identifier { name: ident }, expression))
            } else {
                let expression = parse_next_expression(tokens, current_indent)?;
                Ok(Statement::Expression(expression))
            }
        }
        Some(Token(TokenKind::Newline(_), span)) => {
            // This should technically be unreachable
            compiler_err!("Unexpected newline at {:?}", span)
        }
        Some(_) => {
            let expression = parse_next_expression(tokens, current_indent)?;
            Ok(Statement::Expression(expression))
        }
        None => {
            if let Some(Token(.., Span { index, len })) = tokens.last() {
                syntax_err!(Some(Span { 
                    index: index + *len as u32, 
                    len: 1
                }), "Expected statement")
            } else {
                syntax_err!(None, "Expected statement")
            }
        }
    }
}