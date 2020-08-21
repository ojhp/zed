use std::rc::Rc;

use crate::data::{Expr, Expression};
use crate::eval::{Env, EvalError, EvalResult};

/// The trait for a type that can evaluate expressions.
pub trait Eval {
    /// Evaluate a expression within the given environment returning
    /// the resulting value or an error if the evaluation fails.
    fn eval(&self, expr: &Expr, env: &Env) -> EvalResult<Expr>;
}

/// The default implementation of an expression evaluator.
///
/// # Symbols
/// Symbols are looked up in the environment and the value assigned
/// is returned. If there is no variable definition for the symbol's
/// name, an unbound variable error is returned.
///
/// # Lists
/// Lists are treated as function calls. The initial element is
/// evaluated and if it is a function, it is applied to the rest
/// of the elements of the list. An error is returned if the first
/// element does not evaluate to a function, or if the list is
/// not well formed (does not end with a Nil).
///
/// # Other types
/// All other types evaluate to themselves and are returned as is.
pub struct Evaluator;

impl Evaluator {
    fn unpack_args(&self, head: &Expr, tail: &Expr) -> EvalResult<Vec<Expr>> {
        let mut args = Vec::new();

        let mut current = tail;
        loop {
            match current.as_ref() {
                Expression::Nil => return Ok(args),
                Expression::Pair(a, d) => {
                    args.push(a.clone());
                    current = d;
                }
                _ => {
                    return Err(EvalError::BadlyFormedCall(Rc::new(Expression::Pair(
                        head.clone(),
                        tail.clone(),
                    ))))
                }
            }
        }
    }
}

impl Eval for Evaluator {
    fn eval(&self, expr: &Expr, env: &Env) -> EvalResult<Expr> {
        match expr.as_ref() {
            Expression::Symbol(n) => env.get(n),
            Expression::Pair(a, d) => {
                let func = self.eval(a, env)?;
                let args = self.unpack_args(&a, &d)?;

                if let Expression::Function(f) = func.as_ref() {
                    f.apply(args, env, self)
                } else {
                    Err(EvalError::NotAFunction(func))
                }
            }
            _ => Ok(expr.clone()),
        }
    }
}
