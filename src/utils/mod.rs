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


#[cfg(test)]
mod tests {
    use super::zero_pad;

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
}
