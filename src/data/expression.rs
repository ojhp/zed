use std::fmt::{Display, Formatter, Result as FmtResult};
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::RegexSet;

use crate::data::Number;

/// An expression value.
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    /// The nil value.
    Nil,
    /// A boolean value, true or false.
    Boolean(bool),
    /// A numeric value represented as one of the four available
    /// types.
    Number(Number),
    /// A single character value.
    Character(char),
    /// A string value of multiple characters.
    String(String),
    /// A symbol value represented by a name.
    Symbol(String),
    /// A pair of expression values, can be used to build lists.
    Pair(Expr, Expr),
    /// A vector consisting of zero or more expression values.
    Vector(Vec<Expr>),
    /// A vector consisting of zero or more bytes.
    ByteVector(Vec<u8>),
}

/// A reference counted reference to an expression value.
pub type Expr = Rc<Expression>;

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        fn fmt_char(ch: &char, f: &mut Formatter) -> FmtResult {
            match ch {
                '\u{0007}' => write!(f, "#\\alarm"),
                '\u{0008}' => write!(f, "#\\backspace"),
                '\u{007f}' => write!(f, "#\\delete"),
                '\u{001b}' => write!(f, "#\\escape"),
                '\n' => write!(f, "#\\newline"),
                '\u{0000}' => write!(f, "#\\null"),
                '\r' => write!(f, "#\\return"),
                '\t' => write!(f, "#\\tab"),
                ' ' => write!(f, "#\\space"),
                _ if ch.is_alphanumeric() || ch.is_ascii_graphic() => write!(f, "#\\{}", ch),
                _ => write!(f, "#\\x{:x}", *ch as u32),
            }
        }

        fn fmt_string(str: &str, f: &mut Formatter) -> FmtResult {
            write!(f, "\"")?;

            for ch in str.chars() {
                match ch {
                    '\u{0007}' => write!(f, "\\a")?,
                    '\u{0008}' => write!(f, "\\b")?,
                    '\t' => write!(f, "\\t")?,
                    '\n' => write!(f, "\\n")?,
                    '\r' => write!(f, "\\r")?,
                    '"' => write!(f, "\\\"")?,
                    '\\' => write!(f, "\\\\")?,
                    '|' => write!(f, "\\|")?,
                    _ if ch.is_alphanumeric() || ch.is_ascii_graphic() || ch == ' ' => ch.fmt(f)?,
                    _ => write!(f, "\\x{:x};", ch as u32)?,
                }
            }

            write!(f, "\"")
        }

        fn fmt_symbol(name: &str, f: &mut Formatter) -> FmtResult {
            const SIMPLE_IDENTIFIER_PATTERN: &str =
                r"^[A-Za-z!$%&*/:<=>?^_~][-+A-Za-z0-9!$%&*/:<=>?^_~\.@]*$";
            const PECULIAR_IDENTIFIER_PATTERN: &str =
                r"^[-+]([-+A-Za-z!$%&*/:<=>?^_~@][-+A-Za-z0-9!$%&*/:<=>?^_~\.@]*)?$";
            const DOTTED_IDENTIFIER_PATTERN: &str =
                r"^[-+]?\.[-+A-Za-z!$%&*/:<=>?^_~@\.][-+A-Za-z0-9!$%&*/:<=>?^_~\.@]*$";

            lazy_static! {
                static ref UNQUOTED_IDENTIFIER_REGEX: RegexSet = RegexSet::new(&[
                    SIMPLE_IDENTIFIER_PATTERN,
                    PECULIAR_IDENTIFIER_PATTERN,
                    DOTTED_IDENTIFIER_PATTERN,
                ])
                .unwrap();
            }

            if UNQUOTED_IDENTIFIER_REGEX.is_match(name) {
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
                    '|' => write!(f, "\\|")?,
                    '\\' => write!(f, "\\\\")?,
                    _ if ch.is_alphanumeric() || ch.is_ascii_graphic() || ch == ' ' => ch.fmt(f)?,
                    _ => write!(f, "\\x{:x};", ch as u32)?,
                }
            }

            write!(f, "|")
        }

        fn fmt_list(head: &Expression, f: &mut Formatter) -> FmtResult {
            write!(f, "(")?;

            let mut current = head;
            let mut first = true;
            loop {
                match current {
                    Expression::Nil => break,
                    Expression::Pair(a, d) => {
                        if first {
                            first = false;
                        } else {
                            write!(f, " ")?;
                        }
                        a.fmt(f)?;
                        current = d;
                    }
                    _ => {
                        write!(f, " . {}", current)?;
                        break;
                    }
                }
            }

            write!(f, ")")
        }

        fn fmt_vector<T: Display>(prefix: &str, items: &[T], f: &mut Formatter) -> FmtResult {
            write!(f, "{}(", prefix)?;

            let mut first = true;
            for item in items {
                if first {
                    first = false;
                } else {
                    write!(f, " ")?;
                }
                item.fmt(f)?;
            }

            write!(f, ")")
        }

        match self {
            Expression::Nil => write!(f, "()"),
            Expression::Boolean(true) => write!(f, "#t"),
            Expression::Boolean(false) => write!(f, "#f"),
            Expression::Number(n) => n.fmt(f),
            Expression::Character(c) => fmt_char(c, f),
            Expression::String(s) => fmt_string(s, f),
            Expression::Symbol(n) => fmt_symbol(n, f),
            p @ Expression::Pair(_, _) => fmt_list(p, f),
            Expression::Vector(es) => fmt_vector("#", es, f),
            Expression::ByteVector(bs) => fmt_vector("#u8", bs, f),
        }
    }
}
