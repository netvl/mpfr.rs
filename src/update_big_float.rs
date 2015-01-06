use std::c_str::ToCStr;

use libc::{uintmax_t, intmax_t, c_double, c_float, c_int};

use mpfr_sys::*;

use BigFloat;
use global_rounding_mode;

pub trait UpdateBigFloat for ?Sized {
    fn update_big_float(self, target: &mut BigFloat);
}

macro_rules! impl_big_float_set {
    ($t:ty as $tt:ty, $f:ident) => (
        impl UpdateBigFloat for $t {
            fn update_big_float(self, target: &mut BigFloat) {
                unsafe {
                    $f(&mut target.value, self as $tt, global_rounding_mode::get().to_rnd_t());
                }
            }
        }
    )
}

impl<'a> UpdateBigFloat for &'a BigFloat {
    fn update_big_float(self, target: &mut BigFloat) {
        unsafe {
            mpfr_set(&mut target.value, &self.value, global_rounding_mode::get().to_rnd_t());
        }
    }
}

impl_big_float_set! { f32 as c_float,   mpfr_set_flt }
impl_big_float_set! { f64 as c_double,  mpfr_set_d }
impl_big_float_set! { i32 as intmax_t,  __gmpfr_set_sj }
impl_big_float_set! { u32 as uintmax_t, __gmpfr_set_uj }
impl_big_float_set! { i64 as intmax_t,  __gmpfr_set_sj }
impl_big_float_set! { u64 as uintmax_t, __gmpfr_set_uj }

impl<'a> UpdateBigFloat for &'a str {
    #[inline]
    fn update_big_float(self, target: &mut BigFloat) {
        (self, 10).update_big_float(target);
    }
}

impl<'a> UpdateBigFloat for (&'a str, uint) {
    fn update_big_float(self, target: &mut BigFloat) {
        self.0.with_c_str(|s| {
            let r = unsafe {
                mpfr_set_str(
                    &mut target.value, s, self.1 as c_int, 
                    global_rounding_mode::get().to_rnd_t()
                )
            };
            if r != 0 {
                panic!("Cannot set big float from a string: {}", self.0);
            }
        });
    }
}

// TODO: when bindings for libgmp are available, use them here too, probably as a Cargo feature
