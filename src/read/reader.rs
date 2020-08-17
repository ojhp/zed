use std::io::BufRead;

use crate::data::Expr;
use crate::read::{parse, ReadError, ReadResult};

/// A trait for a component that reads expressions from a source.
pub trait Read {
    /// Reads the next expression from the source, returning an error
    /// if reading or parsing fails, or if the end of the input has
    /// been reached.
    fn read(&mut self) -> ReadResult<Expr>;
}

/// An implementation of the `Read` trait which takes input from a
/// buffered reader and buffers results.
///
/// # Example
/// ```
/// use std::io::BufReader;
/// use std::rc::Rc;
///
/// use zed::data::Expression;
/// use zed::read::{Read, Reader};
///
/// let input = "a (b . c)";
/// let mut reader = Reader::new(BufReader::new(input.as_bytes()));
///
/// assert_eq!(Expression::Symbol(String::from("a")), *reader.read().unwrap());
/// assert_eq!(Expression::Pair(
///         Rc::new(Expression::Symbol(String::from("b"))),
///         Rc::new(Expression::Symbol(String::from("c")))),
///     *reader.read().unwrap());
/// assert!(reader.read().is_err()); // EOF returned at end of input
/// ```
pub struct Reader<T: BufRead> {
    input: T,
    buffer: Vec<Expr>,
}

impl<T: BufRead> Reader<T> {
    pub fn new(input: T) -> Self {
        Reader {
            input,
            buffer: Vec::new(),
        }
    }
}

impl<T: BufRead> Read for Reader<T> {
    fn read(&mut self) -> ReadResult<Expr> {
        fn match_braces(text: &str) -> bool {
            text.chars()
                .filter_map(|c| match c {
                    '(' => Some(1),
                    ')' => Some(-1),
                    _ => None,
                })
                .sum::<i32>()
                <= 0
        }

        loop {
            if let Some(e) = self.buffer.pop() {
                return Ok(e);
            }

            let mut text = String::new();
            if self.input.read_line(&mut text)? == 0 {
                return Err(ReadError::Eof);
            }

            while !match_braces(&text) {
                if self.input.read_line(&mut text)? == 0 {
                    break;
                }
            }

            self.buffer = parse(&text)?.into_iter().rev().collect();
        }
    }
}
