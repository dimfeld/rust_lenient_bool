//! This module provides a single type, `LenientBool`, which implements `FromStr` to convert
//! a string into a boolean. It is more accepting of various boolean representations than
//! the standard bool function, performing case-insensitive matches
//! against `true`, `false`, `t`, and `f`, and also matches `0` and `1`.
//!
//! # Errors
//! Any string not matching the above list will return a `LenientBoolError`.
//!
//! # Examples
//!
//! ```
//! extern crate lenient_bool;
//! use lenient_bool::LenientBool;
//!
//! fn main() {
//!     let b : bool = "1".parse::<LenientBool>().unwrap().into();
//!     assert_eq!(b, true);
//! }
//! ```

use std::ops::Deref;
use std::ascii::AsciiExt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct LenientBool(bool);

#[derive(Debug, PartialEq, Eq)]
pub struct LenientBoolError(());

impl FromStr for LenientBool {
    type Err = LenientBoolError;
    fn from_str(s: &str) -> Result<Self, LenientBoolError> {
        if s.eq_ignore_ascii_case("true") { Ok(LenientBool(true)) }
        else if s.eq_ignore_ascii_case("false") { Ok(LenientBool(false)) }
        else if s.eq_ignore_ascii_case("t") { Ok(LenientBool(true)) }
        else if s.eq_ignore_ascii_case("f") { Ok(LenientBool(false)) }
        else if s == "1" { Ok(LenientBool(true)) }
        else if s == "0" { Ok(LenientBool(false)) }
        else { Err(LenientBoolError(())) }
    }
}

impl From<LenientBool> for bool {
    fn from(n : LenientBool) -> bool { n.0 }
}

impl Deref for LenientBool {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn parse_f() {
        assert_eq!("f".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_f_upper() {
        assert_eq!("F".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_false() {
        assert_eq!("false".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_false_cap() {
        assert_eq!("False".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_false_upper() {
        assert_eq!("FALSE".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_0() {
        assert_eq!("0".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_t() {
        assert_eq!("t".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_t_upper() {
        assert_eq!("T".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_true() {
        assert_eq!("true".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_true_cap() {
        assert_eq!("True".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_true_upper() {
        assert_eq!("TRUE".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_1() {
        assert_eq!("1".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    mod errors {
        use super::*;

        #[test]
        fn parse_empty_err() {
            assert_eq!("".parse::<LenientBool>(), Err(LenientBoolError(())));
        }

        #[test]
        fn parse_bad_input_err() {
            assert_eq!("abc".parse::<LenientBool>(), Err(LenientBoolError(())));
        }
    }

    mod conversion {
        use super::*;

        #[test]
        fn from() {
            let lb = "true".parse::<LenientBool>();
            assert!(lb.is_ok());
            let lb = lb.unwrap();

            // The preferred way.
            let b : bool = lb.into();
            assert_eq!(b, true);

            let lb = "false".parse::<LenientBool>();
            assert!(lb.is_ok());
            let lb = lb.unwrap();

            // The preferred way.
            let b : bool = lb.into();
            assert_eq!(b, false);
        }

        #[test]
        fn deref() {
            let lb = "true".parse::<LenientBool>();
            assert!(lb.is_ok());
            let lb = lb.unwrap();

            assert_eq!(*lb, true);

            let lb = "false".parse::<LenientBool>();
            assert!(lb.is_ok());
            let lb = lb.unwrap();

            assert_eq!(*lb, false);
        }
    }

}
