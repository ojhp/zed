use std::convert::TryFrom;

use peg::parser;

use crate::data::Expr;
use crate::read::ReadResult;

/// Reads a list of expression values from the given text.
/// 
/// If the parser encounters any text that cannot be read as an
/// expression, an error is returned. No partial parses are returned.
/// 
/// # Example
/// ```
/// use zed::data::{Expression, Number};
/// use zed::read::parse;
/// 
/// let values = parse("#f a-symbol 4.2").unwrap();
/// 
/// assert_eq!(Expression::Boolean(false), *values[0]);
/// assert_eq!(Expression::Symbol(String::from("a-symbol")), *values[1]);
/// assert_eq!(Expression::Number(Number::Real(4.2)), *values[2]);
/// ```
pub fn parse(text: &str) -> ReadResult<Vec<Expr>> {
    Ok(expr_parser::expr_list(text)?)
}

parser! {
    grammar expr_parser() for str {
        use std::rc::Rc;
        use std::str::FromStr;

        use num::{BigInt, BigRational, Complex, Num, ToPrimitive};

        use crate::data::{Expression, Number};

        pub rule expr_list() -> Vec<Expr>
            = _* es:expr()**(_*) _* { es }

        rule expr() -> Expr
            = b:boolean()       { Rc::new(Expression::Boolean(b)) }
            / n:number()        { Rc::new(Expression::Number(n)) }
            / c:character()     { Rc::new(Expression::Character(c)) }
            / s:string()        { Rc::new(Expression::String(s)) }
            / i:identifier()    { Rc::new(Expression::Symbol(i)) }
            / list()
            / v:vector()        { Rc::new(Expression::Vector(v)) }
            / b:byte_vector()   { Rc::new(Expression::ByteVector(b)) }

        rule boolean() -> bool
            = ("#true" / "#t")  { true }
            / ("#false" / "#f") { false }

        rule number() -> Number
            = c:complex()   { Number::Complex(c) }
            / r:real()      { Number::Real(r) }
            / r:rational()  { Number::Rational(r) }
            / i:integer()   { Number::Integer(i) }

        rule complex() -> Complex<f64>
            = i:creal() "i"                 { Complex::new(0.0, i) }
            / r:creal() "+" i:cureal() "i"  { Complex::new(r, i) }
            / r:creal() "-" i:cureal() "i"  { Complex::new(r, -i) }
            / r:creal() i:infnan() "i"      { Complex::new(r, i) }
            / r:creal() "+i"                { Complex::new(r, 1.0) }
            / r:creal() "-i"                { Complex::new(r, -1.0) }
            / "+i" !"nf.0"                  { Complex::new(0.0, 1.0) }
            / "-i" !"nf.0"                  { Complex::new(0.0, -1.0) }
        rule creal() -> f64
            = "+"? r:cureal()   { r }
            / "-" r:cureal()    { -r }
            / infnan()
        rule cureal() -> f64
            = s:$(((digit()* "." digit()+) / (digit()+ "."?)) exponent()?)
                {? f64::from_str(s).map_err(|_| "invalid real") }

        rule real() -> f64
            = "+"? r:ureal()    { r }
            / "-" r:ureal()     { -r }
            / infnan()
        rule infnan() -> f64
            = "+inf.0"  { f64::INFINITY }
            / "-inf.0"  { f64::NEG_INFINITY }
            / "+nan.0"  { f64::NAN }
            / "-nan.0"  { -f64::NAN }
        rule ureal() -> f64
            = s:$(((digit()* "." digit()+) / (digit()+ ".")) exponent()?)
                {? f64::from_str(s).map_err(|_| "invalid real") }
        rule exponent() = "e" ['-'|'+']? ['0'..='9']+

        rule rational() -> BigRational
            = "#b" n:bin_integer() "/" d:bin_integer()  { BigRational::new(n, d) }
            / "#o" n:oct_integer() "/" d:oct_integer()  { BigRational::new(n, d) }
            / "#x" n:hex_integer() "/" d:hex_integer()  { BigRational::new(n, d) }
            / "#d"? n:dec_integer() "/" d:dec_integer() { BigRational::new(n, d) }

        rule integer() -> BigInt
            = "#b" i:bin_integer()  { i }
            / "#o" i:oct_integer()  { i }
            / "#x" i:hex_integer()  { i }
            / "#d"? i:dec_integer() { i }
        rule bin_integer() -> BigInt
            = s:$(explicit_sign()? ['0'|'1']+)
                {? BigInt::from_str_radix(s, 2).map_err(|_| "invalid integer") }
        rule oct_integer() -> BigInt
            = s:$(explicit_sign()? ['0'..='7']+)
                {? BigInt::from_str_radix(s, 8).map_err(|_| "invalid integer") }
        rule dec_integer() -> BigInt
            = s:$(explicit_sign()? ['0'..='9']+)
                {? BigInt::from_str_radix(s, 10).map_err(|_| "invalid integer") }
        rule hex_integer() -> BigInt
            = s:$(explicit_sign()? ['0'..='9'|'a'..='f'|'A'..='F']+)
                {? BigInt::from_str_radix(s, 16).map_err(|_| "invalid integer") }

        rule character() -> char
            = "#\\alarm"        { '\u{0007}' }
            / "#\\backspace"    { '\u{0008}' }
            / "#\\delete"       { '\u{007f}' }
            / "#\\escape"       { '\u{001b}' }
            / "#\\newline"      { '\n' }
            / "#\\null"         { '\u{0000}' }
            / "#\\return"       { '\r' }
            / "#\\space"        { ' ' }
            / "#\\tab"          { '\t' }
            / "#\\x" e:$(['0'..='9'|'a'..='f'|'A'..='F']+)
                {? parse_hex_char(e) }
            / "#\\" c:$([_])    { c.chars().next().unwrap() }

        rule string() -> String
            = "\"" s:(string_element() / ignored_space())* "\""
                { s.into_iter().filter_map(|c| c).collect() }
        rule string_element() -> Option<char>
            = !['"'|'\\'] c:$([_])  { Some(c.chars().next().unwrap()) }
            / e:mnemonic_escape()   { Some(e) }
            / "\\\""                { Some('"') }
            / "\\\\"                { Some('\\') }
            / e:inline_hex_escape() { Some(e) }
        rule ignored_space() -> Option<char>
            = "\\" [' '|'\t']* ("\r\n"/"\r"/"\n") [' '|'\t']*   { None }

        rule identifier() -> String
            = s:$(initial() subsequent()*)  { String::from(s) }
            / "|" cs:symbol_element()* "|"  { cs.iter().collect() }
            / s:$(peculiar_identifier())    { String::from(s) }
        rule initial() = letter() / special_initial()
        rule letter() = ['a'..='z'|'A'..='Z']
        rule special_initial() = ['!'|'$'|'%'|'&'|'*'|'/'|':'|'<'|'='|'>'|'?'|'^'|'_'|'~']
        rule subsequent() = initial() / digit() / special_subsequent()
        rule digit() = ['0'..='9']
        rule special_subsequent() = explicit_sign() / ['.'|'@']
        rule explicit_sign() = ['+'|'-']
        rule peculiar_identifier()
            = explicit_sign()? "." dot_subsequent() subsequent()*
            / explicit_sign() (sign_subsequent() subsequent()*)?
        rule dot_subsequent() = sign_subsequent() / "."
        rule sign_subsequent() = initial() / explicit_sign() / "@"
        rule symbol_element() -> char
            = !['|'|'\\'] c:$([_])  { c.chars().next().unwrap() }
            / inline_hex_escape()
            / mnemonic_escape()
            / "\\|" { '|' }
        rule inline_hex_escape() -> char
            = "\\x" c:$(['0'..='9'|'a'..='f'|'A'..='F']+) ";" {? parse_hex_char(c) }
        rule mnemonic_escape() -> char
            = "\\a"     { '\u{0007}' }
            / "\\b"     { '\u{0008}' }
            / "\\t"     { '\t' }
            / "\\n"     { '\n' }
            / "\\r"     { '\r' }
            / "\\\\"    { '\\' }

        rule list() -> Expr
            = "(" _* l:inlist() { l }
        rule inlist() -> Expr
            = ")"                                   { Rc::new(Expression::Nil) }
            / e:expr() _* l:inlist()                { Rc::new(Expression::Pair(e, l)) }
            / e1:expr() _* "." _* e2:expr() _* ")"  { Rc::new(Expression::Pair(e1, e2)) }

        rule vector() -> Vec<Expr>
            = "#(" _* vs:expr()**(_*) _* ")"    { vs }

        rule byte_vector() -> Vec<u8>
            = "#u8(" _* bs:byte()**(_*) _* ")"  { bs }
        rule byte() -> u8
            = i:integer()   {? i.to_u8().map_or(Err("invalid byte"), Ok) }

        rule _() = whitespace() / comment()
        rule whitespace() = [' '|'\t'|'\r'|'\n']
        rule comment()
            = ";" (!['\r'|'\n'] [_])* ("\r\n" / "\r" / "\n")
            / "#|" (!['|'] [_] / "|" !['#'] [_])* "|#"
            / "#;" _* expr()
    }
}

fn parse_hex_char(escape: &str) -> Result<char, &'static str> {
    if let Ok(n) = u32::from_str_radix(escape, 16) {
        if let Ok(ch) = char::try_from(n) {
            return Ok(ch)
        }
    }

    Err("invalid hex escape")
}
