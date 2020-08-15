#![cfg(test)]

use test_case::test_case;

use crate::data::*;
use crate::data::test_helpers::*;

#[test]
fn nil_formatting() {
    assert_eq!("()", nil().to_string());
}

#[test_case(true => "#t"; "r#true")]
#[test_case(false => "#f"; "r#false")]
fn boolean_formatting(value: bool) -> String {
    boolean(value).to_string()
}

#[test_case(integer("123456789") => "123456789"; "integer value")]
#[test_case(rational("1234", "4567") => "1234/4567"; "rational value")]
#[test_case(real(123.456) => "123.456"; "real value")]
#[test_case(complex(12.3, 4.56) => "12.3+4.56i"; "complex value")]
fn number_formatting(value: Number) -> String {
    number(value).to_string()
}

#[test_case('\u{0007}' => "#\\alarm"; "alarm")]
#[test_case('\u{0008}' => "#\\backspace"; "backspace")]
#[test_case('\u{007f}' => "#\\delete"; "delete")]
#[test_case('\u{001b}' => "#\\escape"; "escape")]
#[test_case('\u{000a}' => "#\\newline"; "newline")]
#[test_case('\u{0000}' => "#\\null"; "null")]
#[test_case('\u{000d}' => "#\\return"; "r#return")]
#[test_case('\u{0009}' => "#\\tab"; "tab")]
#[test_case(' ' => "#\\space"; "space")]
#[test_case('a' => "#\\a"; "lower-case a")]
#[test_case('A' => "#\\A"; "upper-case a")]
#[test_case('\u{03bb}' => "#\\λ"; "lambda")]
#[test_case('\u{0014}' => "#\\x14"; "short special_character")]
#[test_case('\u{2400}' => "#\\x2400"; "medium special character")]
#[test_case('\u{e0000}' => "#\\xe0000"; "long special character")]
fn character_formatting(ch: char) -> String {
    character(ch).to_string()
}

#[test_case("Hello World!" => "\"Hello World!\""; "simple text")]
#[test_case("Two lines\nof text" => "\"Two lines\\nof text\""; "two lines")]
#[test_case("\u{0007}\u{0008}\t\n\r\"\\|" => "\"\\a\\b\\t\\n\\r\\\"\\\\\\|\""; "special escapes")]
#[test_case("\u{03bb}\u{0014}\u{2400}\u{e0000}" => "\"λ\\x14;\\x2400;\\xe0000;\""; "long-form escapes")]
fn string_formatting(text: &str) -> String {
    string(text).to_string()
}

#[test_case("simple-ident!" => "simple-ident!"; "simple")]
#[test_case("-" => "-"; "sign-only")]
#[test_case("+@thing" => "+@thing"; "sign and subsequent")]
#[test_case("-.+" => "-.+"; "sign and dot")]
#[test_case("..+123" => "..+123"; "dot and subsequent")]
#[test_case("spaced ident" => "|spaced ident|"; "simple quoted")]
#[test_case("\u{0007}\u{0008}\t\n\r|\\" => "|\\a\\b\\t\\n\\r\\|\\\\|"; "simple escapes")]
#[test_case("\u{03bb}\u{0014}\u{2400}\u{e0000}" => "|λ\\x14;\\x2400;\\xe0000;|"; "complex escapes")]
fn symbol_formatting(name: &str) -> String {
    symbol(name).to_string()
}

#[test_case(boolean(true), symbol("a") => "(#t . a)"; "boolean and symbol")]
#[test_case(character('x'), number(real(4.5)) => "(#\\x . 4.5)"; "character and number")]
#[test_case(nil(), symbol("+") => "(() . +)"; "nil and symbol")]
#[test_case(symbol("+"), nil() => "(+)"; "single element list")]
fn pair_formatting(car: Expr, cdr: Expr) -> String {
    pair(car, cdr).to_string()
}

#[test_case(vec![symbol("a")] => "(a)"; "single element")]
#[test_case(vec![symbol("a"), symbol("b"), symbol("c")] => "(a b c)"; "multi-element")]
#[test_case(vec![nil(), pair(symbol("a"), symbol("b")), list(vec![symbol("c"), symbol("d")])] => "(() (a . b) (c d))"; "complex")]
fn list_formatting(items: Vec<Expr>) -> String {
    list(items).to_string()
}

#[test_case(vec![] => "#()"; "empty")]
#[test_case(vec![symbol("a"), symbol("b"), symbol("c")] => "#(a b c)"; "simple")]
#[test_case(vec![nil(), pair(symbol("a"), symbol("b")), vector(vec![symbol("c"), symbol("d")])] => "#(() (a . b) #(c d))"; "complex")]
fn vector_formatting(items: Vec<Expr>) -> String {
    vector(items).to_string()
}

#[test_case(vec![] => "#u8()"; "empty")]
#[test_case(vec![1, 2, 3, 4] => "#u8(1 2 3 4)"; "simple ascending")]
#[test_case(vec![98, 127, 34, 8] => "#u8(98 127 34 8)"; "mixed up")]
fn byte_vector_formatting(items: Vec<u8>) -> String {
    byte_vector(items).to_string()
}
