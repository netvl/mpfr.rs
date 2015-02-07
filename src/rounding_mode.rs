use mpfr_sys::*;

/// Contains accessors for setting the rounding mode used by default.
///
/// Almost all operations MPFR provides require specifying a rounding mode which will
/// be applied when the result is stored into a variable with smaller precision
/// than required. In order to provide convenient binary operations but still keep
/// the ability to tweak the rounding mode mpfr.rs stores the default (global) rounding
/// mode in a thread-local variable.
///
/// This module provides access functions for this thread-local value.
pub mod global_rounding_mode {
    use std::cell::Cell;

    use super::RoundingMode;

    thread_local! { static RND: Cell<RoundingMode> = Cell::new(RoundingMode::ToNearest) }

    /// Returns the currently used global rounding mode stored in a thread-local variable.
    #[inline]
    pub fn get() -> RoundingMode {
        RND.with(|v| v.get())
    }

    /// Replaces the currently used global rounding mode with the provided one.
    #[inline]
    pub fn set(m: RoundingMode) {
        RND.with(|v| v.set(m))
    }

    /// Invokes an `FnOnce()` closure with the global rounding mode set to the provided
    /// value.
    ///
    /// After the closure is executed, the global rounding mode is reverted to its initial
    /// value.
    #[inline]
    pub fn with<F, T>(m: RoundingMode, f: F) -> T where F: FnOnce() -> T {
        RND.with(move |v| {
            let prev = v.get();
            v.set(m);
            let r = f();
            v.set(prev);
            r
        })
    }
}

/// Represents rounding mode used by the MPFR library in almost all of its operations.
///
/// Rounding mode defines the behavior of operations which can result in a loss of precision,
/// for example, when the result of an arithmetic action is stored into a variable with
/// smaller precision than that of the operands.
///
/// Rounding mode parameter is not exposed on all of these operations directly. Instead it
/// can be set globally with `global_rounding_mode::set()` or adjusted temporarily with 
/// `global_rounding_mode::with()`.
#[derive(Copy, PartialEq, Eq, Debug)]
pub enum RoundingMode {
    ToNearest    = MPFR_RNDN as isize,
    TowardsZero  = MPFR_RNDZ as isize,
    Upwards      = MPFR_RNDU as isize,
    Downwards    = MPFR_RNDD as isize,
    AwayFromZero = MPFR_RNDA as isize
}

impl RoundingMode {
    /// A shorthand for `global_rounding_mode::set()`.
    ///
    /// Compare:
    ///
    /// ```rust
    /// # use mpfr::{global_rounding_mode, RoundingMode};
    /// global_rounding_mode::with(RoundingMode::Downwards, || {
    ///     // ... whatever ...
    /// })
    /// ```
    /// 
    /// vs.
    ///
    /// ```rust
    /// # use mpfr::RoundingMode;
    /// RoundingMode::Downwards.use_in(|| {
    ///     // ... whatever ...
    /// })
    /// ```
    ///
    #[inline]
    pub fn use_in<F, T>(self, f: F) -> T where F: FnOnce() -> T {
        global_rounding_mode::with(self, f)
    }
}
