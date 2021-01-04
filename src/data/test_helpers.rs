#![cfg(test)]

use std::rc::Rc;

use num::{BigInt, Num};

use crate::data::{Expr, Expression, Number};

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

pub fn nil() -> Expr {
    Rc::new(Expression::Nil)
}

pub fn number(value: Number) -> Expr {
    Rc::new(Expression::Number(value))
}

pub fn symbol<T: ToString>(name: T) -> Expr {
    Rc::new(Expression::Symbol(name.to_string()))
}

pub fn pair(car: Expr, cdr: Expr) -> Expr {
    Rc::new(Expression::Pair(car, cdr))
}

pub fn list(items: Vec<Expr>) -> Expr {
    let mut head = nil();

    for item in items.into_iter().rev() {
        head = pair(item, head);
    }

    head
}

macro_rules! list {
    ($($items:expr),* $(,)?) => {
        list(vec![$($items),*])
    };
}
