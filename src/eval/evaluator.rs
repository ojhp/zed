use crate::data::{Expr, Expression};
use crate::eval::{Env, EvalResult};

pub trait Eval {
    fn eval(&self, value: Expr, env: &Env) -> EvalResult<Expr>;
}

pub struct Evaluator;

impl Eval for Evaluator {
    fn eval(&self, value: Expr, env: &Env) -> EvalResult<Expr> {
        match *value {
            Expression::Symbol(ref n) => env.get(n),
            Expression::Pair(_, _) => unimplemented!(),
            _ => Ok(value),
        }
    }
}
