//! Performs validation and correction of GTIN-8 codes.

use std::error::Error;
use std::fmt;
use utils;

/// Errors that make GTIN-8 correction impossible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixError {
    /// The provided string contains non-ASCII characters.
    NonAsciiString,
    /// The provided code was too long to be valid.
    TooLong,
    /// The calculated check-digit did not match the code's check-digit.
    CheckDigitIncorrect,
}

impl Error for FixError {}

impl fmt::Display for FixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixError::NonAsciiString => {
                write!(f, "the provided string contains non-ASCII characters")
            }
            FixError::TooLong => write!(f, "the provided code was too long too be valid"),
            FixError::CheckDigitIncorrect => write!(
                f,
                "the calculated check-digit did not match the code's check-digit"
            ),
        }
    }
}

/// Check that a GTIN-8 code is valid by confirming that it is exactly
/// 8 digits in length and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin8;
///
/// assert_eq!(gtin8::check("14567810"), true);  // Valid GTIN-8
/// assert_eq!(gtin8::check("1456781"), false);  // too short
/// assert_eq!(gtin8::check("14567811"), false); // Bad check digit
/// ```
pub fn check(code: &str) -> bool {
    if code.len() != 8 {
        return false;
    }
    if !utils::is_ascii_numeric(code) {
        return false;
    }

    // Calculate and compare check digit
    let bytes = code.as_bytes();
    let check = utils::compute_check_digit(bytes);
    if check != bytes[7] - 48 {
        return false;
    }

    true
}

/// Attempt to fix an invalid GTIN-8 code by stripping whitespace from
/// the left and right sides and zero-padding the code if it is less
/// than 8 digits in length.
///
/// These corrections fix many common errors introduced by manual data
/// entry and software that treats GTINs as integers rather than
/// strings, thus truncating the leading zeros.
///
/// # Examples
/// ```
/// use gtin_validate::gtin8;
///
/// // Add missing zero, fixing length
/// let result1 = gtin8::fix("5766796");
/// assert!(result1.is_ok());
/// assert_eq!(result1.unwrap(), "05766796");
///
/// // Remove extra whitespace
/// let result2 = gtin8::fix("05766796 ");
/// assert!(result2.is_ok());
/// assert_eq!(result2.unwrap(), "05766796");
/// ```
///
/// Here is how you catch errors:
///
/// ```
/// # use gtin_validate::gtin8;
/// match gtin8::fix("14567811") {
///   Ok(fixed) => {println!("Fixed GTIN-14: {}", fixed);}
///   Err(_) => {println!("Could not fix GTIN-14");}
/// }
/// ```
pub fn fix(code: &str) -> Result<String, FixError> {
    let mut fixed = code.trim().to_string();

    if !fixed.is_ascii() {
        return Err(FixError::NonAsciiString);
    }
    if fixed.len() > 8 {
        return Err(FixError::TooLong);
    }
    fixed = utils::zero_pad(fixed, 8);
    if !check(&fixed) {
        return Err(FixError::CheckDigitIncorrect);
    }

    Ok(fixed)
}

#[cfg(test)]
mod tests {
    use super::check;
    use super::fix;
    use super::FixError;

    #[test]
    fn check_valid() {
        assert_eq!(check("00000000"), true);
        assert_eq!(check("49137712"), true);
        assert_eq!(check("44196318"), true);
    }

    #[test]
    fn check_invalid_length() {
        assert_eq!(check("0000000"), false); // too short
        assert_eq!(check("734289412"), false); // too long
    }

    #[test]
    fn check_non_ascii() {
        assert_eq!(check("❤"), false);
    }

    #[test]
    fn check_non_numeric() {
        assert_eq!(check("a"), false);
        assert_eq!(check("abcdabcd"), false); // length 8
        assert_eq!(check("0000000a"), false); // invalid check digit
    }

    #[test]
    fn check_invalid_check_digit() {
        assert_eq!(check("00000001"), false);
        assert_eq!(check("00000002"), false);
        assert_eq!(check("00000003"), false);
        assert_eq!(check("00000004"), false);
        assert_eq!(check("00000005"), false);
        assert_eq!(check("00000006"), false);
        assert_eq!(check("00000007"), false);
        assert_eq!(check("00000008"), false);
        assert_eq!(check("00000009"), false);
    }

    #[test]
    fn check_static_data() {
        assert_eq!(check("14567810"), true); // Valid GTIN-8
        assert_eq!(check("1456781"), false); // too short
        assert_eq!(check("14567811"), false); // Bad check digit
    }

    #[test]
    fn fix_non_ascii() {
        assert!(fix("❤").is_err());
    }

    #[test]
    fn fix_too_long() {
        assert_eq!(fix("000000000"), Err(FixError::TooLong));
    }

    #[test]
    fn fix_incorrect_check_digit() {
        assert_eq!(fix("14567813"), Err(FixError::CheckDigitIncorrect));
    }

    #[test]
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "00000000");
        assert_eq!(fix("9944220").unwrap(), "09944220");
    }

    proptest! {
        #[test]
        fn doesnt_crash(ref s in ".*") {
            check(s);
        }
    }
}
