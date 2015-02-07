#![allow(missing_copy_implementations)]

use UpdateBigFloat;
use BigFloat;
use Precision;

pub struct BigFloatBuilder;

macro_rules! generate_const_methods {
    ($target:ty, $($method:ident -> $setter:ident),+) => {
        impl $target {
            $(
            #[inline]
            pub fn $method(self) -> BigFloat {
                let mut r = self.fresh();
                r.$setter();
                r
            }
            )+
        }
    }
}

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
        }

        generate_const_methods! { $target,
            const_log2 -> set_to_const_log2,
            const_pi -> set_to_const_pi,
            const_euler -> set_to_const_euler,
            const_catalan -> set_to_const_catalan
        }
    }
}

impl BigFloatBuilder {
    #[inline]
    pub fn with_prec(self, precision: Precision) -> BigFloatBuilderWithPrec {
        BigFloatBuilderWithPrec(precision)
    }
}

impl_building! { BigFloatBuilder, |_| BigFloat::fresh() }

pub struct BigFloatBuilderWithPrec(Precision);

impl_building! { 
    BigFloatBuilderWithPrec,
    |this: BigFloatBuilderWithPrec| BigFloat::fresh_with_prec(this.0)
}
