use crate::constructs::token::symbol::builtin_symbols;
use crate::prelude::*;
use super::token_queue::TokenQueue;
use crate::constructs::ast::*;
use crate::constructs::program::Program;
use crate::constructs::token::TokenKind;
use super::parse_expr::parse_next_expression;

pub fn to_ast(tokens: Vec<TokenKind>) -> Result<Program> {
    let mut token_queue = TokenQueue::new(&tokens);

    Ok(Program { 
        block: parse_block(&mut token_queue, 0)? 
    })
}

pub fn find_end_of_block(token_queue: &mut TokenQueue, block_indent: usize) -> usize {
    let mut length = 0;
    // An block ends when the next line has a lower indentation than the current line
    loop {
        match token_queue.next() {
            Some(TokenKind::Newline(indent)) => {
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

pub fn parse_block(token_queue: &mut TokenQueue, block_indent: usize) -> Result<Block> {
    let mut statements = Vec::new();
    let end = find_end_of_block(&mut token_queue.clone(), block_indent);
    let mut token_queue = token_queue.take(end);

    while let Some(token) = token_queue.peek() {
        if let TokenKind::Newline(_) = token {
            token_queue.next();
            continue;
        }

        statements.push(parse_next_statement(&mut token_queue, block_indent)?);
    }

    if statements.is_empty() {
        return parse_err!("Expected block");
    }

    Ok(Block { statements })
}

pub fn parse_next_statement(token_queue: &mut TokenQueue, current_indent: usize) -> Result<Statement> {
    match token_queue.peek() {
        Some(TokenKind::Keyword(symbol)) if *symbol == *builtin_symbols::LET => {
            let ident = match token_queue.peek_n(1) {
                Some(TokenKind::Identifier(ident)) => ident.get_str().to_string(),
                _ => return parse_err!("Expected identifier after let"),
            };

            token_queue.skip(3);
            let expression = parse_next_expression(token_queue, current_indent)?;
            Ok(Statement::Let(Identifier { name: ident }, expression))
        },
        Some(TokenKind::Keyword(symbol)) if *symbol == *builtin_symbols::RETURN => {
            token_queue.skip(1);
            let expression = parse_next_expression(token_queue, current_indent)?;
            Ok(Statement::Return(expression))
        },
        Some(TokenKind::Identifier(ident)) => {
            let ident = ident.get_str().to_string();
            if let Some(TokenKind::Equal) = token_queue.peek_n(1) {
                token_queue.skip(2);
                let expression = parse_next_expression(token_queue, current_indent)?;
                Ok(Statement::Set(Identifier { name: ident }, expression))
            } else {
                let expression = parse_next_expression(token_queue, current_indent)?;
                Ok(Statement::Expression(expression))
            }
        }
        Some(TokenKind::Newline(_)) => {
            unreachable!("Should be handled by parse_block()");
        }
        Some(_) => {
            let expression = parse_next_expression(token_queue, current_indent)?;
            Ok(Statement::Expression(expression))
        }
        None => parse_err!("Expected statement"),
    }
}