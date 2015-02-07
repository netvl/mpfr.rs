use mpfr_sys::*;

use {BigFloat, grnd};

pub trait FromBigFloat {
    type Target;
    fn from_big_float(x: &BigFloat) -> Self::Target;
}

macro_rules! from_big_float_impl {
    ($t:ty, $f:ident) => {
        impl FromBigFloat for $t {
            type Target = $t;
            fn from_big_float(x: &BigFloat) -> $t {
                unsafe {
                    $f(&x.value, grnd()) as $t
                }
            }
        }
    }
}

from_big_float_impl! { f32, mpfr_get_flt }
from_big_float_impl! { f64, mpfr_get_d }
from_big_float_impl! { i32, __gmpfr_mpfr_get_sj }
from_big_float_impl! { i64, __gmpfr_mpfr_get_sj }
from_big_float_impl! { u32, __gmpfr_mpfr_get_uj }
from_big_float_impl! { u64, __gmpfr_mpfr_get_uj }

impl FromBigFloat for String {
    type Target = (String, u64);

    #[inline]
    fn from_big_float(x: &BigFloat) -> (String, u64) {
        x.to_string_in_base(10)
    }
}

