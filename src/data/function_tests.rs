#![cfg(test)]

use test_case::test_case;

use crate::data::test_helpers::*;
use crate::data::*;
use crate::eval::{Environment, Evaluator};

#[test_case(None, None, 3; "no bounds")]
#[test_case(Some(2), None, 2; "on minimum")]
#[test_case(Some(4), None, 7; "above minimum")]
#[test_case(None, Some(3), 3; "on maximum")]
#[test_case(None, Some(5), 2; "below maximum")]
#[test_case(Some(3), Some(5), 3; "on minimum bound")]
#[test_case(Some(1), Some(8), 4; "within bounds")]
#[test_case(Some(5), Some(8), 8; "on maximum bound")]
#[test_case(Some(3), Some(3), 3; "on precise number")]
fn validate_bounds_ok(min: Option<usize>, max: Option<usize>, actual: usize) {
    let bounds = ArgBounds { min, max };
    let args = vec![nil(); actual];

    assert!(bounds.validate("name", &args).is_ok());
}

#[test]
fn calling_primitive_with_arg_eval_error() {
    let func = primitive("concat", None, None, concat_primitive);
    let args = vec![symbol("x")];
    let env = Environment::empty();

    let result = func.apply(args, &env, &Evaluator);

    assert_eq!("unbound variable: `x`", result.unwrap_err().to_string());
}

#[test]
fn calling_primitive_with_wrong_number_of_arguments() {
    let func = primitive("concat", Some(3), None, concat_primitive);
    let args = vec![string("test"), string("args")];
    let env = Environment::empty();

    let result = func.apply(args, &env, &Evaluator);

    assert_eq!(
        "concat: wrong number of arguments: expected 3, got 2",
        result.unwrap_err().to_string()
    );
}

#[test]
fn calling_primitive_function() {
    let func = primitive("concat", None, None, concat_primitive);
    let args = vec![symbol("a"), symbol("b")];
    let env = Environment::empty();
    env.define("a", &number(integer("45")));
    env.define("b", &boolean(false));

    let result = func.apply(args, &env, &Evaluator);

    assert_eq!(string("45, #f"), result.unwrap());
}

#[test]
fn failure_in_primitive_function_call() {
    let func = primitive("concat", None, None, concat_primitive);
    let env = Environment::empty();

    let result = func.apply(vec![], &env, &Evaluator);

    assert_eq!("concat: failed in call", result.unwrap_err().to_string());
}
