use crate::data::Expr;
use crate::eval::{Eval, EvalResult};

pub struct ExprEvaluator;

impl Eval<Expr> for ExprEvaluator {
    type Output = Expr;

    fn eval(&mut self, value: Expr) -> EvalResult<Expr> {
        Ok(value)
    }
}
