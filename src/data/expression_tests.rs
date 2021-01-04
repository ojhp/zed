#![cfg(test)]

use test_case::test_case;

use crate::data::Expr;
use crate::data::test_helpers::*;

#[test_case(nil(), nil() => true; "nil equals self")]
#[test_case(nil(), symbol("nil") => false; "nil not equal to symbol `nil`")]
#[test_case(number(integer(5)), number(real(5.0)) => true; "equal numbers equal")]
#[test_case(number(integer(-7)), number(real(-7.1)) => false; "inequal numbers not equal")]
#[test_case(number(integer(43)), symbol("43") => false; "number not equal to symbol of number")]
#[test_case(symbol("test"), symbol("test") => true; "symbols with same name equal")]
#[test_case(symbol("a"), symbol("ab") => false; "symbols with different names not equal")]
#[test_case(pair(symbol("x"), symbol("y")), pair(symbol("x"), symbol("y")) => true; "identical pairs equal")]
#[test_case(list!(symbol("x"), symbol("y")), list!(symbol("x"), symbol("y")) => true; "identical lists equal")]
#[test_case(pair(symbol("x"), symbol("y")), list!(symbol("x"), symbol("y")) => false; "list and pair not equal")]
fn equality(a: Expr, b: Expr) -> bool {
    a == b
}

#[test_case(nil() => "()"; "nil value")]
#[test_case(number(integer(-5)) => "-5"; "integral number")]
#[test_case(number(real(12.34)) => "12.34"; "real number")]
#[test_case(symbol("+") => "+"; "simple symbol")]
#[test_case(symbol("test") => "test"; "text symbol")]
#[test_case(symbol("quoted name") => "|quoted name|"; "quoted symbol")]
#[test_case(symbol("escaped\nname\u{0}") => "|escaped\\nname\\x0;|"; "escaped symbol")]
#[test_case(pair(symbol("a"), symbol("b")) => "(a . b)"; "simple pair")]
#[test_case(pair(symbol("a"), pair(symbol("b"), symbol("c"))) => "(a b . c)"; "dotted list")]
#[test_case(list!(symbol("a"), symbol("b"), symbol("c")) => "(a b c)"; "regular list")]
fn formatting(value: Expr) -> String {
    value.to_string()
}
