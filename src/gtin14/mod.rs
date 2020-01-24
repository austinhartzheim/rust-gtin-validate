//! Performs validation and correction of GTIN-14 codes.

use utils;

/// Errors that make GTIN-14 correction impossible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixError {
    /// The provided string contains non-ASCII characters.
    NonAsciiString,
    /// The provided code was too long to be valid.
    TooLong,
    /// The calculated check-digit did not match the code's check-digit.
    CheckDigitIncorrect,
}

/// Check that a GTIN-14 code is valid by confirming that it is exactly
/// 14 digits in length and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin14;
///
/// assert_eq!(gtin14::check("14567815983469"), true);  // Valid GTIN-14
/// assert_eq!(gtin14::check("1456781598346"), false);  // too short
/// assert_eq!(gtin14::check("14567815983468"), false); // Bad check digit
/// ```
#[cfg(not(feature = "simd"))]
pub fn check(code: &str) -> bool {
    if code.len() != 14 {
        return false;
    }
    if !utils::is_ascii_numeric(code) {
        return false;
    }

    // Calculate and compare check digit
    let bytes = code.as_bytes();
    let check = utils::compute_check_digit(bytes);

    check + 48 == bytes[13]
}

/// Check that a GTIN-14 code is valid by confirming that it is exactly
/// 14 digits in length and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin14;
///
/// assert_eq!(gtin14::check("14567815983469"), true);  // Valid GTIN-14
/// assert_eq!(gtin14::check("1456781598346"), false);  // too short
/// assert_eq!(gtin14::check("14567815983468"), false); // Bad check digit
/// ```
#[cfg(feature = "simd")]
pub fn check(code: &str) -> bool {
    if code.len() != 14 {
        return false;
    }

    let bytes = code.as_bytes();
    let vect = packed_simd::u8x16::new(
        48, 48, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], 48,
    );

    utils::check_ascii_simd(vect) && utils::compute_check_digit_simd(vect - 48) + 48 == bytes[13]
}

/// Attempt to fix an invalid GTIN-14 code by stripping whitespace from
/// the left and right sides and zero-padding the code if it is less
/// than 14 digits in length.
///
/// These corrections fix many common errors introduced by manual data
/// entry and software that treats GTINs as integers rather than
/// strings, thus truncating the leading zeros.
///
/// # Examples
/// ```
/// use gtin_validate::gtin14;
///
/// // Add missing zero, fixing length
/// let result1 = gtin14::fix("04527819983417");
/// assert!(result1.is_ok());
/// assert_eq!(result1.unwrap(), "04527819983417");
///
/// // Remove extra whitespace
/// let result2 = gtin14::fix("04527819983417 ");
/// assert!(result2.is_ok());
/// assert_eq!(result2.unwrap(), "04527819983417");
/// ```
///
/// Here is how you catch errors:
///
/// ```
/// # use gtin_validate::gtin14;
/// match gtin14::fix("14507829283411") {
///   Ok(fixed) => {println!("Fixed GTIN-14: {}", fixed);}
///   Err(_) => {println!("Could not fix GTIN-14");}
/// }
/// ```
pub fn fix(code: &str) -> Result<String, FixError> {
    let mut fixed = code.trim().to_string();

    if !fixed.is_ascii() {
        return Err(FixError::NonAsciiString);
    }
    if fixed.len() > 14 {
        return Err(FixError::TooLong);
    }
    fixed = utils::zero_pad(fixed, 14);
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
        assert_eq!(check("00000000000000"), true);
        assert_eq!(check("17342894127884"), true);
        assert_eq!(check("44889977112244"), true);
    }

    #[test]
    fn check_invalid_length() {
        assert_eq!(check("0000000000000"), false);
        assert_eq!(check("1734289412788"), false);
    }

    #[test]
    fn check_non_ascii() {
        assert_eq!(check("❤"), false);
    }

    #[test]
    fn check_non_numeric() {
        assert_eq!(check("a"), false);
        assert_eq!(check("abcdabcdabcdab"), false); // length 14
        assert_eq!(check("0000000000000a"), false); // invalid check digit
    }

    #[test]
    fn check_invalid_check_digit() {
        assert_eq!(check("00000000000001"), false);
        assert_eq!(check("00000000000002"), false);
        assert_eq!(check("00000000000003"), false);
        assert_eq!(check("00000000000004"), false);
        assert_eq!(check("00000000000005"), false);
        assert_eq!(check("00000000000006"), false);
        assert_eq!(check("00000000000007"), false);
        assert_eq!(check("00000000000008"), false);
        assert_eq!(check("00000000000009"), false);
    }

    #[test]
    fn check_static_data() {
        assert_eq!(check("14567815983469"), true); // Valid GTIN-14
        assert_eq!(check("1456781598346"), false); // too short
        assert_eq!(check("14567815983468"), false); // Bad check digit
    }

    #[test]
    fn fix_non_ascii() {
        assert!(fix("❤").is_err());
    }

    #[test]
    fn fix_too_long() {
        assert_eq!(fix("000000000000000"), Err(FixError::TooLong));
    }

    #[test]
    fn fix_incorrect_check_digit() {
        assert_eq!(fix("17342894127889"), Err(FixError::CheckDigitIncorrect));
    }

    #[test]
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "00000000000000");
        assert_eq!(fix("8987561651112").unwrap(), "08987561651112");
    }

    proptest! {
        #[test]
        fn doesnt_crash(ref s in ".*") {
            check(s);
        }
    }
}
