use std::rc::Rc;

use crate::data::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Nil,
    Number(Number),
    Symbol(String),
    Pair(Expr, Expr),
}

pub type Expr = Rc<Expression>;
