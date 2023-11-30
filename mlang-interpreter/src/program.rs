use crate::interpret::Executable;
use crate::interpret::environment::Env;

use mlang::constructs::ast::AST;
use mlang::constructs::token::{Tokens, Token};

use self::error_handling::display_error;

pub trait ProgramStatus { type ProgramData; }
pub struct Unparsed;
pub struct Tokenized;
pub struct Parsed;
pub struct Ready;
impl ProgramStatus for Unparsed { type ProgramData = (); }
impl ProgramStatus for Tokenized { type ProgramData = Vec<Token>; }
impl ProgramStatus for Parsed { type ProgramData = AST; }
impl ProgramStatus for Ready { type ProgramData = AST; }

pub struct Program<S: ProgramStatus> {
    source: String,
    data: S::ProgramData,
}

impl Program<Unparsed> {
    pub fn new(source: String) -> Result<Self, std::io::Error> {
        Ok(Program { 
            source,
            data: () 
        })
    }

    pub fn tokenize(self) -> Result<Program<Tokenized>, String> {
        let tokens = match mlang::tokenize::parse_tokens(&self.source) {
            Ok(tokens) => tokens,
            Err(e) => return Err(display_error(&self.source, e))
        };

        Ok(Program { 
            source: self.source, 
            data: tokens 
        })
    }
    
    // Shortcuts
    pub fn parse_and_run(self) -> Result<String, String> {
        self.tokenize()?
            .parse()?
            .verify()?
            .run()
    }
}

impl Program<Tokenized> {
    pub fn parse(self) -> Result<Program<Parsed>, String> {
        let mut tokens = Tokens::new(&self.data);
        let ast = match mlang::parse::parse(&mut tokens) {
            Ok(ast) => ast,
            Err(e) => return Err(display_error(&self.source, e))
        };

        Ok(Program { source: self.source, data: ast })
    }
}
impl Program<Parsed> {
    pub fn verify(self) -> Result<Program<Ready>, String> {
        let ast = match mlang::verify::verify(self.data) {
            Ok(ast) => ast,
            Err(e) => return Err(display_error(&self.source, e))
        };

        Ok(Program { source: self.source, data: ast })
    }
}

impl Program<Ready> {
    pub fn run(&self) -> Result<String, String> {
        let mut output = String::new();
        let mut env = Env::default();
        let result = self.data.0.execute(&mut env);

        env.write_to_string(&mut output);

        match result {
            Ok(_) => Ok(output),
            Err(e) => Err(e.to_string())
        }
    }
}

// Error prettifying code. (Warning: This sucks)
mod error_handling {
    use mlang::prelude::*;
    use mlang::constructs::token::span::Span;

    pub fn display_error(input_string: &String, err: MLGError) -> String {
        match err {
            ref err @ MLGError::SyntaxErr(Some(span), ..) |
            ref err @ MLGError::SemanticErr(Some(span), ..) => {
                let snippet = get_snippet(input_string, span);
                format!("An error was found in the following code:\n{}\n       {}\n{}", 
                    snippet, 
                    get_error_highlight(input_string, span),
                    err
                )
            }
            _ => err.to_string()
        }
    }

    fn line_number(input: &str, index: usize) -> usize {
        // Count the newlines before the given index
        1 + input.chars()
            .take(index)
            .filter(|&chr| chr == '\n')
            .count()
    }

    fn get_snippet(input: &str, span: Span) -> String {
        let line_span = {
            let beginning_line = line_number(input, span.beginning());
            Span {
                index: beginning_line as u32,
                len: 1 + (line_number(input, span.ending()) - beginning_line) as u16
            }
        };

        const NUM_PREV_LINES: usize = 3;

        input.lines()
            .enumerate()
            .map(|(index, line)| (index + 1, line))
            .skip(line_span.beginning().saturating_sub(NUM_PREV_LINES + 1))
            .take(line_span.length() + NUM_PREV_LINES)
            .fold(String::new(), |prev, (line_number, line)| {
                format!("{prev}\n{line_number:5}: {line}")
            })
    }
    
    fn get_error_highlight(input: &String, span: Span) -> String {
        let line_start_index;

        if let Some((index, _)) = input.as_bytes().iter()
            .enumerate()
            .take(span.beginning())
            .rev()
            .find(|(_, &chr)| chr as char == '\n') 
        {
            line_start_index = index + 1; // Line starts AFTER the newline
        } else {
            line_start_index = 0;
        }
        
        let blank_space_length = span.beginning() - line_start_index;
    
        " ".repeat(blank_space_length) + &"^".repeat(span.length())
    }
}