use mpfr_sys::*;

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
    pub fn use_globally<F>(self, f: F) where F: FnOnce() {
        global_rounding_mode::with(self, f)
    }

    #[inline]
    pub fn to_rnd_t(self) -> mpfr_rnd_t {
        self as mpfr_rnd_t
    }
}
