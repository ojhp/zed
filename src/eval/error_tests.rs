#![cfg(test)]

use std::error::Error;

use crate::eval::*;

#[test]
fn unbound_variable() {
    let error = EvalError::UnboundVariable(String::from("test-variable"));

    assert_eq!("unbound variable: `test-variable`", error.to_string());
    assert!(error.source().is_none());
}
