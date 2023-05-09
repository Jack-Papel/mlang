use std::fs::File;
use std::io::Read;

use mlang::prelude::*;
use mlang::program::Program;

fn main() -> Result<()> {
    let mut source = File::open("test_files/test2.mlg").unwrap();

    let mut mlg_str = String::new();
    source.read_to_string(&mut mlg_str).unwrap();

    let mut output_str = String::new();
    let result = Program::new(mlg_str).run(&mut output_str);

    println!("{}", output_str);

    result
}