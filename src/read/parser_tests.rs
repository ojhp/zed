#![cfg(test)]

use test_case::test_case;

use crate::data::test_helpers::*;
use crate::data::*;
use crate::read::*;

#[test_case("#t" => boolean(true); "short true")]
#[test_case("#f" => boolean(false); "short false")]
#[test_case("#true" => boolean(true); "long true")]
#[test_case("#false" => boolean(false); "long false")]
fn boolean_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("123456789" => number(integer("123456789")); "positive decimal")]
#[test_case("#d-98765" => number(integer("-98765")); "negative decimal")]
#[test_case("#b10101" => number(integer("21")); "positive binary")]
#[test_case("#b-11101" => number(integer("-29")); "negative binary")]
#[test_case("#o73523" => number(integer("30547")); "positive octal")]
#[test_case("#o-7654321" => number(integer("-2054353")); "negative octal")]
#[test_case("#xaf07e" => number(integer("716926")); "positive hexadecimal")]
#[test_case("#x-23d5c" => number(integer("-146780")); "negative hexadecimal")]
fn integer_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("12/34" => number(rational("12", "34")); "decimal")]
#[test_case("#d-56/78" => number(rational("-56", "78")); "specified decimal")]
#[test_case("#b110/-101" => number(rational("6", "-5")); "binary")]
#[test_case("#o741/356" => number(rational("481", "238")); "octal")]
#[test_case("#x-8fe/-3e" => number(rational("2302", "62")); "hexadecimal")]
fn rational_parsein(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("12.34" => number(real(12.34)); "simple")]
#[test_case("12." => number(real(12.0)); "integral real")]
#[test_case(".34" => number(real(0.34)); "fraction")]
#[test_case("+3.5" => number(real(3.5)); "positive sign")]
#[test_case("-7.8" => number(real(-7.8)); "negative sign")]
#[test_case("+inf.0" => number(real(f64::INFINITY)); "positive infinity")]
#[test_case("-inf.0" => number(real(f64::NEG_INFINITY)); "negative infinity")]
fn real_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("+nan.0", true; "positive")]
#[test_case("-nan.0", false; "negative")]
fn real_infinity_parsing(input: &str, positive: bool) {
    let value = parse_single(input);

    if let Expression::Number(Number::Real(n)) = *value {
        assert!(n.is_nan());

        if positive {
            assert!(n.is_sign_positive());
            assert!(!n.is_sign_negative());
        } else {
            assert!(!n.is_sign_positive());
            assert!(n.is_sign_negative());
        }
    }
}

#[test_case("3i" => number(complex(0.0, 3.0)); "simple imaginary")]
#[test_case("4.5+8i" => number(complex(4.5, 8.0)); "simple complex")]
#[test_case("-3-1i" => number(complex(-3.0, -1.0)); "negative complex")]
#[test_case("-2+i" => number(complex(-2.0, 1.0)); "real and unit imaginary")]
#[test_case("4.8-i" => number(complex(4.8, -1.0)); "real and negative unit imaginary")]
#[test_case("0.5+inf.0i" => number(complex(0.5, f64::INFINITY)); "real and infinite imaginary")]
#[test_case("+inf.0-inf.0i" => number(complex(f64::INFINITY, f64::NEG_INFINITY)); "infinite complex")]
#[test_case("+i" => number(complex(0.0, 1.0)); "unit imaginary")]
#[test_case("-i" => number(complex(0.0, -1.0)); "negative unit imaginary")]
#[test_case("4@0" => number(complex(4.0, 0.0)); "polar")]
fn complex_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("#\\alarm" => character('\u{0007}'); "alarm")]
#[test_case("#\\backspace" => character('\u{0008}'); "backspace")]
#[test_case("#\\delete" => character('\u{007f}'); "delete")]
#[test_case("#\\escape" => character('\u{001b}'); "escape")]
#[test_case("#\\newline" => character('\n'); "newline")]
#[test_case("#\\null" => character('\u{0000}'); "null")]
#[test_case("#\\return" => character('\r'); "r#return")]
#[test_case("#\\space" => character(' '); "space")]
#[test_case("#\\tab" => character('\t'); "tab")]
#[test_case("#\\x2400" => character('\u{2400}'); "hex escape")]
#[test_case("#\\ " => character(' '); "literal space")]
#[test_case("#\\x" => character('x'); "simple character")]
#[test_case("#\\λ" => character('λ'); "extended character")]
fn character_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("\"\"" => string(""); "empty string")]
#[test_case("\"normal text\"" => string("normal text"); "normal text")]
#[test_case("\"\\a\\b\\t\\r\\n\\\\\\\"\"" => string("\u{0007}\u{0008}\t\r\n\\\""); "mnemonic escapes")]
#[test_case("\"\\x14;\\x2400;\\xe0000;\"" => string("\u{0014}\u{2400}\u{e0000}"); "hex escapes")]
#[test_case("\"first\\ \t\r\n \t second\"" => string("firstsecond"); "ignored spaces")]
fn string_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("test" => symbol("test"); "simple text")]
#[test_case("!do-thing1" => symbol("!do-thing1"); "symbols mixed in")]
#[test_case("+" => symbol("+"); "simple sign")]
#[test_case("-@thing" => symbol("-@thing"); "extended sign")]
#[test_case("+..23" => symbol("+..23"); "sign and dot")]
#[test_case(".@test" => symbol(".@test"); "dot symbol")]
#[test_case("||" => symbol(""); "empty quoted")]
#[test_case("|something quoted|" => symbol("something quoted"); "simple quoted")]
#[test_case("|\\a\\b\\t\\r\\n\\\\\\||" => symbol("\u{0007}\u{0008}\t\r\n\\|"); "mnemonic escapes")]
#[test_case("|\\x14;\\x2400;\\xe0000;|" => symbol("\u{0014}\u{2400}\u{e0000}"); "hex escapes")]
fn symbol_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("()" => nil(); "empty list")]
#[test_case("( )" => nil(); "space-only list")]
#[test_case("(+ 3 4)" => list(vec![symbol("+"), number(integer("3")), number(integer("4"))]); "simple list")]
#[test_case("(a . b)" => pair(symbol("a"), symbol("b")); "simple pair")]
#[test_case("(a () (b c) . ())" => list(vec![symbol("a"), nil(), list(vec![symbol("b"), symbol("c")])]); "complex list")]
fn list_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("#()" => vector(vec![]); "empty vector")]
#[test_case("#(\t)" => vector(vec![]); "space-only vector")]
#[test_case("#( a b  c)" => vector(vec![symbol("a"), symbol("b"), symbol("c")]); "simple vector")]
fn vector_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("#u8()" => byte_vector(vec![]); "empty byte vector")]
#[test_case("#u8( )" => byte_vector(vec![]); "space-only byte vector")]
#[test_case("#u8(1 2 3)" => byte_vector(vec![1, 2, 3]); "simple byte vector")]
#[test_case("#u8(#xf #o12 #d9)" => byte_vector(vec![15, 10, 9]); "complex bytes")]
fn byte_vector_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("'a" => list(vec![symbol("quote"), symbol("a")]); "quoted form")]
#[test_case("`a" => list(vec![symbol("quasiquote"), symbol("a")]); "quasiquoted form")]
#[test_case(",a" => list(vec![symbol("unquote"), symbol("a")]); "unquoted form")]
#[test_case(",@a" => list(vec![symbol("unquote-splicing"), symbol("a")]); "spliced unquote form")]
fn wrapped_form_parsing(input: &str) -> Expr {
    parse_single(input)
}

#[test_case("a ;line comment\n b" => vec![symbol("a"), symbol("b")]; "line comment")]
#[test_case("a #| block | \r\n #| comment |#b" => vec![symbol("a"), symbol("b")]; "block comment")]
#[test_case("a #; \n b c" => vec![symbol("a"), symbol("c")]; "single item comment")]
fn comment_parsing(input: &str) -> Vec<Expr> {
    parse_multiple(input)
}

fn parse_single(input: &str) -> Expr {
    match parse(input) {
        Ok(result) if result.len() < 1 => panic!("parsed no values"),
        Ok(result) if result.len() > 1 => panic!("parsed more than one value: {:?}", result),
        Err(err) => panic!("parser error: {}", err),
        Ok(mut result) => result.pop().unwrap(),
    }
}

fn parse_multiple(input: &str) -> Vec<Expr> {
    match parse(input) {
        Ok(result) => result,
        Err(err) => panic!("parser error: {}", err),
    }
}
