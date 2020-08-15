use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

use peg::error::ParseError;
use peg::str::LineCol;

#[derive(Debug)]
pub enum ReadError {
    Eof,
    Io(IoError),
    Parse(ParseError<LineCol>),
}

pub type ReadResult<T> = Result<T, ReadError>;

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ReadError::Eof => write!(f, "end of file"),
            ReadError::Io(e) => write!(f, "I/O error: {}", e),
            ReadError::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl Error for ReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReadError::Eof => None,
            ReadError::Io(e) => Some(e),
            ReadError::Parse(e) => Some(e),
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
