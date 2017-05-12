/// Compute the check digit for a GTIN code as described on the
/// [GS1 website](http://www.gs1.org/how-calculate-check-digit-manually)
pub fn compute_check_digit(bytes: &[u8], size: usize) -> u8 {
    let mut even: u16 = 0;
    let mut odd: u16 = 0;
    let mut check: u8;
    let mut curr: u8;

    // Read GTIN in reverse because the even/odd columns are defined
    // right-to-left, with the last non-check-digit column being odd.
    for i in 2..size + 1 {
        curr = bytes[size - i] - 48;
        if i % 2 == 0 {
            odd += curr as u16;
        } else {
            even += curr as u16;
        }
    }

    check = ((3 * odd + even) % 10) as u8;
    if check > 0 {
        check = 10 - check;
    }

    check
}

/// Add zeros to the left side of a string so that it matches the
/// desired length.
///
/// If the string is longer than the desired length, it is returned,
/// without modification.
pub fn zero_pad(upc: String, size: usize) -> String {
    if upc.len() >= size {
        return upc;
    }
    let mut padded = String::with_capacity(size);
    for _ in 0..size - upc.len() {
        padded.push('0');
    }
    padded.push_str(&upc);

    padded
}


/// Check that the string is made entirely of ASCII digits. This
/// function will not accept other number-related characters such as
/// a decimal or negative sign as those are invalid in GTINs.
pub fn is_number(bytes: &[u8], length: usize) -> bool {
    const ASCII_DIGIT_MIN: u8 = 48;
    const ASCII_DIGIT_MAX: u8 = 48 + 9;

    for byte in bytes.iter().take(length) {
        if byte < &ASCII_DIGIT_MIN || byte > &ASCII_DIGIT_MAX {
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::compute_check_digit;
    use super::zero_pad;
    use super::is_number;

    #[test]
    fn compute_check_digit_static_data() {
        assert_eq!(compute_check_digit("000000000000".as_bytes(), 12), 0);
        assert_eq!(compute_check_digit("123456789012".as_bytes(), 12), 2);
        assert_eq!(compute_check_digit("123456789081".as_bytes(), 12), 1);
        assert_eq!(compute_check_digit("036000291452".as_bytes(), 12), 2);
        assert_eq!(compute_check_digit("999999999993".as_bytes(), 12), 3);
        assert_eq!(compute_check_digit("0000000000000".as_bytes(), 13), 0);
        assert_eq!(compute_check_digit("1234123412344".as_bytes(), 13), 4);
        assert_eq!(compute_check_digit("9249874313545".as_bytes(), 13), 5);
        assert_eq!(compute_check_digit("00000000000000".as_bytes(), 14), 0);
        assert_eq!(compute_check_digit("01010101010104".as_bytes(), 14), 4);
        assert_eq!(compute_check_digit("92498743135447".as_bytes(), 14), 7);
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
    fn is_number_valid_numbers() {
        assert_eq!(is_number("0".as_bytes(), 1), true);
        assert_eq!(is_number("1".as_bytes(), 1), true);
        assert_eq!(is_number("00".as_bytes(), 2), true);
    }

    #[test]
    fn is_number_invalid_numbers() {
        assert_eq!(is_number("a".as_bytes(), 1), false);
        assert_eq!(is_number("0a".as_bytes(), 2), false);
        assert_eq!(is_number("-1".as_bytes(), 2), false);
        assert_eq!(is_number("4.2".as_bytes(), 3), false);
    }
}
