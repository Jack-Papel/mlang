use mlang::program::Program;

fn main() {
    print!("\x1Bc"); // Clear terminal

    let source = std::fs::read_to_string("./test_files/test2.mlg").unwrap();

    match Program::new(source).unwrap().parse_and_run() {
        Ok(output) => println!("{}", output),
        Err(error) => {
            println!("{}", error);
            std::process::exit(1)
        },
    }
}