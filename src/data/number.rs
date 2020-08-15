use std::f64::NAN;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::{BigInt, BigRational, Complex, FromPrimitive, One, ToPrimitive, Zero};

/// A number as one of the four available types.
#[derive(Clone, Debug)]
pub enum Number {
    /// An integer number of arbitrary length.
    Integer(BigInt),
    /// A rational number consisting of a numerator and a denominator
    /// of arbitrary length.
    Rational(BigRational),
    /// A real number represented by a 64-bit floating-point number.
    Real(f64),
    /// A complex number consisting of two 64-bit floating-point
    /// numbers, the real component and the imaginary component.
    Complex(Complex<f64>),
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Number::Integer(n) => n.fmt(f),
            Number::Rational(n) => write!(f, "{}/{}", n.numer(), n.denom()),
            Number::Real(n) => std::fmt::Debug::fmt(n, f),
            Number::Complex(n) => n.fmt(f),
        }
    }
}

macro_rules! apply_op {
    ($op:tt, $value:expr, $wrapped:tt) => {
        match $value {
            Number::Integer(n) => apply_op!(@$wrapped, Integer, $op, n),
            Number::Rational(n) => apply_op!(@$wrapped, Rational, $op, n),
            Number::Real(n) => apply_op!(@$wrapped, Real, $op, n),
            Number::Complex(n) => apply_op!(@$wrapped, Complex, $op, n),
        }
    };

    ($op:tt, $left:expr, $right:expr, $wrapped:tt) => {
        match ($left, $right) {
            (Number::Integer(l), Number::Integer(r)) => apply_op!(@$wrapped, Integer, $op, l, r),
            (Number::Integer(l), Number::Rational(r)) => apply_op!(@$wrapped, Rational, $op, &l.to_rational(), r),
            (Number::Integer(l), Number::Real(r)) => apply_op!(@$wrapped, Real, $op, &l.to_real(), r),
            (Number::Integer(l), Number::Complex(r)) => apply_op!(@$wrapped, Complex, $op, &l.to_complex(), r),
            (Number::Rational(l), Number::Integer(r)) => apply_op!(@$wrapped, Rational, $op, l, &r.to_rational()),
            (Number::Rational(l), Number::Rational(r)) => apply_op!(@$wrapped, Rational, $op, l, r),
            (Number::Rational(l), Number::Real(r)) => apply_op!(@$wrapped, Real, $op, &l.to_real(), r),
            (Number::Rational(l), Number::Complex(r)) => apply_op!(@$wrapped, Complex, $op, &l.to_complex(), r),
            (Number::Real(l), Number::Integer(r)) => apply_op!(@$wrapped, Real, $op, l, &r.to_real()),
            (Number::Real(l), Number::Rational(r)) => apply_op!(@$wrapped, Real, $op, l, &r.to_real()),
            (Number::Real(l), Number::Real(r)) => apply_op!(@$wrapped, Real, $op, l, r),
            (Number::Real(l), Number::Complex(r)) => apply_op!(@$wrapped, Complex, $op, &l.to_complex(), r),
            (Number::Complex(l), Number::Integer(r)) => apply_op!(@$wrapped, Complex, $op, l, &r.to_complex()),
            (Number::Complex(l), Number::Rational(r)) => apply_op!(@$wrapped, Complex, $op, l, &r.to_complex()),
            (Number::Complex(l), Number::Real(r)) => apply_op!(@$wrapped, Complex, $op, l, &r.to_complex()),
            (Number::Complex(l), Number::Complex(r)) => apply_op!(@$wrapped, Complex, $op, l, r),
        }
    };

    (@unwrapped, $kind:ident, $op:tt, $value:expr) => { $op $value };
    (@wrapped, $kind:ident, $op:tt, $value:expr) => { Number::$kind($op $value) };
    (@wrapped, $kind:ident, $op:tt, $left:expr, $right:expr) => { Number::$kind($left $op $right) };
    (@unwrapped, $kind:ident, $op:tt, $left:expr, $right:expr) => { $left $op $right };
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        apply_op!(==, self, other, unwrapped)
    }
}

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $op:tt, unary) => {
        impl $trait for Number {
            type Output = Number;

            fn $fn(self) -> Number {
                apply_op!($op, self, wrapped)
            }
        }

        impl<'a> $trait for &'a Number {
            type Output = Number;

            fn $fn(self) -> Number {
                apply_op!($op, self, wrapped)
            }
        }
    };

    ($trait:ident, $fn:ident, $op:tt, binary) => {
        impl $trait<Number> for Number {
            type Output = Number;

            fn $fn(self, other: Number) -> Number {
                apply_op!($op, self, other, wrapped)
            }
        }

        impl<'a> $trait<&'a Number> for Number {
            type Output = Number;

            fn $fn(self, other: &'a Number) -> Number {
                apply_op!($op, &self, other, wrapped)
            }
        }

        impl<'a> $trait<Number> for &'a Number {
            type Output = Number;

            fn $fn(self, other: Number) -> Number {
                apply_op!($op, self, &other, wrapped)
            }
        }

        impl<'a, 'b> $trait<&'a Number> for &'b Number {
            type Output = Number;

            fn $fn(self, other: &'a Number) -> Number {
                apply_op!($op, self, other, wrapped)
            }
        }
    };
}

impl_op!(Neg, neg, -, unary);
impl_op!(Add, add, +, binary);
impl_op!(Sub, sub, -, binary);
impl_op!(Mul, mul, *, binary);
impl_op!(Div, div, /, binary);
impl_op!(Rem, rem, %, binary);

trait Convert {
    fn to_integer(&self) -> BigInt;
    fn to_rational(&self) -> BigRational;
    fn to_real(&self) -> f64;
    fn to_complex(&self) -> Complex<f64>;
}

impl Convert for BigInt {
    fn to_integer(&self) -> BigInt { self.clone() }
    fn to_rational(&self) -> BigRational { BigRational::new(self.clone(), BigInt::one()) }
    fn to_real(&self) -> f64 { self.to_f64().unwrap_or(NAN) }
    fn to_complex(&self) -> Complex<f64> { Complex::new(self.to_real(), 0.0) }
}

impl Convert for BigRational {
    fn to_integer(&self) -> BigInt { self.numer() / self.denom() }
    fn to_rational(&self) -> BigRational { self.clone() }
    fn to_real(&self) -> f64 { self.to_f64().unwrap_or(NAN) }
    fn to_complex(&self) -> Complex<f64> { Complex::new(self.to_real(), 0.0) }
}

impl Convert for f64 {
    fn to_integer(&self) -> BigInt { BigInt::from_f64(*self).unwrap_or_else(BigInt::zero) }
    fn to_rational(&self) -> BigRational { BigRational::from_f64(*self).unwrap_or_else(BigRational::zero) }
    fn to_real(&self) -> f64 { *self }
    fn to_complex(&self) -> Complex<f64> { Complex::new(*self, 0.0) }
}

impl Convert for Complex<f64> {
    fn to_integer(&self) -> BigInt { self.re.to_integer() }
    fn to_rational(&self) -> BigRational { self.re.to_rational() }
    fn to_real(&self) -> f64 { self.re }
    fn to_complex(&self) -> Complex<f64> { *self }
}
