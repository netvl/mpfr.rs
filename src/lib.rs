#![feature(libc, std_misc, core, unicode, io, collections, hash)]

extern crate libc;
extern crate "mpfr-sys" as mpfr_sys;
#[macro_use] #[no_link] extern crate bitflags;

use std::mem;
use std::ptr;
use std::ffi;
use std::fmt;
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::cmp::Ordering;
use std::num::{Int, FromPrimitive};

use libc::c_double;

use mpfr_sys::*;

pub use flags::Flags;
pub use update_big_float::UpdateBigFloat;
pub use from_big_float::FromBigFloat;
pub use to_big_float::ToBigFloat;
pub use builder::{BigFloatBuilder, BigFloatBuilderWithPrec};
pub use rounding_mode::{RoundingMode, global_rounding_mode};
pub use math::Math;
pub use pow::Pow;
pub use precision::{Precision, ToPrecision};

#[macro_use] mod macros;
mod flags;
mod update_big_float;
mod from_big_float;
mod to_big_float;
mod builder;
mod rounding_mode;
mod math;
mod pow;
mod util;
mod precision;

pub mod format;

pub mod traits {
    pub use UpdateBigFloat;
    pub use FromBigFloat;
    pub use ToBigFloat;
    pub use Math;
    pub use Pow;
    pub use ToPrecision;
}

#[inline]
fn grnd() -> mpfr_rnd_t {
    global_rounding_mode::get() as mpfr_rnd_t
}

/// Represents a numerical sign.
#[derive(Copy)]
pub enum Sign {
    Negative,
    Zero,
    Positive
}

impl Sign {
    /// Obtains a `Sign` value from the given integral number.
    ///
    /// The standard convention is used: if the given number is less than zero, then
    /// `Negative` is returned; if greater than zero, then `Positive` is returned;
    /// if equals to zero, then `Zero` is returned.
    pub fn from_int<I: Int>(i: I) -> Sign {
        match i.cmp(&Int::zero()) {
            Ordering::Less => Sign::Negative,
            Ordering::Equal => Sign::Zero,
            Ordering::Greater => Sign::Positive
        }
    }

    /// Converts the `Sign` value to an integral number.
    ///
    /// The standard convention is used: `Negative` is turned to `-1`, `Zero` is turned to `0`,
    /// `Positive` - to `1`.
    pub fn to_int<I: Int>(self) -> I {
        match self {
            Sign::Negative => <I as Int>::zero()-Int::one(), 
            Sign::Zero => Int::zero(),
            Sign::Positive => Int::one()
        }
    }
}

pub struct BigFloat {
    value: __mpfr_struct
}

impl fmt::Debug for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BigFloat {{ prec: {}, sign: {}, exp: {}, d: {:p}, value: {} }}", 
               self.value._mpfr_prec, self.value._mpfr_sign,
               self.value._mpfr_exp, self.value._mpfr_d, self)
    }
}

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use format::{FormatOptions, Format};

        let s = FormatOptions::new(Format::Fixed)
            .with_precision_of(self)
            .format(self);
        write!(f, "{}", s)
    }
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

macro_rules! generate_constant_setters {
    ($t:ty, $($(#[$attr:meta])* fn $method:ident -> $mpfr:ident),+) => (
        impl $t {
        $(
            $(#[$attr])*
            pub fn $method(&mut self) {
                unsafe {
                    $mpfr(&mut self.value, grnd());
                }
            }
        )+
        }
    )
}

impl BigFloat {
    #[inline]
    pub fn set_default_prec(precision: Precision) {
        unsafe { mpfr_set_default_prec(precision.bits() as mpfr_prec_t); }
    }

    #[inline]
    pub fn get_default_prec() -> Precision {
        unsafe { mpfr_get_default_prec().bits() }
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

    pub fn fresh_with_prec(precision: Precision) -> BigFloat {
        BigFloat {
            value: unsafe {
                let mut value = mem::uninitialized();
                mpfr_init2(&mut value, precision.bits() as mpfr_prec_t);
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
        <T as FromBigFloat>::from_big_float(self)
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
            mpfr_set_inf(&mut self.value, sign.to_int());
        }
    }

    #[inline]
    pub fn set_to_zero(&mut self, sign: Sign) {
        unsafe {
            mpfr_set_inf(&mut self.value, sign.to_int());
        }
    }

    #[inline]
    pub fn swap(&mut self, other: &mut BigFloat) {
        unsafe {
             mpfr_swap(&mut self.value, &mut other.value);
        }
    }

    pub fn prec(&self) -> Precision {
        unsafe {
            mpfr_get_prec(&self.value).bits()
        }
    }

    pub fn set_prec_clear(&mut self, precision: Precision) {
        unsafe {
            mpfr_set_prec(&mut self.value, precision.bits() as mpfr_prec_t);
        }
    }

    pub fn set_prec_round(&mut self, precision: Precision) {
        unsafe {
            mpfr_prec_round(&mut self.value, precision.bits() as mpfr_prec_t, grnd());
        }
    }

    pub fn to_string_in_base(&self, base: u32) -> (String, u64) {
        unsafe {
            let mut exp: mpfr_exp_t = 0;
            // We're going to ask MPFR to allocate the string itself, so the maximum
            // possible precision is used
            let s = mpfr_get_str(
                ptr::null_mut(),
                &mut exp, base as libc::c_int, 
                0, &self.value,
                grnd()
            );

            if s.is_null() {
                panic!("Couldn't convert big float to a string");
            }

            let v = ffi::c_str_to_bytes(&(s as *const _)).to_vec();
            mpfr_free_str(s);

            let r = String::from_utf8(v).unwrap();

            (r, exp as u64)
        }
    }

    pub fn sgn(&self) -> Option<Sign> {
        unsafe {
            let r = mpfr_sgn(&self.value);
            match Sign::from_int(r) {
                Sign::Zero if Flags::Erange.is_set() => None,
                s => Some(s)
            }
        }
    }

    #[inline]
    pub fn free_cache() {
        unsafe {
            mpfr_free_cache();
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
    fn is_regular -> mpfr_regular_p,
    #[doc="Checks that this number is an integer."]
    fn is_integer -> mpfr_integer_p
}

generate_constant_setters! { BigFloat,
    fn set_to_const_log2     -> mpfr_const_log2,
    fn set_to_const_pi       -> mpfr_const_pi,
    fn set_to_const_euler    -> mpfr_const_euler,
    fn set_to_const_catalan  -> mpfr_const_catalan
}

impl FromPrimitive for BigFloat {
    fn from_i64(n: i64) -> Option<BigFloat> {
        Some(n.to_big_float())
    }

    fn from_u64(n: u64) -> Option<BigFloat> {
        Some(n.to_big_float())
    }
    
    fn from_f32(n: f32) -> Option<BigFloat> {
        Some(n.to_big_float())
    }

    fn from_f64(n: f64) -> Option<BigFloat> {
        Some(n.to_big_float())
    }
}

// Addition
impl_commutative_op! { Add, add, mpfr_add, mpfr_add_d }

// Multiplication
impl_commutative_op! { Mul, mul, mpfr_mul, mpfr_mul_d }

// Subtraction
impl_noncommutative_op! { Sub, sub, mpfr_sub, mpfr_sub_d }

// Division
impl_noncommutative_op! { Div, div, mpfr_div, mpfr_div_d }

// Unary negation

impl Neg for BigFloat {
    type Output = BigFloat;

    fn neg(mut self) -> BigFloat {
        unsafe {
            mpfr_neg(&mut self.value, &self.value, grnd());
        }
        self
    }
}

impl<'r> Neg for &'r BigFloat {
    type Output = BigFloat;

    #[inline]
    fn neg(self) -> BigFloat {
        -self.clone()
    }
}

// Equality check

impl PartialEq for BigFloat {
    #[inline]
    fn eq(&self, other: &BigFloat) -> bool {
        unsafe { mpfr_equal_p(&self.value, &other.value) > 0 } 
    }
}

impl PartialOrd for BigFloat {
    fn partial_cmp(&self, other: &BigFloat) -> Option<Ordering> {
        match unsafe { mpfr_cmp(&self.value, &other.value) } {
            r if r < 0 => Some(Ordering::Less),
            r if r > 0 => Some(Ordering::Greater),
            _ if Flags::Erange.is_set() => None,
            _ => Some(Ordering::Equal)
        }
    }
}

