//! # upc_validate
//!
//! Performs validation and correction of UPC strings.

use std::ascii::AsciiExt;


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

/// Add zeros to the left side of a string so that it matches the
/// desired length.
///
/// If the string is longer than the desired length, it is returned,
/// without modification.
fn zero_pad(upc: String, size: usize) -> String {
    if upc.len() >= size {
        return upc;
    }
    let mut padded = String::with_capacity(size);
    for _ in 0 .. size - upc.len() {
        padded.push('0');
    }
    padded.push_str(&upc);
    return padded;
}

/// Check that a UPC-A code is valid by confirming that it is made of
/// exactly 12 digits and that the check-digit is correct.
///
/// # Examples
/// ```
/// use upc_validate::check_upca;
///
/// assert_eq!(check_upca("000000000000"), true);  // valid UPC
/// assert_eq!(check_upca("00000000000"), false);  // invalid, UPC too short
/// assert_eq!(check_upca("000000000001"), false); // invalid, wrong check digit
/// ```
pub fn check_upca(upc: &str) -> bool {
    let check: u8;

    // Check that input is ASCII with length 12
    if upc.is_ascii() == false {
        return false;
    }
    if upc.len() != 12 {
        return false;
    }

    let bytes = upc.as_bytes();
    for i in 0 .. 12 {
        // Checking that all bytes are ASCII digits
        if bytes[i] < 48 || bytes[i] > 48 + 9 {
            return false;
        }
    }
    
    check = compute_upca_check_digit(bytes);

    // Calculate and compare check digit 
    if bytes[11] < 48 || bytes[11] > 48 + 9 {
        return false;
    }
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
/// use upc_validate::fix_upca;
///
/// // Add missing zero, fixing length:
/// let result1 = fix_upca("87248795257");
/// assert!(result1.is_ok());
/// assert_eq!(result1.unwrap(), "087248795257");
///
/// // Remove extra whitespace:
/// let result2 = fix_upca("087248795257 ");
/// assert!(result2.is_ok());
/// assert_eq!(result2.unwrap(), "087248795257");
/// ```
///
/// It is also possible to detect errors:
///
/// ```
/// use upc_validate::fix_upca;
/// let result = fix_upca("123412341234123"); // UPC too long
/// assert!(result.is_err());
/// ```
pub fn fix_upca(upc: &str) -> Result<String, &str> {
    let mut fixed = upc.trim_left().trim_right().to_string();

    if upc.is_ascii() == false {
        return Err("Cannot operate on non-ASCII data");
    }
    if fixed.len() > 12 {
        return Err("Cannot fix UPC-A. Length is longer than 12.");
    }
    fixed = zero_pad(fixed, 12);
    if !check_upca(&fixed) {
        return Err("Final validation failed");
    }
    
    return Ok(fixed);
}

#[cfg(test)]
mod tests {
    use super::compute_upca_check_digit;
    use super::zero_pad;
    use super::check_upca;
    use super::fix_upca;

    #[test]
    fn compute_upca_check_digit_static_data() {
        assert_eq!(compute_upca_check_digit("000000000000".as_bytes()), 0);
        assert_eq!(compute_upca_check_digit("123456789012".as_bytes()), 2);
        assert_eq!(compute_upca_check_digit("123456789081".as_bytes()), 1);
        assert_eq!(compute_upca_check_digit("036000291452".as_bytes()), 2);
        assert_eq!(compute_upca_check_digit("999999999993".as_bytes()), 3);
    }


    #[test]
    fn zero_pad_static_data() {
        assert_eq!(zero_pad("hello".to_string(), 6), "0hello".to_string());
        assert_eq!(zero_pad("".to_string(), 0), "".to_string());
    }

    #[test]
    fn zero_pad_string_longer_than_desired_length() {
        assert_eq!(zero_pad("hello".to_string(), 3), "hello".to_string());
        assert_eq!(zero_pad("hello".to_string(), 0), "hello".to_string());
    }
    
    #[test]
    fn check_upca_valid() {
        assert!(check_upca(&"000000000000") == true);
    }

    #[test]
    fn check_upca_invalid_length() {
        assert!(check_upca("000") == false);
    }

    #[test]
    fn check_upca_non_ascii() {
        assert!(check_upca("❤") == false);
    }

    #[test]
    fn check_upca_invalid_check_digit() {
        assert_eq!(check_upca("000000000001"), false);
        assert_eq!(check_upca("000000000002"), false);
        assert_eq!(check_upca("000000000003"), false);
        assert_eq!(check_upca("000000000004"), false);
        assert_eq!(check_upca("000000000005"), false);
        assert_eq!(check_upca("000000000006"), false);
        assert_eq!(check_upca("000000000007"), false);
        assert_eq!(check_upca("000000000008"), false);
        assert_eq!(check_upca("000000000009"), false);
    }

    #[test]
    fn check_upca_static_data() {
        assert_eq!(check_upca("000000000000"), true);
        assert_eq!(check_upca("123456789012"), true);
        assert_eq!(check_upca("123456789013"), false);
        assert_eq!(check_upca("999999999993"), true);
        assert_eq!(check_upca("999999999999"), false);
    }


    #[test]
    fn fix_upca_non_ascii() {
        assert!(fix_upca("❤").is_err());
    }

    #[test]
    fn fix_upca_needs_zero_padding() {
        assert!(fix_upca("0").is_ok());
        assert_eq!(fix_upca("0").unwrap(), "000000000000");
    }
}
