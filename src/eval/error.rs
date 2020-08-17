use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An error type returned when problem occurs while evaluating
/// expressions.
#[derive(Debug)]
pub enum EvalError {
    /// A reference was made to an unbound variable.
    UnboundVariable(String),
}

/// A result type returned by functions and methods that evaluate
/// expressions.
pub type EvalResult<T> = Result<T, EvalError>;

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EvalError::UnboundVariable(n) => write!(f, "unbound variable: `{}`", n),
        }
    }
}

impl Error for EvalError {}
