use std::ffi::{self, CString};
use std::ptr;
use std::str;
use std::borrow::ToOwned;

use libc::c_char;

use mpfr_sys::*;

use BigFloat;
use global_rounding_mode;

// TODO: add flags

#[derive(Copy, PartialEq, Eq)]
pub enum Format {
    HexFloat,
    Binary,
    Fixed,
    Scientific,
    FixedOrScientific
}

#[derive(Copy, PartialEq, Eq)]
pub enum Case {
    Lower,
    Upper
}

#[derive(Copy, PartialEq, Eq)]
pub enum RoundingMode {
    Specific(::RoundingMode),
    Global
}

#[allow(missing_copy_implementations)]
#[derive(Copy, PartialEq, Eq)]
pub struct FormatOptions {
    pub format: Format,
    pub case: Case,
    pub rounding_mode: RoundingMode,
    pub width: Option<u32>,
    pub precision: Option<u32>
}

impl FormatOptions {
    pub fn new(format: Format) -> FormatOptions {
        FormatOptions {
            format: format,
            case: Case::Lower,
            rounding_mode: RoundingMode::Global,
            width: None,
            precision: None
        }
    }

    #[inline]
    pub fn with_format(mut self, format: Format) -> FormatOptions {
        self.format = format;
        self
    }

    #[inline]
    pub fn with_case(mut self, case: Case) -> FormatOptions {
        self.case = case;
        self
    }

    #[inline]
    pub fn with_rounding_mode(mut self, mode: ::RoundingMode) -> FormatOptions {
        self.rounding_mode = RoundingMode::Specific(mode);
        self
    }

    #[inline]
    pub fn with_global_rounding_mode(mut self) -> FormatOptions {
        self.rounding_mode = RoundingMode::Global;
        self
    }

    #[inline]
    pub fn with_width(mut self, width: u32) -> FormatOptions {
        self.width = Some(width);
        self
    }

    #[inline]
    pub fn without_width(mut self) -> FormatOptions {
        self.width = None;
        self
    }

    #[inline]
    pub fn with_precision(mut self, precision: u32) -> FormatOptions {
        self.precision = Some(precision);
        self
    }

    #[inline]
    pub fn without_precision(mut self) -> FormatOptions {
        self.precision = None;
        self
    }

    pub fn format_string(&self) -> String {
        let mut result = b"%".to_owned();
        if let Some(width) = self.width {
            write!(&mut result, "{}", width).unwrap();
        }
        if let Some(precision) = self.precision {
            write!(&mut result, ".{}", precision).unwrap();
        }
        let mut result = String::from_utf8(result).unwrap();
        result.push_str("R*");  // rounding mode from arguments
        let c = match self.format {
            Format::HexFloat => 'a',
            Format::Binary => 'b',
            Format::Fixed => 'f',
            Format::Scientific => 'e',
            Format::FixedOrScientific => 'g'
        };
        let c = match self.case {
            Case::Lower => c,
            Case::Upper if self.format == Format::Binary => c,
            Case::Upper => c.to_uppercase()
        };
        result.push(c);
        result
    }

    #[inline]
    pub fn format(&self, x: &BigFloat) -> String {
        let f = CString::from_vec(self.format_string().into_bytes());
        unsafe {
            let rnd_mode = match self.rounding_mode {
                RoundingMode::Specific(m) => m,
                RoundingMode::Global => global_rounding_mode::get()
            }.to_rnd_t();

            let mut r: *mut c_char = ptr::null_mut();
            let n = mpfr_asprintf(&mut r, f.as_ptr(), rnd_mode, &x.value);

            if n < 0 {
                panic!("Could not format the big float");
            }

            let rs = str::from_utf8(ffi::c_str_to_bytes(&(r as *const _))).unwrap().to_owned();

            mpfr_free_str(r);

            rs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_and_case() {
        macro_rules! test_formats {
            ($($f:path, $c:path -> $e:expr);+) => {{
                $(assert_eq!($e, FormatOptions::new($f).with_case($c).format_string()[]);)+
            }};

            ($($f:path -> $e:expr);+) => {{
                $(assert_eq!($e, FormatOptions::new($f).format_string()[]);)+
            }}
        }

        test_formats! {
            Format::HexFloat                       -> "%R*a";
            Format::Binary                         -> "%R*b";
            Format::Fixed                          -> "%R*f";
            Format::Scientific                     -> "%R*e";
            Format::FixedOrScientific              -> "%R*g"
        }

        test_formats! {
            Format::HexFloat,          Case::Lower -> "%R*a";
            Format::Binary,            Case::Lower -> "%R*b";
            Format::Fixed,             Case::Lower -> "%R*f";
            Format::Scientific,        Case::Lower -> "%R*e";
            Format::FixedOrScientific, Case::Lower -> "%R*g"
        }

        test_formats! {
            Format::HexFloat,          Case::Upper -> "%R*A";
            Format::Binary,            Case::Upper -> "%R*b";
            Format::Fixed,             Case::Upper -> "%R*F";
            Format::Scientific,        Case::Upper -> "%R*E";
            Format::FixedOrScientific, Case::Upper -> "%R*G"
        }
    }

    #[test]
    fn test_width_and_precision() {
        let f = FormatOptions::new(Format::Fixed);
        assert_eq!("%10R*f", f.with_width(10).format_string()[]);
        assert_eq!("%.20R*f", f.with_precision(20).format_string()[]);
        assert_eq!("%10.20R*f", f.with_width(10).with_precision(20).format_string()[]);
    }
}
