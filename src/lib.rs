pub mod parse;
pub mod interpret;
pub mod constructs;
pub use constructs::program;
pub mod tokenize;
pub mod verify;

pub mod prelude {
    use crate::constructs::token::span::Span;

    pub type Result<T> = std::result::Result<T, MLGErr>;

    #[derive(Debug, Clone)]
    pub enum MLGErr {
        SyntaxErr(Option<Span>, String),
        SemanticErr(Option<Span>, String),
        ExecErr(String),
        CompilerErr(String),
    }

    impl ToString for MLGErr {
        fn to_string(&self) -> String {
            match self {
                Self::SyntaxErr(_, s) => format!("Invalid syntax: {}", s),
                Self::SemanticErr(_, s) => format!("Error: {}", s),
                Self::ExecErr(s) => format!("Failed to execute: {}", s),
                Self::CompilerErr(s) => format!("Failed to compile: {}", s),
            }
        }
    }

    macro_rules! syntax_err {
        ($span:expr, $($args:tt)*) => (Err(MLGErr::SyntaxErr($span, format!($($args)*))))
    }

    macro_rules! semantic_err {
        ($span:expr, $($args:tt)*) => (Err(MLGErr::SyntaxErr($span, format!($($args)*))))
    }

    macro_rules! exec_err {
        ($($arg:tt)*) => (Err(MLGErr::ExecErr(format!($($arg)*))))
    }

    macro_rules! compiler_err {
        ($($arg:tt)*) => (Err(MLGErr::ExecErr(format!($($arg)*))))
    }

    pub(crate) use {syntax_err, semantic_err, exec_err, compiler_err};
}