use libc::c_ulong;

use mpfr_sys::*;

use {BigFloat, grnd};

pub trait Math {
    type Output;

    fn sqr(self) -> Self::Output;
    fn sqrt(self) -> Self::Output;
    fn sqrt_rec(self) -> Self::Output;
    fn cbrt(self) -> Self::Output;
    fn root(self, k: u32) -> Self::Output;
    fn abs(self) -> Self::Output;
    fn log(self) -> Self::Output;
    fn log2(self) -> Self::Output;
    fn log10(self) -> Self::Output;
    fn exp(self) -> Self::Output;
    fn exp2(self) -> Self::Output;
    fn exp10(self) -> Self::Output;
    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn tan(self) -> Self::Output;
    fn sec(self) -> Self::Output;
    fn csc(self) -> Self::Output;
    fn cot(self) -> Self::Output;
    fn acos(self) -> Self::Output;
    fn asin(self) -> Self::Output;
    fn atan(self) -> Self::Output;
    fn cosh(self) -> Self::Output;
    fn sinh(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
    fn sech(self) -> Self::Output;
    fn csch(self) -> Self::Output;
    fn coth(self) -> Self::Output;
    fn acosh(self) -> Self::Output;
    fn asinh(self) -> Self::Output;
    fn atanh(self) -> Self::Output;
    fn log1p(self) -> Self::Output;
    fn expm1(self) -> Self::Output;
}

macro_rules! impl_math_val {
    ($($meth:ident($($p:ident: $t:ty as $ct:ty),*) -> $mpfr:ident);+) => {
        impl Math for BigFloat {
            type Output = BigFloat;

            $(
            fn $meth(mut self, $($p: $t),*) -> BigFloat {
                unsafe {
                    $mpfr(&mut self.value, &self.value $(, $p as $ct)*, grnd());
                }
                self
            }
            )+
        }
    }
}

macro_rules! impl_math_ref {
    ($($meth:ident($($p:ident: $t:ty as $ct:ty),*) -> $mpfr:ident);+) => {
        impl<'r> Math for &'r BigFloat {
            type Output = BigFloat;

            $(
            #[inline]
            fn $meth(self, $($p: $t),*) -> BigFloat {
                self.clone().$meth($($p),*)
            }
            )+
        }
    }
}

macro_rules! impl_math_all {
    ($($args:tt)*) => {
        impl_math_val! { $($args)* }
        impl_math_ref! { $($args)* }
    }
}

impl_math_all! {
    sqr() -> mpfr_sqr;
    sqrt() -> mpfr_sqrt;
    sqrt_rec() -> mpfr_rec_sqrt;
    cbrt() -> mpfr_cbrt;
    root(k: u32 as c_ulong) -> mpfr_root;
    abs() -> mpfr_abs;
    log() -> mpfr_log;
    log2() -> mpfr_log2;
    log10() -> mpfr_log10;
    exp() -> mpfr_exp;
    exp2() -> mpfr_exp2;
    exp10() -> mpfr_exp10;
    sin() -> mpfr_sin;
    cos() -> mpfr_cos;
    tan() -> mpfr_tan;
    sec() -> mpfr_sec;
    csc() -> mpfr_csc;
    cot() -> mpfr_cot;
    acos() -> mpfr_acos;
    asin() -> mpfr_asin;
    atan() -> mpfr_atan;
    cosh() -> mpfr_cosh;
    sinh() -> mpfr_sinh;
    tanh() -> mpfr_tanh;
    sech() -> mpfr_sech;
    csch() -> mpfr_csch;
    coth() -> mpfr_coth;
    acosh() -> mpfr_acosh;
    asinh() -> mpfr_asinh;
    atanh() -> mpfr_atanh;
    log1p() -> mpfr_log1p;
    expm1() -> mpfr_expm1
}
