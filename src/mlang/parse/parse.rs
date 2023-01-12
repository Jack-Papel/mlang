use crate::prelude::*;
use crate::mlang::parse::*;
use crate::mlang::program::Program;

// uhmmm... >~<
pub fn parse(source: String) -> Result<Program> {
    parse_ast::to_ast(parse_tokens::tokenize(&source))
}