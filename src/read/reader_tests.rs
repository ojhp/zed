#![cfg(test)]

use std::io::BufReader;

use test_case::test_case;

use crate::data::test_helpers::*;
use crate::data::Expr;
use crate::read::*;

#[test_case("a b c", vec![symbol("a"), symbol("b"), symbol("c")]; "single line data")]
#[test_case("a\nb\n c", vec![symbol("a"), symbol("b"), symbol("c")]; "multi-line data")]
#[test_case("a\n\nb\n", vec![symbol("a"), symbol("b")]; "empty lines")]
#[test_case("(+\na\nb\n)", vec![list(vec![symbol("+"), symbol("a"), symbol("b")])]; "multi-line expression")]
fn test_reader(input: &str, expected: Vec<Expr>) {
    let mut reader = Reader::new(BufReader::new(input.as_bytes()));

    for expr in &expected {
        match reader.read() {
            Ok(e) => assert_eq!(expr, &e),
            Err(err) => panic!("read error: {}", err),
        }
    }

    match reader.read() {
        Ok(e) => panic!("unexpected result: {}", e),
        Err(ReadError::Eof) => {}
        Err(err) => panic!("read error: {}", err),
    }
}
