use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::eval::EvalError;
use crate::print::PrintError;
use crate::read::ReadError;

#[derive(Debug)]
pub enum ZedError {
    Read(ReadError),
    Eval(EvalError),
    Print(PrintError),
}

pub type ZedResult<T> = Result<T, ZedError>;

impl Display for ZedError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ZedError::*;

        match self {
            Read(e) => e.fmt(f),
            Eval(e) => e.fmt(f),
            Print(e) => e.fmt(f),
        }
    }
}

impl Error for ZedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use ZedError::*;

        match self {
            Read(e) => e.source(),
            Eval(e) => e.source(),
            Print(e) => e.source(),
        }
    }
}

impl From<ReadError> for ZedError {
    fn from(err: ReadError) -> ZedError {
        ZedError::Read(err)
    }
}

impl From<EvalError> for ZedError {
    fn from(err: EvalError) -> ZedError {
        ZedError::Eval(err)
    }
}

impl From<PrintError> for ZedError {
    fn from(err: PrintError) -> ZedError {
        ZedError::Print(err)
    }
}
