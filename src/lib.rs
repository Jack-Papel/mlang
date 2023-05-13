pub mod parse;
pub mod interpret;
pub mod constructs;
pub mod tokenize;
pub mod verify;
pub mod program;

pub mod prelude {
    use std::fmt::Display;

    use crate::constructs::token::span::Span;

    #[derive(Debug, Clone)]
    pub enum MLGError {
        CompilationError(CompilationError),
        ExecutionError(ExecutionError),
    }

    #[derive(Debug, Clone)]
    pub enum CompilationError {
        SyntaxErr(Option<Span>, String),
        SemanticErr(Option<Span>, String),
        CompilerErr(String)
    }

    #[derive(Debug, Clone)]
    pub enum ExecutionError {
        // TODO, make different execution error types.
        ExecErr(String),
    }

    impl Display for MLGError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::CompilationError(err) => err.fmt(f),
                Self::ExecutionError(err) => err.fmt(f)
            }
        }
    }

    impl Display for CompilationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::SyntaxErr(_, s) => write!(f, "Invalid syntax: {}", s),
                Self::SemanticErr(_, s) => write!(f, "Error: {}", s),
                Self::CompilerErr(s) => write!(f, "Failed to compile: {}", s),
            }
        }
    }

    impl Display for ExecutionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ExecErr(s) => write!(f, "Failed to execute: {}", s),
            }
        }
    }

    macro_rules! syntax_err {
        ($span:expr, $($args:tt)*) => (Err(CompilationError::SyntaxErr($span, format!($($args)*))))
    }

    macro_rules! semantic_err {
        ($span:expr, $($args:tt)*) => (Err(CompilationError::SyntaxErr($span, format!($($args)*))))
    }

    macro_rules! compiler_err {
        ($($arg:tt)*) => (Err(CompilationError::CompilerErr(format!($($arg)*))))
    }

    macro_rules! exec_err {
        ($($arg:tt)*) => (Err(ExecutionError::ExecErr(format!($($arg)*))))
    }

    pub(crate) use {syntax_err, semantic_err, exec_err, compiler_err};
}