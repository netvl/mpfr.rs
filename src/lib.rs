extern crate "mpfr-sys" as mpfr_sys;

use std::mem;

pub struct BigFloat {
    value: mpfr_sys::__mpfr_struct
}

impl Drop for BigFloat {
    fn drop(&mut self) {
        unsafe { mpfr_sys::mpfr_clear(&mut self.value) }
    }
}

impl Clone for BigFloat {
    fn clone(&self) -> BigFloat {
        let mut new_value = unsafe { mem::uninitialized() };
        unsafe {
            let prec = mpfr_sys::mpfr_get_prec(&self.value);
            mpfr_sys::mpfr_init2(&mut new_value, prec);
            // rounding mode does not matter here
            mpfr_sys::mpfr_set(&mut new_value, &self.value, mpfr_sys::MPFR_RNDN);
        }
        BigFloat { value: new_value }
    }
}

impl BigFloat {
    pub fn new() -> BigFloat {
        BigFloat {
            value: unsafe {
                let mut value = mem::uninitialized();
                mpfr_sys::mpfr_init(&mut value);
                value
            }
        }
    }

    pub fn new_with_prec(precision: uint) -> BigFloat {
        BigFloat {
            value: unsafe {
                let mut value = mem::uninitialized();
                mpfr_sys::mpfr_init2(&mut value, precision as mpfr_sys::mpfr_prec_t);
                value
            }
        }
    }
}
