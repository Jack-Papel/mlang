mod parse_ast;
mod parse_expr;
mod parse_match_expr;

use crate::constructs::ast::AST;
use crate::constructs::token::Tokens;
use crate::prelude::*;

pub fn parse(tokens: &mut Tokens) -> Result<AST> {
    Ok(AST(parse_ast::parse_block(tokens, 0)?))
}