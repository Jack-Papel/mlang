use std::fs::File;
use std::io::Read;
use std::process::exit;

use mlang::constructs::token::span::Span;
use mlang::prelude::*;
use mlang::program::Program;

fn main() -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");

    let mut source = File::open("test_files/test2.mlg").unwrap();

    let mut mlg_str = String::new();
    source.read_to_string(&mut mlg_str).unwrap();

    let mut output_str = String::new();

    Program::new(mlg_str.clone())
        .unwrap_or_else(|error| handle_error(mlg_str.clone(), error))
        .run(&mut output_str)
        .unwrap_or_else(|error| handle_error(mlg_str.clone(), error));

    println!("{}", output_str);

    Ok(())
}

fn handle_error(input_string: String, error: MLGErr) -> ! {
    match error {
        MLGErr::ParseErr(Some(span), ..) => {
            let substring = slice_preceding_code(&input_string, span);
            println!("An error was found in the following code:\n{}{}\n{}", "-".repeat(60), substring, highlight_error(&input_string, span));
        }
        _ => {}
    }

    println!("{}", error.to_string());
    exit(1);
}

fn slice_preceding_code<'a>(input_string: &'a String, span: Span) -> &'a str {
    let mut lower_index = span.index.saturating_sub(10) as usize;
    let mut higher_index = span.index as usize + span.len as usize;
    
    const NUM_LINES: usize = 3;
    let mut lines_counted = 0;

    if let Some((idx, _)) = input_string.as_bytes().iter()
        .take(lower_index)
        .enumerate()
        .rev()
        .find(|(_, &chr)| {
            if chr == '\n' as u8 {
                lines_counted += 1;
                return lines_counted >= NUM_LINES;
            }
            return false;
        }) {
        lower_index = idx;
    } else {
        lower_index = 0;
    }
    
    if let Some((idx, _)) = input_string.as_bytes().iter()
        .skip(higher_index)
        .enumerate()
        .find(|(_, &chr)| chr == '\n' as u8) 
    {
        higher_index = higher_index + idx;
    } else {
        higher_index = input_string.len() - 1;
    }

    return &input_string[lower_index..higher_index];
}

fn highlight_error<'a>(input_string: &'a String, span: Span) -> String {
    let mut line_start_index = span.index as usize;
    if let Some((idx, _)) = input_string.as_bytes().iter()
        .enumerate()
        .take(line_start_index)
        .rev()
        .find(|(_, &chr)| chr == '\n' as u8) 
    {
        line_start_index = idx;
    } else {
        line_start_index = 0;
    }
    
    let blank_space_length = span.index as usize - line_start_index - 1;

    return " ".repeat(blank_space_length) + &"^".repeat(span.len as usize);
}