use super::ast::Block;
use super::interpret::execution::{Executable, Env};
use super::parse::parse;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Program {
    pub block: Block,
}

impl Program {
    pub fn new(program: String) -> Self {
        parse::parse(program).unwrap()
    }

    pub fn run(&self, output: &mut String) -> Result<()> {
        println!("Running program");
        let mut env = Env::new();
        self.block.execute(&mut env)?;

        env.write_to_string(output);

        Ok(())
    }
}