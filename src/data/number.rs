use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use num::{BigInt, ToPrimitive};

#[derive(Clone, Debug)]
pub enum Number {
    Integer(BigInt),
    Real(f64),
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        use Number::*;

        fn i2r(integer: &BigInt) -> f64 {
            integer.to_f64().unwrap_or(f64::NAN)
        }

        match (self, other) {
            (Integer(a), Integer(b)) => a == b,
            (Integer(a), Real(b)) => &i2r(a) == b,
            (Real(a), Integer(b)) => a == &i2r(b),
            (Real(a), Real(b)) => a == b,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Number::*;

        match self {
            Integer(i) => Display::fmt(i, f),
            Real(r) => Debug::fmt(r, f),
        }
    }
}
