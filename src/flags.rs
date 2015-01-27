use mpfr_sys::*;

#[derive(Copy)]
pub enum Flags {
    Underflow,
    Overflow,
    DivByZero,
    Nan,
    Inexact,
    Erange
}

macro_rules! impl_flags {
    ($($f:ident -> $get:ident, $set:ident, $clear:ident);+) => {
        impl Flags {
            pub fn is_set(self) -> bool {
                (match self {
                    $(Flags::$f => unsafe { $get() }),+
                }) != 0
            }

            pub fn set(self) {
                match self {
                    $(Flags::$f => unsafe { $set() }),+
                }
            }
            
            pub fn clear(self) {
                match self {
                    $(Flags::$f => unsafe { $clear() }),+
                }
            }
        }
    }
}

impl_flags! {
    Underflow -> mpfr_underflow_p, mpfr_set_underflow, mpfr_clear_underflow;
    Overflow -> mpfr_overflow_p, mpfr_set_overflow, mpfr_clear_overflow;
    DivByZero -> mpfr_divby0_p, mpfr_set_divby0, mpfr_clear_divby0;
    Nan -> mpfr_nanflag_p, mpfr_set_nanflag, mpfr_clear_nanflag;
    Inexact -> mpfr_inexflag_p, mpfr_set_inexflag, mpfr_clear_inexflag;
    Erange -> mpfr_erangeflag_p, mpfr_set_erangeflag, mpfr_clear_erangeflag
}

