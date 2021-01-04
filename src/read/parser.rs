use std::convert::TryFrom;
use std::rc::Rc;

use num::{BigInt, Num};

use crate::data::{Expr, Expression, Number};
use crate::read::ReadResult;

pub (crate) fn parse_exprs(text: &str) -> ReadResult<Vec<Expr>> {
    Ok(zed::exprs(text)?)
}

pub (crate) fn is_simple_identifier(text: &str) -> bool {
    zed::simple_identifier(text).is_ok()
}

peg::parser! {
    grammar zed() for str {
        pub rule exprs() -> Vec<Expr>
            = _* es:expr()**(_*) _*     { es }
        pub rule simple_identifier() = initial() subsequent()* / peculiar_identifier()

        rule expr() -> Expr
            = n:number()        { Rc::new(Expression::Number(n)) }
            / i:identifier()    { Rc::new(Expression::Symbol(i)) }
            / list()

        rule number() -> Number
            = r:real()      { Number::Real(r) }
            / i:integer()   { Number::Integer(i) }
        rule integer() -> BigInt
            = "#b" s:$(explicit_sign()? ['0'|'1']+)     {? parse_number(s, 2, "invalid integer") }
            / "#o" s:$(explicit_sign()? ['0'..='7']+)   {? parse_number(s, 8, "invalid integer") }
            / "#x" s:$(explicit_sign()? hex_digit()+)   {? parse_number(s, 16, "invalid integer") }
            / "#d"? s:$(explicit_sign()? digit()+)      {? parse_number(s, 10, "invalid integer") }
        rule real() -> f64
            = s:$(explicit_sign()? digit()+ exponent())                 {? parse_number(s, 10, "invalid real") }
            / s:$(explicit_sign()? digit()+ "." digit()* exponent()?)   {? parse_number(s, 10, "invalid real") }
            / s:$(explicit_sign()? "." digit()+ exponent()?)            {? parse_number(s, 10, "invalid real") }
        rule exponent() = "e" explicit_sign()? digit()+

        rule identifier() -> String
            = s:$(initial() subsequent()*)      { s.to_string() }
            / "|" cs:symbol_element()* "|"      { cs.into_iter().collect() }
            / s:$(peculiar_identifier())        { s.to_string() }
        rule initial() = letter() / special_initial()
        rule letter() = ['a'..='z'|'A'..='Z']
        rule special_initial() = ['!'|'$'|'%'|'&'|'*'|'/'|':'|'<'|'='|'>'|'?'|'^'|'_'|'~']
        rule subsequent() = initial() / digit() / special_subsequent()
        rule digit() = ['0'..='9']
        rule hex_digit() = digit() / ['a'..='f'|'A'..='F']
        rule explicit_sign() = ['+'|'-']
        rule special_subsequent() = explicit_sign() / ['.'|'@']
        rule inline_hex_escape() -> char
            = "\\x" s:$(hex_digit()+) ";"       {? parse_hex_escape(s) }
        rule mnemonic_escape() -> char
            = "\\a"     { '\u{0007}' }
            / "\\b"     { '\u{0008}' }
            / "\\t"     { '\t' }
            / "\\n"     { '\n' }
            / "\\r"     { '\r' }
        rule peculiar_identifier()
            = explicit_sign() (sign_subsequent() subsequent()*)?
            / explicit_sign() "." dot_subsequent() subsequent()*
            / "." dot_subsequent() subsequent()*
        rule dot_subsequent() = sign_subsequent() / "."
        rule sign_subsequent() = initial() / explicit_sign() / "@"
        rule symbol_element() -> char
            = !['\\'|'|'] c:$([_])      { c.chars().next().unwrap() }
            / inline_hex_escape()
            / mnemonic_escape()
            / "\\|"                     { '|' }

        rule list() -> Expr
            = "(" _* l:list_internal()      { l }
        rule list_internal() -> Expr
            = ")"                                   { Rc::new(Expression::Nil) }
            / e1:expr() _* "." _* e2:expr() _* ")"  { Rc::new(Expression::Pair(e1, e2)) }
            / e:expr() _* l:list_internal()         { Rc::new(Expression::Pair(e, l)) }

        rule _()
            = [' '|'\t'|'\r'|'\n']
            / ";" (!['\r'|'\n'] [_])* (['\r'|'\n'] / ![_])
    }
}

fn parse_number<T: Num>(text: &str, radix: u32, msg: &'static str) -> Result<T, &'static str> {
    T::from_str_radix(text, radix).map_err(|_| msg)
}

fn parse_hex_escape(escape: &str) -> Result<char, &'static str> {
    if let Ok(n) = u32::from_str_radix(escape, 16) {
        if let Ok(ch) = char::try_from(n) {
            return Ok(ch);
        }
    }

    Err("invalid escape sequence")
}
