#![cfg(test)]

use test_case::test_case;

use crate::data::test_helpers::*;
use crate::data::Expr;
use crate::eval::*;

#[test_case(nil(); "nil value")]
#[test_case(boolean(true); "boolean value")]
#[test_case(number(integer("54321")); "integer value")]
#[test_case(number(rational("54", "321")); "rational value")]
#[test_case(number(real(34.78)); "real value")]
#[test_case(number(complex(8.4, 7.3)); "complex value")]
#[test_case(character('x'); "character value")]
#[test_case(string("a value"); "string value")]
#[test_case(vector(vec![nil(), character('f')]); "vector value")]
#[test_case(byte_vector(vec![1, 2, 3, 4]); "byte vector value")]
fn simple_evaluation(value: Expr) {
    let env = Environment::empty();
    let result = Evaluator.eval(value.clone(), &env).unwrap();

    assert_eq!(value, result);
}

#[test_case("a", string("test"), 0; "basic variable lookup")]
#[test_case("b", character('4'), 1; "outer environment lookup")]
#[test_case("c", number(real(34.56)), 100; "multi-level environment lookup")]
fn variable_lookup(name: &str, value: Expr, depth: usize) {
    let mut env = Environment::empty();
    env.define(name, &value);

    for _ in 0..depth {
        env = Environment::inner(&env);
    }

    assert_eq!(value, env.get(name).unwrap());
}
