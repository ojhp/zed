#![cfg(test)]

use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

use crate::read::*;

#[test]
fn end_of_file() {
    let error = ReadError::Eof;

    assert_eq!("end of file", error.to_string());
    assert!(error.source().is_none());
}

#[test]
fn io_error() {
    let io_error: IoError = ErrorKind::PermissionDenied.into();
    let error = ReadError::Io(ErrorKind::PermissionDenied.into());

    assert_eq!(format!("I/O error: {}", io_error), error.to_string());
    assert!(error.source().is_some());
    assert_eq!(error.source().unwrap().to_string(), io_error.to_string());
}
