use std::ffi::CString;

use libc::{uintmax_t, intmax_t, c_double, c_float, c_int};

use mpfr_sys::*;

use {BigFloat, grnd};

pub trait UpdateBigFloat {
    fn update_big_float(self, target: &mut BigFloat);
}

impl<'a> UpdateBigFloat for &'a BigFloat {
    fn update_big_float(self, target: &mut BigFloat) {
        unsafe {
            mpfr_set(&mut target.value, &self.value, grnd());
        }
    }
}

macro_rules! impl_big_float_set {
    ($t:ty as $tt:ty, $f:ident) => {
        impl UpdateBigFloat for $t {
            fn update_big_float(self, target: &mut BigFloat) {
                unsafe {
                    $f(&mut target.value, self as $tt, grnd());
                }
            }
        }
    };
    ($($t:ty as $tt:ty, $f:ident);+) => {
        $(impl_big_float_set! { $t as $tt, $f })+
    }
}

impl_big_float_set! {
    f32 as c_float,   mpfr_set_flt;
    f64 as c_double,  mpfr_set_d;
    i8  as intmax_t,  __gmpfr_set_sj;
    u8  as uintmax_t, __gmpfr_set_uj;
    i16 as intmax_t,  __gmpfr_set_sj;
    u16 as uintmax_t, __gmpfr_set_uj;
    i32 as intmax_t,  __gmpfr_set_sj;
    u32 as uintmax_t, __gmpfr_set_uj;
    i64 as intmax_t,  __gmpfr_set_sj;
    u64 as uintmax_t, __gmpfr_set_uj
}

impl<'a> UpdateBigFloat for &'a str {
    #[inline]
    fn update_big_float(self, target: &mut BigFloat) {
        (self, 10).update_big_float(target);
    }
}

impl<'a> UpdateBigFloat for (&'a str, usize) {
    fn update_big_float(self, target: &mut BigFloat) {
        let s = CString::from_slice(self.0.as_bytes());

        let r = unsafe {
            mpfr_set_str(&mut target.value, s.as_ptr(), self.1 as c_int, grnd())
        };

        if r != 0 {
            panic!("Cannot set big float from a string: {}", self.0);
        }
    }
}

// TODO: when bindings for libgmp are available, use them here too, probably as a Cargo feature
