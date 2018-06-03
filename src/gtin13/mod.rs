//! Performs validation and correction of GTIN-13 and EAN-13 codes.

use utils;

/// Errors that make GTIN-13 correction impossible.
#[derive(Debug)]
pub enum FixError {
    /// The provided string contains non-ASCII characters.
    NonAsciiString,
    /// The provided code was too long to be valid.
    TooLong,
    /// The calculated check-digit did not match the code's check-digit.
    CheckDigitIncorrect,
}

/// Check that a GTIN-13 code is valid by checking the length (should be
/// exactly 13 digits) and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin13;
///
/// assert_eq!(gtin13::check("1498279802125"), true);  // Valid GTIN-13
/// assert_eq!(gtin13::check("468712378699"), false);  // Too short
/// assert_eq!(gtin13::check("1498279802124"), false); // Bad check digit
/// ```
pub fn check(code: &str) -> bool {
    if !code.is_ascii() {
        return false;
    }
    if code.len() != 13 {
        return false;
    }

    let bytes = code.as_bytes();
    if !utils::is_number(bytes, 13) {
        return false;
    }

    // Calculate and compare check digit
    let check = utils::compute_check_digit(bytes, 13);
    if check != bytes[12] - 48 {
        return false;
    }

    true
}

/// Attempt to fix an invalid GTIN-13 code by stripping whitespace from
/// the let and right sides and zero-padding the code if it is less than
/// 13 digits in length.
///
/// These corrections fix many common errors introduced by manual data
/// entry and software that treats GTINs as integers rather than strings,
/// thus truncating the leading zeros.
///
/// # Examples
/// ```
/// use gtin_validate::gtin13;
///
/// // Add missing zero, fixing length
/// let result1 = gtin13::fix("495205944325");
/// assert!(result1.is_ok());
/// assert_eq!(result1.unwrap(), "0495205944325");
///
/// // Remove extra whitespace
/// let result2 = gtin13::fix("4823011492925 ");
/// assert!(result2.is_ok());
/// assert_eq!(result2.unwrap(), "4823011492925");
/// ```
///
/// Here is how you catch errors:
///
/// ```
/// # use gtin_validate::gtin13;
/// match gtin13::fix("04567432178913") {
///   Ok(fixed) => {println!("{} is OK!", fixed);}
///   Err(_) => {println!("UPC is invalid");}
/// }
/// ```

pub fn fix(code: &str) -> Result<String, FixError> {
    let mut fixed = code.trim_left().trim_right().to_string();

    if !fixed.is_ascii() {
        return Err(FixError::NonAsciiString);
    }
    if fixed.len() > 13 {
        return Err(FixError::TooLong);
    }
    fixed = utils::zero_pad(fixed, 13);
    if !check(&fixed) {
        return Err(FixError::CheckDigitIncorrect);
    }

    Ok(fixed)
}

#[cfg(test)]
mod tests {
    use super::check;
    use super::fix;

    #[test]
    fn check_valid() {
        assert_eq!(check("0000000000000"), true);
        assert_eq!(check("8845791354268"), true);
        assert_eq!(check("0334873614126"), true);
    }

    #[test]
    fn check_invalid_length() {
        assert_eq!(check("000"), false);
        assert_eq!(check("00000000000000"), false);
    }

    #[test]
    fn check_non_ascii() {
        assert_eq!(check("❤"), false);
    }

    #[test]
    fn check_non_numeric() {
        assert_eq!(check("a"), false);
        assert_eq!(check("abcdabcdabcda"), false); // length 13
        assert_eq!(check("000000000000a"), false); // invalid check digit
    }

    #[test]
    fn check_invalid_check_digit() {
        assert_eq!(check("0000000000001"), false);
        assert_eq!(check("0000000000002"), false);
        assert_eq!(check("0000000000003"), false);
        assert_eq!(check("0000000000004"), false);
        assert_eq!(check("0000000000005"), false);
        assert_eq!(check("0000000000006"), false);
        assert_eq!(check("0000000000007"), false);
        assert_eq!(check("0000000000008"), false);
        assert_eq!(check("0000000000009"), false);
    }

    #[test]
    fn check_static_data() {
        assert_eq!(check("0000000000000"), true);
        assert_eq!(check("0123456789012"), true);
        assert_eq!(check("0123456789013"), false);
        assert_eq!(check("0999999999993"), true);
        assert_eq!(check("0999999999999"), false);
        assert_eq!(check("4459121265748"), true);
        assert_eq!(check("4459121265747"), false);
    }

    #[test]
    fn fix_non_ascii() {
        assert!(fix("❤").is_err());
    }

    #[test]
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "0000000000000");
        assert_eq!(fix("123012301238").unwrap(), "0123012301238");
    }
}
