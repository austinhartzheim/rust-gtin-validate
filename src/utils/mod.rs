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
    for _ in 0 .. size - upc.len() {
        padded.push('0');
    }
    padded.push_str(&upc);
    return padded;
}


/// Check that the string is made entirely of ASCII digits. This
/// function will not accept other number-related characters such as
/// a decimal or negative sign as those are invalid in GTINs.
pub fn is_number(bytes: &[u8], length: usize) -> bool {
    for i in 0 .. length {
        if bytes[i] < 48 || bytes[i] > 48 + 9 {
            return false;
        }
    }
    return true;
}


#[cfg(test)]
mod tests {
    use super::zero_pad;
    use super::is_number;

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
