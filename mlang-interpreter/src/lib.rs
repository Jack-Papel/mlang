pub mod interpret;
pub mod program;

pub mod prelude {
    use std::fmt::Display;

    pub type Result<T> = std::result::Result<T, ExecutionError>;

    pub struct ExecutionError(pub String);

    impl Display for ExecutionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    macro_rules! exec_err {
        ($($arg:tt)*) => (Err(ExecutionError(format!($($arg)*))))
    }

    pub(crate) use exec_err;
}