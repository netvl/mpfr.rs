#![allow(missing_copy_implementations)]

use UpdateBigFloat;
use BigFloat;

pub struct BigFloatBuilder;

macro_rules! impl_building {
    ($target:ty, $constructor:expr) => {
        impl $target {
            #[inline]
            pub fn fresh(self) -> BigFloat {
                $constructor(self)
            }

            #[inline]
            pub fn from<T: UpdateBigFloat>(self, value: T) -> BigFloat {
                let mut r = self.fresh();
                r.set_to(value);
                r
            }

            #[inline]
            pub fn const_log2(self) -> BigFloat {
                let mut r = self.fresh();
                r.set_to_const_log2();
                r
            }

            #[inline]
            pub fn const_pi(self) -> BigFloat {
                let mut r = self.fresh();
                r.set_to_const_pi();
                r
            }

            #[inline]
            pub fn const_euler(self) -> BigFloat {
                let mut r = self.fresh();
                r.set_to_const_euler();
                r
            }

            #[inline]
            pub fn const_catalan(self) -> BigFloat {
                let mut r = self.fresh();
                r.set_to_const_catalan();
                r
            }
        }
    }
}

impl BigFloatBuilder {
    #[inline]
    pub fn with_prec(self, precision: usize) -> BigFloatBuilderWithPrec {
        BigFloatBuilderWithPrec(precision)
    }
}

impl_building! { BigFloatBuilder, |:_| BigFloat::fresh() }

pub struct BigFloatBuilderWithPrec(usize);

impl_building! { 
    BigFloatBuilderWithPrec,
    |: this: BigFloatBuilderWithPrec| BigFloat::fresh_with_prec(this.0)
}
