pub(self) mod parse_ast;
pub(self) mod parse_expr;
pub(self) mod parse_match_expr;

use crate::constructs::token::Tokens;
use crate::prelude::*;
use crate::program::UnverifiedProgram;

pub fn parse(source: String) -> Result<UnverifiedProgram> {
    let parsed_tokens = crate::tokenize::parse_tokens(&source)?;

    parse_ast::to_ast(&mut Tokens::new(&parsed_tokens))
}