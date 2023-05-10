pub mod parse;
pub mod interpret;
pub mod constructs;
pub use constructs::program;

pub mod prelude {
    use crate::constructs::token::span::Span;

    pub type Result<T> = std::result::Result<T, MLGErr>;

    #[derive(Debug, Clone)]
    pub enum MLGErr {
        ParseErr(Option<Span>, String),
        ExecErr(String),
        CompilerErr(String),
    }

    impl ToString for MLGErr {
        fn to_string(&self) -> String {
            match self {
                Self::ParseErr(span, s) => format!("Failed to parse: {}", s),
                Self::ExecErr(s) => format!("Failed to execute: {}", s),
                Self::CompilerErr(s) => format!("Failed to compile: {}", s),
            }
        }
    }

    macro_rules! parse_err {
        ($span:expr, $($args:tt)*) => (Err(MLGErr::ParseErr($span, format!($($args)*))))
    }

    macro_rules! exec_err {
        ($($arg:tt)*) => (Err(MLGErr::ExecErr(format!($($arg)*))))
    }

    pub(crate) use {parse_err, exec_err};
}