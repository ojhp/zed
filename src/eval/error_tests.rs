#![cfg(test)]

use std::error::Error;

use test_case::test_case;

use crate::data::test_helpers::*;
use crate::data::*;
use crate::eval::*;

#[test_case("+" => "unbound variable: `+`"; "symbol")]
#[test_case("add" => "unbound variable: `add`"; "short name")]
fn unbound_variable(name: &str) -> String {
    let error = EvalError::UnboundVariable(String::from(name));

    assert!(error.source().is_none());
    error.to_string()
}

#[test_case("+", 4, 3 => "+: wrong number of arguments: expected 4, got 3"; "symbol")]
#[test_case("add", 2, 0 => "add: wrong number of arguments: expected 2, got 0"; "short name")]
fn not_enough_arguments(name: &str, min: usize, actual: usize) -> String {
    let error = EvalError::NotEnoughArguments(String::from(name), min, actual);

    assert!(error.source().is_none());
    error.to_string()
}

#[test_case("+", 2, 3 => "+: too many arguments: at most 2, got 3"; "symbol")]
#[test_case("add", 8, 24 => "add: too many arguments: at most 8, got 24"; "short name")]
fn too_many_arguments(name: &str, min: usize, actual: usize) -> String {
    let error = EvalError::TooManyArguments(String::from(name), min, actual);

    assert!(error.source().is_none());
    error.to_string()
}

#[test_case(pair(symbol("+"), symbol("a")) => "not a proper call form: (+ . a)"; "a pair")]
#[test_case(pair(symbol("+"), pair(symbol("a"), symbol("b"))) => "not a proper call form: (+ a . b)"; "improper list")]
fn badly_formed_call(expr: Expr) -> String {
    let error = EvalError::BadlyFormedCall(expr);

    assert!(error.source().is_none());
    error.to_string()
}

#[test_case(number(real(56.78)) => "not a function: 56.78"; "a number")]
#[test_case(symbol("not-a-func") => "not a function: not-a-func"; "a symbol")]
fn not_a_function(expr: Expr) -> String {
    let error = EvalError::NotAFunction(expr);

    assert!(error.source().is_none());
    error.to_string()
}

#[test_case("do-something", "it broke" => "do-something: it broke"; "simple")]
#[test_case("+", "not a number: (123)" => "+: not a number: (123)"; "real-life example")]
fn function_error<E: Into<Box<dyn Error>>>(name: &str, err: E) -> String {
    let error = EvalError::FunctionError(String::from(name), err.into());

    assert!(error.source().is_some());
    error.to_string()
}
