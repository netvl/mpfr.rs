extern crate mpfr;

use mpfr::BigFloat;
use mpfr::traits::*;
use mpfr::format::{flags, FormatOptions, Format};

#[test]
fn test_basic_conversions() {
    let f = BigFloat::new().from::<f64>(12345.67);
    let r = f.get::<f64>();
    assert_eq!(12345.67, r);

    let f = BigFloat::new().from::<i64>(-123456);
    let r = f.get::<i64>();
    assert_eq!(-123456, r);
}

#[test]
fn test_format() {
    let f = BigFloat::new().from::<f64>(12345.67);
    let r = FormatOptions::new(Format::Fixed)
        .with_flags(flags::SIGN | flags::ZERO_PADDED)
        .with_width(12)
        .with_precision(3.digits())
        .format(&f);
    assert_eq!("+0012345.670", &r[]);
}

#[test]
fn test_addition() {
    let x = BigFloat::new().from(12345f64);
    let y = BigFloat::new().from(54322u16);
    let z = x + y;
    let zz = BigFloat::new().from(66667i32);
    assert_eq!(z, zz);

    let x = BigFloat::new().from(12345f64);
    let z = x + 54322u32;
    let zz = BigFloat::new().from(66667i32);
    assert_eq!(z, zz);
}

#[test]
fn test_subtraction() {
    let x = BigFloat::new().from(12345f64);
    let y = BigFloat::new().from(54322u16);
    let z = x - y;
    let zz = BigFloat::new().from(-41977i32);
    assert_eq!(z, zz);

    let x = BigFloat::new().from(12345f64);
    let z = x - 54322i32;
    let zz = BigFloat::new().from(-41977i32);
    assert_eq!(z, zz);
}

#[test]
fn test_multiplication() {
    let x = BigFloat::new().from(12345f64);
    let mut y = BigFloat::new().from(-54322i32);
    let z = x * &y;
    y.set_to(-670605090i64);
    assert_eq!(z, y);

    let mut x = BigFloat::new().from(12345f64);
    let z = &x * (-54322i32);
    x.set_to(-670605090i64);
    assert_eq!(z, x);
}

#[test]
fn test_division() {
    let x = BigFloat::new().with_prec(30.digits()).from("2839231.999769250");
    let mut y = BigFloat::new().from(3.346643375f64);
    let mut z = x / &y;
    z.set_prec_round(10.digits());  // supress division artifcats
    y.set_to(848382u32);
    assert_eq!(z, y);
}


