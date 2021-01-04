#![cfg(test)]

use test_case::test_case;

use crate::data::Expr;
use crate::data::test_helpers::*;
use crate::read::{ExprReader, Read, ReadError, ReadResult};

#[test_case("()" => nil(); "nil value")]
#[test_case("45" => number(integer(45)); "positive integer")]
#[test_case("-9876" => number(integer(-9876)); "negative integer")]
#[test_case("4.56" => number(real(4.56)); "positive real")]
#[test_case("-87.0" => number(real(-87.0)); "negative real")]
#[test_case("4e-2" => number(real(0.04)); "exponent real")]
#[test_case("2." => number(real(2.0)); "left-handed real")]
#[test_case(".92" => number(real(0.92)); "right-handed real")]
#[test_case("test-sym" => symbol("test-sym"); "basic symbol")]
#[test_case("|quoted symbol|" => symbol("quoted symbol"); "quoted symbol")]
#[test_case("|escaped\\nsymbol\\x45;|" => symbol("escaped\nsymbol\u{45}"); "escaped symbol")]
#[test_case("+two" => symbol("+two"); "sign symbol")]
#[test_case(".dotted" => symbol(".dotted"); "dot symbol")]
#[test_case("(a . b)" => pair(symbol("a"), symbol("b")); "pair value")]
#[test_case("(a b . c)" => pair(symbol("a"), pair(symbol("b"), symbol("c"))); "dotted list")]
#[test_case("(a b c)" => list!(symbol("a"), symbol("b"), symbol("c")); "normal list")]
#[test_case("(a (b) . ())" => list!(symbol("a"), list!(symbol("b"))); "complex list")]
fn parse_single_expr(text: &str) -> Expr {
    let input = MockTextReader::new(text);
    let mut reader = ExprReader::new(input);

    match reader.read() {
        Ok(e) => return e,
        Err(e) => panic!("{}", e),
    }
}

#[test_case("(+ 2 3) equals 5" => vec![
    list!(symbol("+"), number(integer(2)), number(integer(3))),
    symbol("equals"),
    number(integer(5)),
]; "multiple values")]
#[test_case("a b ;c d \n e f" => vec![
    symbol("a"),
    symbol("b"),
    symbol("e"),
    symbol("f"),
]; "values with comment")]
fn parse_multiple_expr(text: &str) -> Vec<Expr> {
    let input = MockTextReader::new(text);
    let mut reader = ExprReader::new(input);

    let mut items = Vec::new();
    loop {
        match reader.read() {
            Ok(e) => items.push(e),
            Err(ReadError::Eof) => return items,
            Err(e) => panic!("{}", e),
        }
    }
}

struct MockTextReader {
    text: Option<String>,
}

impl MockTextReader {
    pub fn new<T: ToString>(text: T) -> Self {
        MockTextReader {
            text: Some(text.to_string()),
        }
    }
}

impl Read for MockTextReader {
    type Output = String;

    fn read(&mut self) -> ReadResult<String> {
        if let Some(text) = self.text.take() {
            Ok(text)
        } else {
            Err(ReadError::Eof)
        }
    }
}
