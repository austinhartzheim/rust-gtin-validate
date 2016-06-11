//! Performs validation and correction of GTIN-8 codes.

use std::ascii::AsciiExt;
use utils;


/// Errors that make GTIN-8 correction impossible.
#[derive(Debug)]
pub enum FixError {
    /// The provided string contains non-ASCII characters.
    NonAsciiString,
    /// The provided code was too long to be valid.
    TooLong,
    /// The calculated check-digit did not match the code's check-digit.
    CheckDigitIncorrect
}

/// Check that a GTIN-8 code is valid by confirming that it is exactly
/// 8 digits in length and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin8;
///
/// assert_eq!(gtin8::check("14567810"), true);  // Valid GTIN-14
/// assert_eq!(gtin8::check("1456781"), false);  // too short
/// assert_eq!(gtin8::check("14567811"), false); // Bad check digit
/// ```
pub fn check(code: &str) -> bool {
    if code.is_ascii() == false {
        return false;
    }
    if code.len() != 8 {
        return false;
    }

    let bytes = code.as_bytes();
    if !utils::is_number(bytes, 8) {
        return false;
    }

    // Calculate and compare check digit
    let check = utils::compute_check_digit(bytes, 8);
    if check != bytes[7] - 48 {
        return false;
    }

    return true;
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
    let mut fixed = code.trim_left().trim_right().to_string();

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

    return Ok(fixed);
}

#[cfg(test)]
mod tests {
    use super::check;
    use super::fix;

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
    fn fix_non_ascii() {
        assert!(fix("❤").is_err());
    }

    #[test]
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "00000000");
        assert_eq!(fix("9944220").unwrap(), "09944220");
    }
}
