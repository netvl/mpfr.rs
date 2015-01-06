#![allow(missing_copy_implementations)]

use UpdateBigFloat;
use BigFloat;

pub struct BigFloatBuilder;

impl BigFloatBuilder {
    #[inline]
    pub fn with_prec(self, precision: uint) -> BigFloatBuilderWithPrec {
        BigFloatBuilderWithPrec(precision)
    }

    #[inline]
    pub fn fresh(self) -> BigFloat { BigFloat::fresh() }

    #[inline]
    pub fn from<T: UpdateBigFloat>(self, value: T) -> BigFloat {
        let mut r = BigFloat::fresh();
        r.set_to(value);
        r
    }
}

pub struct BigFloatBuilderWithPrec(uint);

impl BigFloatBuilderWithPrec {
    #[inline]
    pub fn fresh(self) -> BigFloat { BigFloat::fresh_with_prec(self.0) }

    #[inline]
    pub fn from<T: UpdateBigFloat>(self, value: T) -> BigFloat {
        let mut r = BigFloat::fresh_with_prec(self.0);
        r.set_to(value);
        r
    }
}
