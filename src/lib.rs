//! This module provides a single type, `LenientBool`, which implements `FromStr` to convert
//! a string into a boolean. It is more accepting of various boolean representations than
//! the standard bool function, performing case-insensitive matches
//! against `true`, `false`, `t`, and `f`, `yes`, `no`, `y`, `n`, `0`, and `1`.
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

use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct LenientBool(pub bool);

#[derive(Debug, PartialEq, Eq)]
pub struct LenientBoolError(());

impl FromStr for LenientBool {
    type Err = LenientBoolError;
    fn from_str(s: &str) -> Result<Self, LenientBoolError> {
        if s.eq_ignore_ascii_case("true")
        || s.eq_ignore_ascii_case("t")
        || s.eq_ignore_ascii_case("yes")
        || s.eq_ignore_ascii_case("y")
        || s == "1" {
            Ok(LenientBool(true))
        } else
        if s.eq_ignore_ascii_case("false")
        || s.eq_ignore_ascii_case("f")
        || s.eq_ignore_ascii_case("no")
        || s.eq_ignore_ascii_case("n")
        || s == "0" {
            Ok(LenientBool(false))
        } else {
            Err(LenientBoolError(()))
        }
    }
}

impl From<LenientBool> for bool {
    fn from(n: LenientBool) -> bool { n.0 }
}

impl From<bool> for LenientBool {
    fn from(b: bool) -> bool { LenientBool(b) }
}

impl AsRef<bool> for LenientBool {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

impl AsMut<bool> for LenientBool {
    fn as_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}

impl Deref for LenientBool {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LenientBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
    fn parse_n() {
        assert_eq!("n".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_n_upper() {
        assert_eq!("N".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_no() {
        assert_eq!("no".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_no_cap() {
        assert_eq!("No".parse::<LenientBool>(), Ok(LenientBool(false)));
    }

    #[test]
    fn parse_no_upper() {
        assert_eq!("NO".parse::<LenientBool>(), Ok(LenientBool(false)));
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
    fn parse_y() {
        assert_eq!("y".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_y_upper() {
        assert_eq!("Y".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_yes() {
        assert_eq!("yes".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_yes_cap() {
        assert_eq!("Yes".parse::<LenientBool>(), Ok(LenientBool(true)));
    }

    #[test]
    fn parse_yes_upper() {
        assert_eq!("YES".parse::<LenientBool>(), Ok(LenientBool(true)));
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
