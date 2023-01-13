pub(self) mod parse_ast;
pub(self) mod parse_expr;
pub(self) mod parse_match_expr;
pub(self) mod parse_tokens;
pub(self) mod tokens;
pub(self) mod util;

use crate::prelude::*;
use crate::mlang::program::Program;

pub fn parse(source: String) -> Result<Program> {
    parse_ast::to_ast(parse_tokens::tokenize(&source)?)
}