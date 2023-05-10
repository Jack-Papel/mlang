use std::fs::File;
use std::io::Read;

use mlang::prelude::*;
use mlang::program::Program;

fn main() -> Result<()> {
    print!("\x1Bc"); // Clear terminal

    let mut source = File::open("test_files/test2.mlg").unwrap();

    let mut mlg_str = String::new();
    source.read_to_string(&mut mlg_str).unwrap();

    let mut output_str = String::new();

    Program::new(mlg_str.clone())
        .unwrap_or_else(|error| error_handling::handle(mlg_str.clone(), error))
        .run(&mut output_str)
        .unwrap_or_else(|error| error_handling::handle(mlg_str.clone(), error));

    println!("{}", output_str);

    Ok(())
}

mod error_handling {
    use std::process::exit;

    use mlang::{prelude::MLGErr, constructs::token::span::Span};

    pub fn handle(input_string: String, error: MLGErr) -> ! {
        match error {
            MLGErr::ParseErr(Some(span), ..) => {
                show_error_location(&input_string, span)
            }
            _ => {}
        }
    
        println!("{}", error.to_string());
        exit(1)
    }
    
    /*
    Error prettifying code. (Warning: This sucks)
    */
    fn show_error_location(input_string: &String, span: Span) {
        let substring = get_offending_code(input_string, span);
    
        let mut highlight_str = get_error_highlight(&input_string, span);
        // When we add the line numbers, everything gets shifted.
        highlight_str.insert_str(0, "      \t");
    
        println!("An error was found in the following code:\n{}\n{}", substring, highlight_str);
    }
    
    fn get_offending_code(input_string: &String, span: Span) -> String {
        let mut lower_index = span.index.saturating_sub(10) as usize;
        let mut higher_index = span.index as usize + span.len as usize;
        let mut input_string = input_string.clone();
        
        const NUM_LINES: usize = 3;
        let mut lines_counted = 0;
    
        if let Some((idx, _)) = input_string.clone().as_bytes().iter()
            .enumerate()
            .take(lower_index)
            .rev()
            .find(|(idx, &chr)| {
                if chr == '\n' as u8 {
                    input_string.insert_str(*idx + 1, &(format!("{:5}:\t", get_line_number_for_index(&input_string, *idx))));
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
    
        return input_string[lower_index..higher_index].to_string();
    }
    
    fn get_error_highlight<'a>(input_string: &'a String, span: Span) -> String {
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
    
    fn get_line_number_for_index(input_string: &String, idx: usize) -> usize {
        let mut line_number = 1;
        let mut current_index = 0;
    
        while current_index < idx && current_index < input_string.len() {
            current_index += 1;
            if input_string.chars().nth(current_index) == Some('\n') {
                line_number += 1;
            }
        }
    
        return line_number;
    }
}