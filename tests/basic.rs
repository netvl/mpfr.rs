#![feature(globs)]

extern crate mpfr;

use mpfr::BigFloat;

#[test]
fn test_basic_conversions() {
    let f = BigFloat::new().from::<f64>(12345.67);
    let r = f.get::<f64>();
    assert_eq!(12345.67, r);

    let f = BigFloat::new().from::<i64>(-123456);
    let r = f.get::<i64>();
    assert_eq!(-123456, r);
}
