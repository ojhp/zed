#![cfg(test)]

use test_case::test_case;

use crate::data::test_helpers::*;
use crate::data::*;

#[test_case("0"; "zero")]
#[test_case("1234567890"; "large number")]
#[test_case("-54321"; "negative number")]
fn integer_formatting(int: &str) {
    assert_eq!(int, integer(int).to_string());
}

#[test_case("0", "1"; "zero")]
#[test_case("1", "1"; "one")]
#[test_case("5", "1"; "integer")]
#[test_case("1", "4"; "fraction")]
#[test_case("-5", "6"; "negative")]
fn rational_formatting(numer: &str, denom: &str) {
    assert_eq!(
        format!("{}/{}", numer, denom),
        rational(numer, denom).to_string()
    );
}

#[test_case(0.0, "0.0"; "zero")]
#[test_case(1.0, "1.0"; "one")]
#[test_case(78.0, "78.0"; "integer")]
#[test_case(8.6, "8.6"; "fraction")]
#[test_case(-32.5, "-32.5"; "negative")]
fn real_formatting(value: f64, expected: &str) {
    assert_eq!(expected, real(value).to_string());
}

#[test_case(0.0, 0.0, "0+0i"; "zero")]
#[test_case(1.0, 0.0, "1+0i"; "one")]
#[test_case(0.0, 1.0, "0+1i"; "one imaginary")]
#[test_case(2.3, 4.5, "2.3+4.5i"; "complex number")]
#[test_case(-3.5, 2.8, "-3.5+2.8i"; "negative real")]
#[test_case(0.1, -3.5, "0.1-3.5i"; "negative imaginary")]
#[test_case(-5.6, -7.8, "-5.6-7.8i"; "negative both")]
fn complex_formatting(re: f64, im: f64, expected: &str) {
    assert_eq!(expected, complex(re, im).to_string());
}

#[test_case(integer("123"), integer("123") => true; "equal integers")]
#[test_case(integer("123"), integer("456") => false; "unequal integers")]
#[test_case(integer("123"), rational("123", "1") => true; "equal integer and rational")]
#[test_case(integer("123"), rational("123", "2") => false; "unequal integer and rational")]
#[test_case(integer("123"), real(123.0) => true; "equal integer and real")]
#[test_case(integer("123"), real(45.6) => false; "unqeual integer and real")]
#[test_case(integer("123"), complex(123.0, 0.0) => true; "equal integer and complex")]
#[test_case(integer("123"), complex(123.0, 456.0) => false; "unqeual integer and complex")]
#[test_case(rational("123", "1"), integer("123") => true; "equal rational and integer")]
#[test_case(rational("1", "23"), integer("123") => false; "unequal rational and integer")]
#[test_case(rational("1", "23"), rational("1", "23") => true; "equal rationals")]
#[test_case(rational("1", "23"), rational("12", "34") => false; "unequal rationals")]
#[test_case(rational("3", "2"), real(1.5) => true; "equal rational and real")]
#[test_case(rational("1", "23"), real(1.4) => false; "unequal rational and real")]
#[test_case(rational("3", "2"), complex(1.5, 0.0) => true; "equal rational and complex")]
#[test_case(rational("3", "2"), complex(1.5, 1.5) => false; "unequal rational and complex")]
#[test_case(real(12.0), integer("12") => true; "equal real and integer")]
#[test_case(real(12.3), integer("12") => false; "unequal real and integer")]
#[test_case(real(2.5), rational("5", "2") => true; "equal real and rational")]
#[test_case(real(12.3), rational("5", "2") => false; "unequal real and rational")]
#[test_case(real(12.3), real(12.3) => true; "equal reals")]
#[test_case(real(12.3), real(1.23) => false; "unequal reals")]
#[test_case(real(12.3), complex(12.3, 0.0) => true; "equal real and complex")]
#[test_case(real(12.3), complex(12.3, 4.56) => false; "unequal real and complex")]
#[test_case(complex(12.0, 0.0), integer("12") => true; "equal complex and integer")]
#[test_case(complex(12.0, 1.0), integer("12") => false; "unequal complex and integer")]
#[test_case(complex(0.75, 0.0), rational("3", "4") => true; "equal complex and rational")]
#[test_case(complex(0.75, 0.1), rational("3", "4") => false; "unequal complex and rational")]
#[test_case(complex(12.3, 0.0), real(12.3) => true; "equal complex and real")]
#[test_case(complex(12.3, 4.56), real(12.3) => false; "unequal complex and real")]
#[test_case(complex(12.3, 4.56), complex(12.3, 4.56) => true; "equal complexes")]
#[test_case(complex(12.3, 4.56), complex(1.23, 4.56) => false; "unequal complexes")]
fn equality(left: Number, right: Number) -> bool {
    left == right
}

#[test_case(integer("9876") => integer("-9876"); "integer negation")]
#[test_case(rational("123", "456") => rational("-123", "456"); "rational negation")]
#[test_case(real(38.65) => real(-38.65); "real negation")]
#[test_case(complex(12.3, 4.56) => complex(-12.3, -4.56); "complex negation")]
fn negation(value: Number) -> Number {
    -value
}

#[test_case(integer("123"), integer("123") => integer("246"); "two integers")]
#[test_case(integer("123"), rational("5", "4") => rational("497", "4"); "integer and rational")]
#[test_case(integer("123"), real(12.3) => real(135.3); "integer and real")]
#[test_case(integer("123"), complex(12.3, 4.56) => complex(135.3, 4.56); "integer and complex")]
#[test_case(rational("5", "4"), integer("123") => rational("497", "4"); "rational and integer")]
#[test_case(rational("5", "4"), rational("5", "4") => rational("10", "4"); "two rationals")]
#[test_case(rational("5", "4"), real(12.3) => real(13.55); "rational and real")]
#[test_case(rational("5", "4"), complex(12.3, 4.56) => complex(13.55, 4.56); "rational and complex")]
#[test_case(real(12.3), integer("123") => real(135.3); "real and integer")]
#[test_case(real(12.3), rational("5", "4") => real(13.55); "real and rational")]
#[test_case(real(12.3), real(12.3) => real(24.6); "two reals")]
#[test_case(real(12.3), complex(12.3, 4.56) => complex(24.6, 4.56); "real and complex")]
#[test_case(complex(12.3, 4.56), integer("123") => complex(135.3, 4.56); "complex and integer")]
#[test_case(complex(12.3, 4.56), rational("5", "4") => complex(13.55, 4.56); "complex and rational")]
#[test_case(complex(12.3, 4.56), real(12.3) => complex(24.6, 4.56); "complex and real")]
#[test_case(complex(12.3, 4.56), complex(12.3, 4.56) => complex(24.6, 9.12); "two complexes")]
fn addition(left: Number, right: Number) -> Number {
    left + right
}

#[test_case(integer("123"), integer("56") => integer("67"); "two integers")]
#[test_case(integer("86"), rational("5", "6") => rational("511", "6"); "integer and rational")]
#[test_case(rational("55", "4"), real(-9.8) => real(23.55); "rational and real")]
#[test_case(complex(2.6, 3.4), real(8.6) => complex(-6.0, 3.4); "complex and real")]
#[test_case(complex(2.6, 3.4), complex(8.6, 1.2) => complex(-6.0, 2.2); "two complexes")]
fn subtraction(left: Number, right: Number) -> Number {
    left - right
}

#[test_case(integer("7"), integer("-8") => integer("-56"); "two integers")]
#[test_case(rational("8", "7"), integer("9") => rational("72", "7"); "rational and integer")]
#[test_case(real(2.5), rational("3", "4") => real(1.875); "real and rational")]
#[test_case(real(2.0), complex(1.2, 3.4) => complex(2.4, 6.8); "real and complex")]
#[test_case(complex(2.0, 3.0), complex(1.0, 4.0) => complex(-10.0, 11.0); "two complexes")]
fn multiplication(left: Number, right: Number) -> Number {
    left * right
}

#[test_case(integer("8"), integer("3") => integer("2"); "two integers")]
#[test_case(rational("9", "5"), real(2.0) => real(0.9); "rational and real")]
fn division(left: Number, right: Number) -> Number {
    left / right
}

#[test_case(integer("8"), integer("3") => integer("2"); "two integers")]
#[test_case(rational("9", "5"), real(2.0) => real(1.8); "rational and real")]
fn modulo(left: Number, right: Number) -> Number {
    left % right
}
