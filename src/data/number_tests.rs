#![cfg(test)]

use test_case::test_case;

use crate::data::Number;
use crate::data::test_helpers::*;

#[test_case(integer(876), integer(876) => true; "equal integers")]
#[test_case(integer(-5), integer(5) => false; "inequal integers")]
#[test_case(real(8.32), real(8.32) => true; "equal decimals")]
#[test_case(real(8.32), real(8.33) => false; "inequal decimals")]
#[test_case(integer(-45), real(-45.0) => true; "equal integer and real")]
#[test_case(integer(3), real(3.001) => false; "inequal integer and real")]
fn equality(a: Number, b: Number) -> bool {
    a == b
}

#[test_case(integer(984) => "984"; "positive integer")]
#[test_case(integer(-3) => "-3"; "negative integer")]
#[test_case(integer_str("98765432123456789", 10) => "98765432123456789"; "large integer")]
#[test_case(real(1.765) => "1.765"; "positive real")]
#[test_case(real(-87.654) => "-87.654"; "negative real")]
#[test_case(real(0.234) => "0.234"; "fraction real")]
#[test_case(real(97.0) => "97.0"; "integral real")]
fn formatting(value: Number) -> String {
    value.to_string()
}
