use crate::eval::EvalResult;

pub trait Eval<T> {
    type Output;

    fn eval(&mut self, value: T) -> EvalResult<Self::Output>;
}
