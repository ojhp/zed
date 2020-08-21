use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::data::Expr;
use crate::eval::{Env, Eval, EvalError, EvalResult};

#[derive(Clone, Debug, PartialEq)]
/// An enumeration of different types of function and syntax element
/// that may be applied to arguments in a call form.
pub enum Function {
    /// A primitive function that takes a set of evaluated arguments
    /// and returns a result or error.
    Primitive(String, ArgBounds, PrimitiveFn),
}

/// The type of the implementation of a primitive function.
pub type PrimitiveFn = fn(Vec<Expr>) -> Result<Expr, Box<dyn Error>>;

impl Function {
    /// Applies the function to a set of arguments in the given environment
    /// and using an eval implementation if required.
    ///
    /// # Primitive Function
    /// For primitive functions, the arguments are all evaluated by the
    /// eval implementation with the environment. Then some validation is
    /// performed to check that the number of arguments is within the bounds
    /// defined by the function. Finally, if those both succeeed, the
    /// arguments are passed to the function implementation and the result
    /// is returned.
    pub fn apply(&self, args: Vec<Expr>, env: &Env, eval: &dyn Eval) -> EvalResult<Expr> {
        match self {
            Function::Primitive(n, a, f) => {
                let args = self.eval_args(args, env, eval)?;
                a.validate(n, &args)?;
                f(args).map_err(|e| EvalError::FunctionError(n.clone(), e))
            }
        }
    }

    fn eval_args(&self, args: Vec<Expr>, env: &Env, eval: &dyn Eval) -> EvalResult<Vec<Expr>> {
        let mut results = Vec::with_capacity(args.len());

        for arg in args {
            results.push(eval.eval(&arg, env)?);
        }

        Ok(results)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Function::Primitive(n, _, _) => write!(f, "#<Function {}>", n),
        }
    }
}

/// A set of bounds for the number of arguments that may be passed
/// to a function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArgBounds {
    /// The minimum number of arguments allowed.
    pub min: Option<usize>,
    /// The maximum number of arguments allowed.
    pub max: Option<usize>,
}

impl ArgBounds {
    /// Validate an argument list against this set of bounds for a
    /// function with the given name.
    ///
    /// If a minimum is defined and the number of arguments is less
    /// than it, or a maximum is defined and the number of arguments
    /// is greater than it, an error is returned. Otherwise the
    /// method returns a successful result.
    pub fn validate(&self, name: &str, args: &[Expr]) -> EvalResult<()> {
        match (self.min, self.max) {
            (Some(m), _) if args.len() < m => Err(EvalError::NotEnoughArguments(
                String::from(name),
                m,
                args.len(),
            )),
            (_, Some(m)) if args.len() > m => Err(EvalError::TooManyArguments(
                String::from(name),
                m,
                args.len(),
            )),
            _ => Ok(()),
        }
    }
}
