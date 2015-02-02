use std::num::ToPrimitive;

use util;

#[derive(Copy, PartialEq, Eq, Debug)]
pub struct Precision(u32);

impl Precision {
    #[inline]
    pub fn bits(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn digits(self) -> u32 {
        util::precision::bits_to_digits(self.0)
    }
}

pub trait ToPrecision {
    fn bits(self) -> Precision;
    fn digits(self) -> Precision;
}

impl<T: ToPrimitive> ToPrecision for T {
    #[inline]
    fn bits(self) -> Precision {
        Precision(self.to_u32().unwrap())
    }

    #[inline]
    fn digits(self) -> Precision {
        Precision(util::precision::digits_to_bits(self.to_u32().unwrap()))
    }
}
