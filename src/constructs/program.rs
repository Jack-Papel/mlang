use crate::prelude::*;
use super::ast::Block;
use crate::interpret::execution::{Executable, Env};

#[derive(Debug, Clone)]
pub struct Program {
    pub block: Block,
}

impl Program {
    pub fn new(program: String) -> Self {
        crate::parse::parse(program).unwrap()
    }

    pub fn run(&self, output: &mut String) -> Result<()> {
        println!("Running program");
        let mut env = Env::new();
        let result = self.block.execute(&mut env);

        env.write_to_string(output);

        result.map(|_|())
    }
}