#![feature(globs, macro_rules)]

extern crate libc;
extern crate "mpfr-sys" as mpfr_sys;

use std::mem;

use mpfr_sys::*;

pub use update_big_float::UpdateBigFloat;
pub use builder::{BigFloatBuilder, BigFloatBuilderWithPrec};

mod update_big_float;
mod builder;

pub mod global_rounding_mode {
    use std::cell::Cell;

    use super::RoundingMode;

    thread_local! { static RND: Cell<RoundingMode> = Cell::new(RoundingMode::ToNearest) }

    #[inline]
    pub fn get() -> RoundingMode {
        RND.with(|v| v.get())
    }

    #[inline]
    pub fn set(m: RoundingMode) {
        RND.with(|v| v.set(m))
    }

    #[inline]
    pub fn with<F>(m: RoundingMode, f: F) where F: FnOnce() {
        RND.with(move |v| {
            let prev = v.get();
            v.set(m);
            f();
            v.set(prev);
        });
    }
}

#[derive(Copy, PartialEq, Eq, Show)]
pub enum RoundingMode {
    ToNearest    = MPFR_RNDN as int,
    TowardsZero  = MPFR_RNDZ as int,
    Upwards      = MPFR_RNDU as int,
    Downwards    = MPFR_RNDD as int,
    AwayFromZero = MPFR_RNDA as int
}

impl RoundingMode {
    #[inline]
    fn to_rnd_t(self) -> mpfr_rnd_t {
        self as mpfr_rnd_t
    }
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
    pub fn set_to<T: UpdateBigFloat>(&mut self, value: &T) {
        value.update_big_float(self);
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
}
