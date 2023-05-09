pub mod parse;
pub mod interpret;
pub mod constructs;
pub use constructs::program;

pub mod prelude {
    pub type Result<T> = std::result::Result<T, MLGErr>;

    #[derive(Debug, Clone)]
    pub enum MLGErr {
        ParseErr(String),
        ExecErr(String),
    }

    impl ToString for MLGErr {
        fn to_string(&self) -> String {
            match self {
                Self::ParseErr(s) => format!("Failed to parse: {}", s),
                Self::ExecErr(s) => format!("Failed to execute: {}", s),
            }
        }
    }

    macro_rules! parse_err {
        ($($arg:tt)*) => (Err(MLGErr::ParseErr(format!($($arg)*))))
    }

    macro_rules! exec_err {
        ($($arg:tt)*) => (Err(MLGErr::ExecErr(format!($($arg)*))))
    }

    pub(crate) use {parse_err, exec_err};
}