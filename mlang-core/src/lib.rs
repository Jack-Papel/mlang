pub mod parse;
pub mod constructs;
pub mod tokenize;
pub mod verify;

pub mod prelude {
    use std::fmt::Display;
    use crate::constructs::token::span::Span;

    pub type Result<T> = std::result::Result<T, MLGError>;

    #[derive(Debug, Clone)]
    pub enum MLGError {
        SyntaxErr(Option<Span>, String),
        SemanticErr(Option<Span>, String),
        CompilerErr(String)
    }

    impl Display for MLGError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::SyntaxErr(_, s) => write!(f, "Invalid syntax: {}", s),
                Self::SemanticErr(_, s) => write!(f, "Error: {}", s),
                Self::CompilerErr(s) => write!(f, "Failed to compile: {}", s),
            }
        }
    }

    macro_rules! syntax_err {
        ($span:expr, $($args:tt)*) => (Err(MLGError::SyntaxErr($span, format!($($args)*))))
    }

    macro_rules! semantic_err {
        ($span:expr, $($args:tt)*) => (Err(MLGError::SyntaxErr($span, format!($($args)*))))
    }

    macro_rules! compiler_err {
        ($($arg:tt)*) => (Err(MLGError::CompilerErr(format!($($arg)*))))
    }

    pub(crate) use {syntax_err, semantic_err, compiler_err};
}