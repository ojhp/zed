#![cfg(test)]

use std::error::Error;
use std::rc::Rc;

use num::{BigInt, BigRational, Complex, Num};

use crate::data::*;

pub fn integer(value: &str) -> Number {
    Number::Integer(BigInt::from_str_radix(value, 10).unwrap())
}

pub fn rational(numer: &str, denom: &str) -> Number {
    let numer = BigInt::from_str_radix(numer, 10).unwrap();
    let denom = BigInt::from_str_radix(denom, 10).unwrap();
    Number::Rational(BigRational::new(numer, denom))
}

pub fn real(value: f64) -> Number {
    Number::Real(value)
}

pub fn complex(re: f64, im: f64) -> Number {
    Number::Complex(Complex::new(re, im))
}

pub fn primitive(
    name: &str,
    min: Option<usize>,
    max: Option<usize>,
    func: PrimitiveFn,
) -> Function {
    Function::Primitive(String::from(name), ArgBounds { min, max }, func)
}

pub fn nil() -> Expr {
    Rc::new(Expression::Nil)
}

pub fn boolean(value: bool) -> Expr {
    Rc::new(Expression::Boolean(value))
}

pub fn number(value: Number) -> Expr {
    Rc::new(Expression::Number(value))
}

pub fn character(ch: char) -> Expr {
    Rc::new(Expression::Character(ch))
}

pub fn string(text: &str) -> Expr {
    Rc::new(Expression::String(String::from(text)))
}

pub fn symbol(name: &str) -> Expr {
    Rc::new(Expression::Symbol(String::from(name)))
}

pub fn pair(car: Expr, cdr: Expr) -> Expr {
    Rc::new(Expression::Pair(car, cdr))
}

pub fn list(items: Vec<Expr>) -> Expr {
    let mut head = Rc::new(Expression::Nil);

    for item in items.into_iter().rev() {
        head = Rc::new(Expression::Pair(item, head));
    }

    head
}

pub fn vector(items: Vec<Expr>) -> Expr {
    Rc::new(Expression::Vector(items))
}

pub fn byte_vector(items: Vec<u8>) -> Expr {
    Rc::new(Expression::ByteVector(items))
}

pub fn function(func: Function) -> Expr {
    Rc::new(Expression::Function(func))
}

pub fn concat_primitive(args: Vec<Expr>) -> Result<Expr, Box<dyn Error>> {
    if args.len() == 0 {
        return Err("failed in call".into());
    }

    let text = args.iter().map(|a| a.to_string()).collect::<Vec<String>>();
    Ok(string(&text.join(", ")))
}
