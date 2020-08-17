#![cfg(test)]

use crate::data::test_helpers::*;
use crate::eval::*;

#[test]
fn defining_and_retrieving_a_variable() {
    let env = Environment::empty();
    env.define("a", &character('x'));

    assert_eq!(character('x'), env.get("a").unwrap());
}

#[test]
fn defining_and_updating_a_variable() {
    let env = Environment::empty();
    env.define("a", &character('f'));
    env.set("a", &character('g')).unwrap();

    assert_eq!(character('g'), env.get("a").unwrap());
}

#[test]
fn getting_an_unbound_variable() {
    let env = Environment::empty();

    assert_eq!(
        "unbound variable: `a`",
        env.get("a").unwrap_err().to_string()
    );
}

#[test]
fn setting_an_unbound_variable() {
    let env = Environment::empty();

    assert_eq!(
        "unbound variable: `a`",
        env.set("a", &character('x')).unwrap_err().to_string()
    );
}

#[test]
fn hiding_in_inner_environment() {
    let outer = Environment::empty();
    let inner = Environment::inner(&outer);

    outer.define("a", &number(real(1.0)));
    outer.define("b", &number(real(2.0)));
    inner.define("a", &number(real(3.0)));

    assert_eq!(number(real(1.0)), outer.get("a").unwrap());
    assert_eq!(number(real(2.0)), outer.get("b").unwrap());
    assert_eq!(number(real(3.0)), inner.get("a").unwrap());
    assert_eq!(number(real(2.0)), inner.get("b").unwrap());
}

#[test]
fn setting_from_outer_environment() {
    let outer = Environment::empty();
    let inner = Environment::inner(&outer);

    outer.define("a", &number(real(1.0)));
    outer.define("b", &number(real(2.0)));
    inner.define("a", &number(real(3.0)));

    inner.set("a", &number(real(4.0))).unwrap();
    inner.set("b", &number(real(5.0))).unwrap();

    assert_eq!(number(real(1.0)), outer.get("a").unwrap());
    assert_eq!(number(real(5.0)), outer.get("b").unwrap());
    assert_eq!(number(real(4.0)), inner.get("a").unwrap());
    assert_eq!(number(real(5.0)), inner.get("b").unwrap());
}
