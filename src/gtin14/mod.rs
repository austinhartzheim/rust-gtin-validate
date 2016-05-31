use std::ascii::AsciiExt;
use utils;

#[derive(Debug)]
pub enum FixError {
    NonAsciiString,
    TooLong,
    CheckDigitIncorrect
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
pub fn check(upc: &str) -> bool {
    if upc.is_ascii() == false {
        return false;
    }
    if upc.len() != 14 {
        return false;
    }

    let bytes = upc.as_bytes();
    if !utils::is_number(bytes, 14) {
        return false;
    }

    // Calculate and compare check digit
    let check = utils::compute_check_digit(bytes, 14);
    if check != bytes[13] - 48 {
        return false;
    }

    return true;
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
    let mut fixed = code.trim_left().trim_right().to_string();

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

    return Ok(fixed);
}

#[cfg(test)]
mod tests {
    use super::check;
    use super::fix;

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
    fn fix_non_ascii() {
        assert!(fix("❤").is_err());
    }

    #[test]
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "00000000000000");
        assert_eq!(fix("8987561651112").unwrap(), "08987561651112");
    }
}
