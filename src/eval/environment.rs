use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::data::Expr;
use crate::eval::{EvalError, EvalResult};

/// An evaluation environment mapping variable names to values.
///
/// An environment may have an outer environment which is checked
/// after this environment in case of a particular variable not being
/// defined here.
///
/// # Example
/// ```
/// use std::rc::Rc;
///
/// use zed::data::Expression;
/// use zed::eval::Environment;
///
/// // Declare some sample values
/// let value1 = Rc::new(Expression::Character('1'));
/// let value2 = Rc::new(Expression::Character('2'));
///
/// // Create an outer environment mapping `a` to '1' and an inner
/// // environment wrapping it.
/// let outer = Environment::empty();
/// outer.define("a", &value1);
/// let inner = Environment::inner(&outer);
///
/// // Now both environments return '1' for the name `a`.
/// assert_eq!(value1, outer.get("a").unwrap());
/// assert_eq!(value1, inner.get("a").unwrap());
///
/// // Redefine the variable `a` in the inner environment, giving
/// // it the value '2'.
/// inner.define("a", &value2);
///
/// // Now the inner environment returns its new definition, while
/// // the outer environment continues to return its original value.
/// assert_eq!(value1, outer.get("a").unwrap());
/// assert_eq!(value2, inner.get("a").unwrap());
/// ```
pub struct Environment {
    data: RefCell<HashMap<String, Expr>>,
    outer: Option<Env>,
}

/// A counted reference to an evaluation environment.
pub type Env = Rc<Environment>;

impl Environment {
    /// Creates a new empty environment with no variable definitions
    /// and no outer environment.
    pub fn empty() -> Env {
        Rc::new(Environment {
            data: RefCell::new(HashMap::new()),
            outer: None,
        })
    }

    /// Creates a new environment with no variable definitions, but
    /// with a reference to an outer environment.
    pub fn inner(outer: &Env) -> Env {
        Rc::new(Environment {
            data: RefCell::new(HashMap::new()),
            outer: Some(outer.clone()),
        })
    }

    /// Defines a new variable in this environment, mapping the given
    /// name to the given value.
    pub fn define(&self, name: &str, value: &Expr) {
        let mut data = self.data.borrow_mut();
        if let Some(v) = data.get_mut(name) {
            *v = value.clone();
        } else {
            data.insert(String::from(name), value.clone());
        }
    }

    /// Sets the value of an existing variable to the given value. If
    /// the variable does not exist in this environment, the outer
    /// environment is checked. If there is no outer environment, or
    /// that also does not contain the variable, an unbound variable
    /// error is returned.
    pub fn set(&self, name: &str, value: &Expr) -> EvalResult<()> {
        if let Some(v) = self.data.borrow_mut().get_mut(name) {
            *v = value.clone();
            Ok(())
        } else if let Some(o) = &self.outer {
            o.set(name, value)
        } else {
            Err(EvalError::UnboundVariable(String::from(name)))
        }
    }

    /// Gets a the value of a variable by name. If the variable does
    /// not exist in this environment, the outer environment is checked.
    /// If there is no outer environment, or that also does not contain
    /// the variable, an unbound error is returned.
    pub fn get(&self, name: &str) -> EvalResult<Expr> {
        if let Some(v) = self.data.borrow().get(name) {
            Ok(v.clone())
        } else if let Some(o) = &self.outer {
            o.get(name)
        } else {
            Err(EvalError::UnboundVariable(String::from(name)))
        }
    }
}
