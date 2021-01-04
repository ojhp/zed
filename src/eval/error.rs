use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum EvalError {
    Custom(Box<dyn Error>),
}

pub type EvalResult<T> = Result<T, EvalError>;

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use EvalError::*;

        match self {
            Custom(e) => e.fmt(f),
        }
    }
}

impl Error for EvalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use EvalError::*;

        match self {
            Custom(e) => e.source(),
        }
    }
}
