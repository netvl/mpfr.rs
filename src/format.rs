use std::ffi::CString;
use std::ptr;
use std::borrow::{ToOwned, IntoCow, Cow};

use libc::{c_char, size_t};

use mpfr_sys::*;

use {BigFloat, Precision, global_rounding_mode};

pub use self::flags::Flags;

pub mod flags {
    bitflags! {
        flags Flags: u8 {
            const ALTERNATE_FORM = 1 << 0,
            const ZERO_PADDED    = 1 << 1,
            const LEFT_ADJUSTED  = 1 << 2,
            const BLANK          = 1 << 3,
            const SIGN           = 1 << 4
        }
    }
}

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

#[derive(Copy, PartialEq, Eq)]
pub struct FormatOptions {
    pub format: Format,
    pub flags: Flags,
    pub case: Case,
    pub rounding_mode: RoundingMode,
    pub width: Option<u32>,
    pub precision: Option<u32>
}

impl FormatOptions {
    pub fn new(format: Format) -> FormatOptions {
        FormatOptions {
            format: format,
            flags: Flags::empty(),
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
    pub fn with_flags(mut self, flags: Flags) -> FormatOptions {
        self.flags = flags;
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
    pub fn with_precision_of(mut self, value: &BigFloat) -> FormatOptions {
        self.precision = Some(value.prec().digits());
        self
    }

    #[inline]
    pub fn with_precision(mut self, precision: Precision) -> FormatOptions {
        self.precision = Some(precision.digits());
        self
    }

    #[inline]
    pub fn without_precision(mut self) -> FormatOptions {
        self.precision = None;
        self
    }

    pub fn format_string(&self) -> String {
        let mut result = b"%".to_owned();

        for_flags! { self.flags,
            flags::ALTERNATE_FORM => result.push(b'#'),
            flags::ZERO_PADDED    => result.push(b'0'),
            flags::LEFT_ADJUSTED  => result.push(b'-'),
            flags::BLANK          => result.push(b' '),
            flags::SIGN           => result.push(b'+')
        }

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

    pub fn format(&self, x: &BigFloat) -> String {
        unsafe { format_raw(self.format_string().into_cow(), self.rounding_mode, x) }
    }

}

pub unsafe fn format_raw(fmt: Cow<str>, rounding_mode: RoundingMode, x: &BigFloat) -> String {
    let f = CString::new(fmt.into_owned()).unwrap();

    let rnd_mode = match rounding_mode {
        RoundingMode::Specific(m) => m,
        RoundingMode::Global => global_rounding_mode::get()
    } as mpfr_rnd_t;

    // get the required number of bytes
    let n = mpfr_snprintf(ptr::null_mut(), 0, f.as_ptr(), rnd_mode, &x.value);
    if n < 0 {
        panic!("Could not format the big float");
    }

    // allocate the buffer and format the string into it
    let n = n as usize;
    let mut data: Vec<u8> = Vec::with_capacity(n+1);
    data.set_len(n+1);

    let n = mpfr_snprintf(
        data.as_mut_ptr() as *mut c_char, 
        n as size_t + 1,
        f.as_ptr(), rnd_mode, &x.value
    );
    if n < 0 {
        panic!("Could not format the big float");
    }
    data.pop();  // drop the zero byte

    String::from_utf8(data).unwrap()
}

#[cfg(test)]
mod tests {
    use std::str;

    use traits::ToPrecision;

    use super::*;

    #[test]
    fn test_format_and_case() {
        macro_rules! test_formats {
            ($($f:expr, $c:expr; -> $e:expr);+) => {{
                $(assert_eq!($e, &FormatOptions::new($f).with_case($c).format_string()[]);)+
            }};

            ($($f:expr; -> $e:expr);+) => {{
                $(assert_eq!($e, &FormatOptions::new($f).format_string()[]);)+
            }}
        }

        test_formats! {
            Format::HexFloat;                       -> "%R*a";
            Format::Binary;                         -> "%R*b";
            Format::Fixed;                          -> "%R*f";
            Format::Scientific;                     -> "%R*e";
            Format::FixedOrScientific;              -> "%R*g"
        }

        test_formats! {
            Format::HexFloat,          Case::Lower; -> "%R*a";
            Format::Binary,            Case::Lower; -> "%R*b";
            Format::Fixed,             Case::Lower; -> "%R*f";
            Format::Scientific,        Case::Lower; -> "%R*e";
            Format::FixedOrScientific, Case::Lower; -> "%R*g"
        }

        test_formats! {
            Format::HexFloat,          Case::Upper; -> "%R*A";
            Format::Binary,            Case::Upper; -> "%R*b";
            Format::Fixed,             Case::Upper; -> "%R*F";
            Format::Scientific,        Case::Upper; -> "%R*E";
            Format::FixedOrScientific, Case::Upper; -> "%R*G"
        }
    }

    #[test]
    fn test_flags() {
        use util::SliceSubsetsExt;

        let flags = [
            (flags::ALTERNATE_FORM, b"#"),
            (flags::ZERO_PADDED,    b"0"),
            (flags::LEFT_ADJUSTED,  b"-"),
            (flags::BLANK,          b" "),
            (flags::SIGN,           b"+")
        ];

        let cases = flags.subsets()
            .map(|s| s.fold((Flags::empty(), Vec::new()), |(fs, ss), &(f, s)| (fs | f, ss + s)));

        for (flags, s) in cases {
            let expected_string = format!("%{}R*f", str::from_utf8(&s[]).unwrap());
            let actual_string = FormatOptions::new(Format::Fixed)
                .with_flags(flags)
                .format_string();
            assert_eq!(expected_string, actual_string);
        }
    }

    #[test]
    fn test_width_and_precision() {
        let f = FormatOptions::new(Format::Fixed);
        assert_eq!("%10R*f", &f.with_width(10).format_string()[]);
        assert_eq!("%.20R*f", &f.with_precision(20.digits()).format_string()[]);
        assert_eq!("%10.20R*f", &f.with_width(10).with_precision(20.digits()).format_string()[]);
    }
}
