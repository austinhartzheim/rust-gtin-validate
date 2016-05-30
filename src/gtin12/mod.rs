//! # gtin12
//!
//! Performs validation and correction of UPC strings.

use std::ascii::AsciiExt;
use utils;


#[derive(Debug)]
pub enum UpcAFixError {
    NonAsciiString,
    TooLong,
    CheckDigitIncorrect
}


/// Computes the check digit of a UPC-A code.
/// Assumes that all bytes in `upc` are valid ASCII digits.
fn compute_upca_check_digit(upc: &[u8]) -> u8 {
    let mut even: u8 = 0;
    let mut odd: u8 = 0;
    let mut check: u8;
    let mut curr: u8;
    
    for i in 0..11 {
        curr = upc[i] - 48;
        
        if i % 2 == 0 {
            odd += curr;
        } else {
            even += curr;
        }
    }

    // check = (10 - (((3*odd + even) as i16) % 10)) as u8;
    check = (3*odd + even) % 10;
    if check > 0 {
        check = 10 - check;
    }
    return check;
}


/// Check that a UPC-A code is valid by confirming that it is made of
/// exactly 12 digits and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin12;
///
/// assert_eq!(gtin12::check("000000000000"), true);  // valid UPC
/// assert_eq!(gtin12::check("00000000000"), false);  // invalid, UPC too short
/// assert_eq!(gtin12::check("000000000001"), false); // invalid, wrong check digit
/// ```
pub fn check(upc: &str) -> bool {
    let check: u8;

    // Check that input is ASCII with length 12
    if !upc.is_ascii() {
        return false;
    }
    if upc.len() != 12 {
        return false;
    }

    let bytes = upc.as_bytes();
    if !utils::is_number(bytes, 12) {
        return false;
    }
    
    check = compute_upca_check_digit(bytes);

    // Calculate and compare check digit 
    if check != bytes[11] - 48 {
        return false;
    }
    
    return true;
}

/// Attempt to fix invalid UPC codes by stripping whitespace from the
/// left and right sides and zero-padding the UPC if it is less than 12
/// digits in length.
///
/// These corrections fix many common errors introduced by manual data
/// entry and software that treats UPCs as integers rather than strigns,
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
pub fn fix(upc: &str) -> Result<String, UpcAFixError> {
    let mut fixed = upc.trim_left().trim_right().to_string();

    if upc.is_ascii() == false {
        return Err(UpcAFixError::NonAsciiString);
    }
    if fixed.len() > 12 {
        return Err(UpcAFixError::TooLong);
    }
    fixed = utils::zero_pad(fixed, 12);
    if !check(&fixed) {
        return Err(UpcAFixError::CheckDigitIncorrect);
    }
    
    return Ok(fixed);
}

#[cfg(test)]
mod tests {
    use super::compute_upca_check_digit;
    use super::check;
    use super::fix;

    #[test]
    fn compute_upca_check_digit_static_data() {
        assert_eq!(compute_upca_check_digit("000000000000".as_bytes()), 0);
        assert_eq!(compute_upca_check_digit("123456789012".as_bytes()), 2);
        assert_eq!(compute_upca_check_digit("123456789081".as_bytes()), 1);
        assert_eq!(compute_upca_check_digit("036000291452".as_bytes()), 2);
        assert_eq!(compute_upca_check_digit("999999999993".as_bytes()), 3);
    }
    
    #[test]
    fn check_valid() {
        assert!(check(&"000000000000") == true);
    }

    #[test]
    fn check_invalid_length() {
        assert!(check("000") == false);
    }

    #[test]
    fn check_non_ascii() {
        assert!(check("❤") == false);
    }

    #[test]
    fn check_non_numeric() {
        assert!(check("a") == false);
        assert!(check("abcdabcdabcd") == false); // length 12
        assert!(check("00000000000a") == false); // invalid check digit
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
    fn fix_needs_zero_padding() {
        assert!(fix("0").is_ok());
        assert_eq!(fix("0").unwrap(), "000000000000");
    }
}
