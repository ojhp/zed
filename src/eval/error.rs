use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::data::Expr;

/// An error type returned when problem occurs while evaluating
/// expressions.
#[derive(Debug)]
pub enum EvalError {
    /// A reference was made to an unbound variable.
    UnboundVariable(String),
    /// Not enough arguments were given to a function call.
    NotEnoughArguments(String, usize, usize),
    /// Too many arguments were given to a function call.
    TooManyArguments(String, usize, usize),
    /// A call form was encountered but was not a proper list.
    BadlyFormedCall(Expr),
    /// A call form was encountered without a valid function.
    NotAFunction(Expr),
    /// An error raised by a function implementation.
    FunctionError(String, Box<dyn Error>),
}

/// A result type returned by functions and methods that evaluate
/// expressions.
pub type EvalResult<T> = Result<T, EvalError>;

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EvalError::UnboundVariable(n) => write!(f, "unbound variable: `{}`", n),
            EvalError::NotEnoughArguments(n, m, a) => write!(
                f,
                "{}: wrong number of arguments: expected {}, got {}",
                n, m, a
            ),
            EvalError::TooManyArguments(n, m, a) => {
                write!(f, "{}: too many arguments: at most {}, got {}", n, m, a)
            }
            EvalError::BadlyFormedCall(e) => write!(f, "not a proper call form: {}", e),
            EvalError::NotAFunction(e) => write!(f, "not a function: {}", e),
            EvalError::FunctionError(n, e) => write!(f, "{}: {}", n, e),
        }
    }
}

impl Error for EvalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EvalError::FunctionError(_, e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
