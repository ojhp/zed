use std::fmt::{Display, Formatter, Result as FmtResult};
use std::rc::Rc;

use crate::data::Number;
use crate::read::is_simple_identifier;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Nil,
    Number(Number),
    Symbol(String),
    Pair(Expr, Expr),
}

pub type Expr = Rc<Expression>;

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Expression::*;

        fn fmt_symbol(name: &str, f: &mut Formatter) -> FmtResult {
            if is_simple_identifier(name) {
                return name.fmt(f);
            }

            write!(f, "|")?;

            for ch in name.chars() {
                match ch {
                    '\u{0007}' => write!(f, "\\a")?,
                    '\u{0008}' => write!(f, "\\b")?,
                    '\t' => write!(f, "\\t")?,
                    '\n' => write!(f, "\\n")?,
                    '\r' => write!(f, "\\r")?,
                    _ if ch.is_alphanumeric() || ch.is_ascii_graphic() || ch == ' ' => ch.fmt(f)?,
                    _ => write!(f, "\\x{:x};", ch as u32)?,
                }
            }

            write!(f, "|")
        }

        fn fmt_list(head: &Expression, mut tail: &Expression, f: &mut Formatter) -> FmtResult {
            write!(f, "({}", head)?;

            loop {
                match tail {
                    Nil => break,
                    Pair(h, t) => {
                        write!(f, " {}", h)?;
                        tail = t;
                    },
                    _ => {
                        write!(f, " . {}", tail)?;
                        break;
                    },
                }
            }

            write!(f, ")")
        }

        match self {
            Nil => write!(f, "()"),
            Number(n) => n.fmt(f),
            Symbol(n) => fmt_symbol(n, f),
            Pair(h, t) => fmt_list(h, t, f),
        }
    }
}
