use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, ErrorKind};

use peg::error::ParseError;
use peg::str::LineCol;
use rustyline::error::ReadlineError;

#[derive(Debug)]
pub enum ReadError {
    Eof,
    Io(IoError),
    Parse(ParseError<LineCol>),
}

pub type ReadResult<T> = Result<T, ReadError>;

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ReadError::*;

        match self {
            Eof => write!(f, "end-of-file"),
            Io(e) => write!(f, "i/o error: {}", e),
            Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl Error for ReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use ReadError::*;

        match self {
            Eof => None,
            Io(e) => Some(e),
            Parse(e) => Some(e),
        }
    }
}

impl From<IoError> for ReadError {
    fn from(err: IoError) -> ReadError {
        ReadError::Io(err)
    }
}

impl From<ParseError<LineCol>> for ReadError {
    fn from(err: ParseError<LineCol>) -> ReadError {
        ReadError::Parse(err)
    }
}

impl From<ReadlineError> for ReadError {
    fn from(err: ReadlineError) -> ReadError {
        use ReadlineError::*;

        match err {
            Eof => ReadError::Eof,
            Io(e) => ReadError::Io(e),
            _ => ReadError::Io(IoError::new(ErrorKind::Other, err)),
        }
    }
}
