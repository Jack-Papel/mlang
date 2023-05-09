pub(self) mod parse_ast;
pub(self) mod parse_expr;
pub(self) mod parse_match_expr;
pub(self) mod parse_tokens;
pub(self) mod token_queue;

use crate::prelude::*;
use crate::program::Program;

pub fn parse(source: String) -> Result<Program> {
    parse_ast::to_ast(parse_tokens::tokenize(&source)?)
}