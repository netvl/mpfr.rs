extern crate libc;
extern crate "mpfr-sys" as mpfr_sys;

use std::mem;
use std::ptr;
use std::borrow::ToOwned;
use std::str;
use std::ffi;

use mpfr_sys::*;

pub use update_big_float::UpdateBigFloat;
pub use from_big_float::FromBigFloat;
pub use builder::{BigFloatBuilder, BigFloatBuilderWithPrec};
pub use format::FormatOptions;
pub use rounding_mode::{RoundingMode, global_rounding_mode};

mod util;
mod update_big_float;
mod from_big_float;
mod builder;
mod rounding_mode;

pub mod format;

pub mod traits {
    pub use UpdateBigFloat;
    pub use FromBigFloat;
}

#[derive(Copy)]
pub enum Sign {
    Negative,
    Positive
}

#[inline]
pub fn set_default_prec(precision: uint) {
    unsafe {
        mpfr_set_default_prec(precision as mpfr_prec_t);
    }
}

#[inline]
pub fn get_default_prec() -> uint {
    unsafe {
        mpfr_get_default_prec() as uint
    }
}

pub struct BigFloat {
    value: __mpfr_struct
}

impl Drop for BigFloat {
    fn drop(&mut self) {
        unsafe { mpfr_clear(&mut self.value) }
    }
}

impl Clone for BigFloat {
    fn clone(&self) -> BigFloat {
        let mut new_value = unsafe { mem::uninitialized() };
        unsafe {
            let prec = mpfr_get_prec(&self.value);
            mpfr_init2(&mut new_value, prec);
            // rounding mode does not matter here
            mpfr_set(&mut new_value, &self.value, MPFR_RNDN);
        }
        BigFloat { value: new_value }
    }
}

impl BigFloat {
    #[inline]
    pub fn new() -> BigFloatBuilder { BigFloatBuilder }

    pub fn fresh() -> BigFloat {
        BigFloat {
            value: unsafe {
                let mut value = mem::uninitialized();
                mpfr_init(&mut value);
                value
            }
        }
    }

    pub fn fresh_with_prec(precision: uint) -> BigFloat {
        BigFloat {
            value: unsafe {
                let mut value = mem::uninitialized();
                mpfr_init2(&mut value, precision as mpfr_prec_t);
                value
            }
        }
    }

    #[inline]
    pub fn set_to<T: UpdateBigFloat>(&mut self, value: T) {
        value.update_big_float(self);
    }

    #[inline]
    pub fn get<T: FromBigFloat>(&self) -> T::Target {
        FromBigFloat::from_big_float(self, None::<T>)
    }

    #[inline]
    pub fn set_to_nan(&mut self) {
        unsafe {
            mpfr_set_nan(&mut self.value);
        }
    }

    #[inline]
    pub fn set_to_inf(&mut self, sign: Sign) {
        unsafe {
            mpfr_set_inf(&mut self.value, match sign {
                Sign::Negative => -1,
                Sign::Positive =>  1
            });
        }
    }

    #[inline]
    pub fn set_to_zero(&mut self, sign: Sign) {
        unsafe {
            mpfr_set_inf(&mut self.value, match sign {
                Sign::Negative => -1,
                Sign::Positive =>  1
            });
        }
    }

    #[inline]
    pub fn swap(&mut self, other: &mut BigFloat) {
        unsafe {
             mpfr_swap(&mut self.value, &mut other.value);
        }
    }

    pub fn prec(&self) -> uint {
        unsafe {
            mpfr_get_prec(&self.value) as uint
        }
    }

    pub fn set_prec_clear(&mut self, precision: uint) {
        unsafe {
            mpfr_set_prec(&mut self.value, precision as mpfr_prec_t);
        }
    }

    pub fn set_prec_round(&mut self, precision: uint) {
        unsafe {
            mpfr_prec_round(&mut self.value, 
                            precision as mpfr_prec_t, 
                            global_rounding_mode::get().to_rnd_t());
        }
    }

    pub fn to_string_in_base(&self, base: uint) -> (String, u64) {
        unsafe {
            let mut exp: mpfr_exp_t = 0;
            // We're going to ask MPFR to allocate the string itself, so the maximum
            // possible precision is used
            let s = mpfr_get_str(
                ptr::null_mut(),
                &mut exp, base as libc::c_int, 
                0, &self.value,
                global_rounding_mode::get().to_rnd_t()
            );

            if s.is_null() {
                panic!("Couldn't convert big float to a string");
            }

            let r = str::from_utf8(ffi::c_str_to_bytes(&(s as *const _))).unwrap().to_owned();

            mpfr_free_str(s);

            (r, exp as u64)
        }
    }
}
