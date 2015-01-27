use libc::{c_long, c_ulong};

use mpfr_sys::*;

use {BigFloat, grnd};

pub trait Pow<RHS=Self> {
    type Output;
    fn pow(self, rhs: RHS) -> Self::Output;
}

macro_rules! impl_pow_op {
    ($tr:ident, $meth:ident, $mpfr:ident, $mpfr_u32:ident, $mpfr_i32:ident) => {
        impl_noncommutative_op_val_ref! { $tr, $meth, $mpfr }
        impl_noncommutative_op_val_val! { $tr, $meth }
        impl_noncommutative_op_ref_val! { $tr, $meth, $mpfr }
        impl_noncommutative_op_ref_ref! { $tr, $meth }
        impl_noncommutative_op_val_prim! { $tr, $meth, u32, c_ulong, $mpfr_u32 }
        impl_noncommutative_op_val_prim! { $tr, $meth, i32, c_long, $mpfr_i32 }
    }
}

impl_pow_op! { Pow, pow, mpfr_pow, mpfr_pow_ui, mpfr_pow_si }

