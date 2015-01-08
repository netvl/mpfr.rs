extern crate libc;
extern crate "mpfr-sys" as mpfr_sys;

use std::mem;
use std::ptr;
use std::borrow::ToOwned;
use std::str;
use std::ffi;
use std::ops::{Add, Mul};

use libc::c_double;

use mpfr_sys::*;

pub use update_big_float::UpdateBigFloat;
pub use from_big_float::FromBigFloat;
pub use to_big_float::ToBigFloat;
pub use builder::{BigFloatBuilder, BigFloatBuilderWithPrec};
pub use rounding_mode::{RoundingMode, global_rounding_mode};

mod util;
mod update_big_float;
mod from_big_float;
mod to_big_float;
mod builder;
mod rounding_mode;

pub mod format;

pub mod traits {
    pub use UpdateBigFloat;
    pub use FromBigFloat;
    pub use ToBigFloat;
}

#[derive(Copy)]
pub enum Sign {
    Negative,
    Positive
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
        BigFloat::new().with_prec(self.prec()).from(self)
    }
}

macro_rules! generate_predicates {
    ($t:ty, $($(#[$attr:meta])* fn $method:ident -> $mpfr:ident),+) => (
        impl $t {
        $(
            $(#[$attr])*
            pub fn $method(&self) -> bool {
                unsafe {
                    $mpfr(&self.value) != 0
                }
            }
        )+
        }
    )
}

impl BigFloat {
    #[inline]
    pub fn set_default_prec(precision: uint) {
        unsafe { mpfr_set_default_prec(precision as mpfr_prec_t); }
    }

    #[inline]
    pub fn get_default_prec() -> uint {
        unsafe { mpfr_get_default_prec() as uint }
    }

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

generate_predicates! { BigFloat,
    #[doc="Checks that this number is NaN."]
    fn is_nan     -> mpfr_nan_p,
    #[doc="Checks that this number is an infinity (positive or negative)."]
    fn is_inf     -> mpfr_inf_p,
    #[doc="Checks that this number is an ordinary number (neither NaN nor an infinity)."]
    fn is_number  -> mpfr_number_p,
    #[doc="Checks that this number is zero."]
    fn is_zero    -> mpfr_zero_p,
    #[doc="Checks that this number is a regular number (neither NaN, nor an infinity nor zero)."]
    fn is_regular -> mpfr_regular_p
}

// Implementation of arithmetic operations for BigFloat.
// Basically, all operations are implemented both for BigFloat and &BigFloat as RHS and LHS
// (4 variants total: value + value, value + reference, reference + value, reference + reference).
//
// If one of the operands is a value, then it is used to hold the result. If both of the
// operands are values, LHS gets a priority. If both of the operands are references,
// the LHS is cloned and used to hold the result.
//
// The order is important if participating values have different precisions.
//
// If one of the operands is of primitive type, then either the BigFloat operand will
// be reused for the result or it will be cloned.

macro_rules! impl_commutative_op_val_ref {
    ($tr:ident, $meth:ident, $mpfr:ident) => {
        impl<'a> $tr<&'a BigFloat> for BigFloat {
            type Output = BigFloat;

            fn $meth(mut self, rhs: &'a BigFloat) -> BigFloat {
                unsafe {
                    $mpfr(
                        &mut self.value,
                        &self.value, &rhs.value,
                        global_rounding_mode::get().to_rnd_t()
                    );
                }
                self
            }
        }
    }
}

macro_rules! impl_commutative_op_val_val {
    ($tr:ident, $meth:ident) => {
        impl $tr<BigFloat> for BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: BigFloat) -> BigFloat {
                self.$meth(&rhs)
            }
        }
    }
}

macro_rules! impl_commutative_op_ref_val {
    ($tr:ident, $meth:ident) => {
        impl<'r> $tr<BigFloat> for &'r BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: BigFloat) -> BigFloat {
                rhs.$meth(self)
            }
        }
    }
}

macro_rules! impl_commutative_op_ref_ref {
    ($tr:ident, $meth:ident) => {
        impl<'a, 'r> $tr<&'a BigFloat> for &'r BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: &'a BigFloat) -> BigFloat {
                let c = self.clone();
                c.$meth(rhs)
            }
        }
    }
}

macro_rules! impl_commutative_op {
    ($tr:ident, $meth:ident, $mpfr:ident) => {
        impl_commutative_op_val_ref! { $tr, $meth, $mpfr }
        impl_commutative_op_val_val! { $tr, $meth }
        impl_commutative_op_ref_val! { $tr, $meth }
        impl_commutative_op_ref_ref! { $tr, $meth }
    }
}

// Addition

impl_commutative_op! { Add, add, mpfr_add }

// x + f
impl Add<f64> for BigFloat {
    type Output = BigFloat;

    fn add(mut self, rhs: f64) -> BigFloat {
        unsafe {
            mpfr_add_d(
                &mut self.value,
                &self.value, rhs as c_double,
                global_rounding_mode::get().to_rnd_t()
            );
        }
        self
    }
}

// &x + f
impl<'r> Add<f64> for &'r BigFloat {
    type Output = BigFloat;

    #[inline]
    fn add(self, rhs: f64) -> BigFloat {
        let r = self.clone();
        r + rhs
    }
}

/*
// f + x
impl Add<BigFloat> for f64 {
    type Output = BigFloat;

    #[inline]
    fn add(self, rhs: BigFloat) -> BigFloat {
        rhs + self
    }
}

// f + &x
impl<'r> Add<&'r BigFloat> for f64 {
    type Output = BigFloat;

    #[inline]
    fn add(self, rhs: &'r BigFloat) -> BigFloat {
        rhs + self
    }
}
*/

// Multiplication

impl_commutative_op! { Mul, mul, mpfr_mul }
