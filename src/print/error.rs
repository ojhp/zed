use std::error::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum PrintError {
    Io(IoError),
}

pub type PrintResult<T> = Result<T, PrintError>;

impl Display for PrintError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use PrintError::*;

        match self {
            Io(e) => write!(f, "i/o error: {}", e),
        }
    }
}

impl Error for PrintError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use PrintError::*;

        match self {
            Io(e) => Some(e),
        }
    }
}

impl From<IoError> for PrintError {
    fn from(err: IoError) -> PrintError {
        PrintError::Io(err)
    }
}
