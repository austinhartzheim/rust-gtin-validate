//! Performs validation and correction of GTIN-12 and UPC-A codes.

use utils;

/// Errors that make GTIN-12 correction impossible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixError {
    /// The provided string contains non-ASCII characters.
    NonAsciiString,
    /// The provided code was too long to be valid.
    TooLong,
    /// The calculated check-digit did not match the code's check-digit.
    CheckDigitIncorrect,
}

/// Check that a UPC-A code is valid by confirming that it is made of
/// exactly 12 digits and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin12;
///
/// assert_eq!(gtin12::check("897854613315"), true);  // Valid GTIN-12
/// assert_eq!(gtin12::check("89785461331"), false);  // Too short
/// assert_eq!(gtin12::check("897854613318"), false); // Bad check digit
/// ```
#[cfg(not(feature = "simd"))]
pub fn check(code: &str) -> bool {
    if code.len() != 12 {
        return false;
    }
    if !utils::is_ascii_numeric(code) {
        return false;
    }

    // Calculate and compare check digit
    let bytes = code.as_bytes();
    let check = utils::compute_check_digit(bytes);

    check + 48 == bytes[11]
}

// Check that a UPC-A code is valid by confirming that it is made of
/// exactly 12 digits and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin12;
///
/// assert_eq!(gtin12::check("897854613315"), true);  // Valid GTIN-12
/// assert_eq!(gtin12::check("89785461331"), false);  // Too short
/// assert_eq!(gtin12::check("897854613318"), false); // Bad check digit
/// ```
#[cfg(feature = "simd")]
pub fn check(code: &str) -> bool {
    if code.len() != 12 {
        return false;
    }

    let bytes = code.as_bytes();
    let vect = packed_simd::u8x16::new(
        48, 48, 48, 48, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        bytes[7], bytes[8], bytes[9], bytes[10], 48,
    );

    utils::check_ascii_simd(vect) && utils::compute_check_digit_simd(vect - 48) + 48 == bytes[11]
}

/// Attempt to fix invalid UPC codes by stripping whitespace from the
/// left and right sides and zero-padding the UPC if it is less than 12
/// digits in length.
///
/// These corrections fix many common errors introduced by manual data
/// entry and software that treats UPCs as integers rather than strings,
/// thus truncating leading zeros.
///
/// # Examples
/// ```
/// use gtin_validate::gtin12;
///
/// // Add missing zero, fixing length:
/// let result1 = gtin12::fix("87248795257");
/// assert!(result1.is_ok());
/// assert_eq!(result1.unwrap(), "087248795257");
///
/// // Remove extra whitespace:
/// let result2 = gtin12::fix("087248795257 ");
/// assert!(result2.is_ok());
/// assert_eq!(result2.unwrap(), "087248795257");
/// ```
///
/// It is also possible to detect errors:
///
/// ```
/// use gtin_validate::gtin12;
/// let result = gtin12::fix("123412341234123"); // UPC too long
/// assert!(result.is_err());
/// ```
pub fn fix(code: &str) -> Result<String, FixError> {
    let mut fixed = code.trim().to_string();

    if !fixed.is_ascii() {
        return Err(FixError::NonAsciiString);
    }
    if fixed.len() > 12 {
        return Err(FixError::TooLong);
    }
    fixed = utils::zero_pad(fixed, 12);
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
        assert_eq!(check(&"000000000000"), true);
    }

    #[test]
    fn check_invalid_length() {
        assert_eq!(check("000"), false);
    }

    #[test]
    fn check_non_ascii() {
        assert_eq!(check("❤"), false);
    }

    #[test]
    fn check_non_numeric() {
        assert_eq!(check("a"), false);
        assert_eq!(check("abcdabcdabcd"), false); // length 12
        assert_eq!(check("00000000000a"), false); // invalid check digit
    }

    #[test]
    fn check_invalid_check_digit() {
        assert_eq!(check("000000000001"), false);
        assert_eq!(check("000000000002"), false);
        assert_eq!(check("000000000003"), false);
        assert_eq!(check("000000000004"), false);
        assert_eq!(check("000000000005"), false);
        assert_eq!(check("000000000006"), false);
        assert_eq!(check("000000000007"), false);
        assert_eq!(check("000000000008"), false);
        assert_eq!(check("000000000009"), false);
    }

    #[test]
    fn check_static_data() {
        assert_eq!(check("000000000000"), true);
        assert_eq!(check("123456789012"), true);
        assert_eq!(check("123456789013"), false);
        assert_eq!(check("999999999993"), true);
        assert_eq!(check("999999999999"), false);
    }

    #[test]
    fn fix_non_ascii() {
        assert!(fix("❤").is_err());
    }

    #[test]
    fn fix_too_long() {
        assert_eq!(fix("0000000000000"), Err(FixError::TooLong));
    }

    #[test]
    fn fix_incorrect_check_digit() {
        assert_eq!(fix("123456789013"), Err(FixError::CheckDigitIncorrect));
    }

    #[test]
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "000000000000");
    }

    proptest! {
        #[test]
        fn doesnt_crash(ref s in ".*") {
            check(s);
        }
    }
}
