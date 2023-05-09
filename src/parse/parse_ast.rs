use crate::prelude::*;
use super::token_queue::TokenQueue;
use crate::constructs::ast::*;
use crate::constructs::program::Program;
use crate::constructs::token::Token;
use super::parse_expr::parse_next_expression;

pub fn to_ast(tokens: Vec<Token>) -> Result<Program> {
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
            Some(Token::NEWLINE(indent)) => {
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
        if let Token::NEWLINE(_) = token {
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
        Some(Token::LET) => {
            let ident = match token_queue.peek_n(1) {
                Some(Token::IDENTIFIER(ident)) => ident.to_string(),
                _ => return parse_err!("Expected identifier after let"),
            };

            token_queue.skip(3);
            let expression = parse_next_expression(token_queue, current_indent)?;
            Ok(Statement::Let(Identifier { name: ident }, expression))
        },
        Some(Token::RETURN) => {
            token_queue.skip(1);
            let expression = parse_next_expression(token_queue, current_indent)?;
            Ok(Statement::Return(expression))
        },
        Some(Token::IDENTIFIER(ident)) => {
            let ident = ident.to_string();
            if let Some(Token::EQUAL) = token_queue.peek_n(1) {
                token_queue.skip(2);
                let expression = parse_next_expression(token_queue, current_indent)?;
                Ok(Statement::Set(Identifier { name: ident }, expression))
            } else {
                let expression = parse_next_expression(token_queue, current_indent)?;
                Ok(Statement::Expression(expression))
            }
        }
        Some(Token::NEWLINE(_)) => {
            unreachable!("Should be handled by parse_block()");
        }
        Some(_) => {
            let expression = parse_next_expression(token_queue, current_indent)?;
            Ok(Statement::Expression(expression))
        }
        None => parse_err!("Expected statement"),
    }
}