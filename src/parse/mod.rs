pub(self) mod parse_ast;
pub(self) mod parse_expr;
pub(self) mod parse_match_expr;

use crate::constructs::ast::AST;
use crate::constructs::token::Tokens;
use crate::prelude::*;

pub fn parse(tokens: &mut Tokens) -> Result<AST, CompilationError> {
    Ok(AST { 
        block: parse_ast::parse_block(tokens, 0)? 
    })
}