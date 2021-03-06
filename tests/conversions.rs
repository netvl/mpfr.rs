extern crate mpfr;

use mpfr::BigFloat;
use mpfr::traits::*;
use mpfr::format::{FormatOptions, Format};

fn format(prec: u32, x: &BigFloat) -> String {
    FormatOptions::new(Format::Fixed).with_precision(prec.digits()).format(x)
}

#[test]
fn test_primitives() {
    BigFloat::set_default_prec(16.bits());

    let x = 1234i32.to_big_float();
    assert_eq!(16, x.prec().bits());
    assert_eq!("1234.0", &format(1, &x)[]);

    let x = 1234i32.to_big_float_with_prec(32.bits());
    assert_eq!(32, x.prec().bits());
    assert_eq!("1234.0", &format(1, &x)[]);
}

#[test]
fn test_string() {
    BigFloat::set_default_prec(16.bits());

    let x = "1234.56".to_big_float();
    assert_eq!(16, x.prec().bits());
    assert_eq!("1234.56", &format(2, &x)[]);

    let x = "1234.56".to_big_float_with_prec(32.bits());
    assert_eq!(32, x.prec().bits());
    assert_eq!("1234.56", &format(2, &x)[]);
}

#[test]
fn test_string_with_base() {
    BigFloat::set_default_prec(32.bits());

    let x = ("abcd.ef", 16us).to_big_float();
    assert_eq!(32, x.prec().bits());
    assert_eq!("43981.93", &format(2, &x)[]);

    let x = ("abcd.ef", 16us).to_big_float_with_prec(64.bits());
    assert_eq!(64, x.prec().bits());
    assert_eq!("43981.93", &format(2, &x)[]);
}
