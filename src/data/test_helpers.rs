#![cfg(test)]

use num::{BigInt, Num};

use crate::data::Number;

pub fn integer<T: Into<BigInt>>(value: T) -> Number {
    Number::Integer(value.into())
}

pub fn integer_str<T: AsRef<str>>(s: T, radix: u32) -> Number {
    let value = BigInt::from_str_radix(s.as_ref(), radix).unwrap();
    Number::Integer(value)
}

pub fn real(value: f64) -> Number {
    Number::Real(value)
}
