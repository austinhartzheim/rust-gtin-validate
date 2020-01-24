#[cfg(feature = "simd")]
use std::ops::BitOr;

/// Compute the check digit for a GTIN code as described on the
/// [GS1 website](http://www.gs1.org/how-calculate-check-digit-manually)
/// This function assumes that the passed in bytes are already
/// guaranteed to be ASCII digits (eg, by calling is_ascii_numeric).
#[cfg(not(feature = "simd"))]
pub fn compute_check_digit(bytes: &[u8]) -> u8 {
    let mut even: u16 = 0;
    let mut odd: u16 = 0;
    let mut check: u8;
    let mut curr: u8;

    // Read GTIN in reverse because the even/odd columns are defined
    // right-to-left, with the last non-check-digit column being odd.
    for i in 2..bytes.len() + 1 {
        curr = bytes[bytes.len() - i] - 48;
        if i % 2 == 0 {
            odd += u16::from(curr);
        } else {
            even += u16::from(curr);
        }
    }

    check = ((3 * odd + even) % 10) as u8;
    if check > 0 {
        check = 10 - check;
    }

    check
}

#[cfg(feature = "simd")]
#[inline]
pub fn check_ascii_simd(vect: packed_simd::u8x16) -> bool {
    static GT: packed_simd::u8x16 = packed_simd::u8x16::splat(57);
    static LT: packed_simd::u8x16 = packed_simd::u8x16::splat(48);
    let res = vect.gt(GT).bitor(vect.lt(LT));
    res.none()
}

#[cfg(feature = "simd")]
#[inline]
pub fn compute_check_digit_simd(mut vect: packed_simd::u8x16) -> u8 {
    static MUL: packed_simd::u8x16 =
        packed_simd::u8x16::new(3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1);
    vect *= MUL;
    let check = vect.wrapping_sum() % 10;
    if check > 0 {
        10 - check
    } else {
        check
    }
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

#[inline]
#[cfg(not(feature = "simd"))]
pub fn is_ascii_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
#[cfg(not(feature = "simd"))]
mod tests {
    use super::compute_check_digit;
    use super::is_ascii_numeric;
    use super::zero_pad;

    use proptest::prelude::*;

    #[test]
    fn compute_check_digit_static_data() {
        assert_eq!(compute_check_digit("000000000000".as_bytes()), 0);
        assert_eq!(compute_check_digit("123456789012".as_bytes()), 2);
        assert_eq!(compute_check_digit("123456789081".as_bytes()), 1);
        assert_eq!(compute_check_digit("036000291452".as_bytes()), 2);
        assert_eq!(compute_check_digit("999999999993".as_bytes()), 3);
        assert_eq!(compute_check_digit("0000000000000".as_bytes()), 0);
        assert_eq!(compute_check_digit("1234123412344".as_bytes()), 4);
        assert_eq!(compute_check_digit("9249874313545".as_bytes()), 5);
        assert_eq!(compute_check_digit("00000000000000".as_bytes()), 0);
        assert_eq!(compute_check_digit("01010101010104".as_bytes()), 4);
        assert_eq!(compute_check_digit("92498743135447".as_bytes()), 7);
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
    fn is_ascii_numeric_valid_numbers() {
        assert_eq!(is_ascii_numeric("0"), true);
        assert_eq!(is_ascii_numeric("1"), true);
        assert_eq!(is_ascii_numeric("00"), true);
        assert_eq!(is_ascii_numeric("99"), true);
    }

    #[test]
    fn is_ascii_numeric_invalid_numbers() {
        assert_eq!(is_ascii_numeric("a"), false);
        assert_eq!(is_ascii_numeric("0a"), false);
        assert_eq!(is_ascii_numeric("-1"), false);
        assert_eq!(is_ascii_numeric("4.2"), false);
    }

    proptest! {
        #[test]
        fn compute_check_digit_doesnt_crash(ref code in "[0-9]*") {
            assert!(compute_check_digit(code.as_bytes()) < 10);
        }
    }
}

#[cfg(test)]
#[cfg(feature = "simd")]
mod simd_tests {
    use super::check_ascii_simd;
    use super::compute_check_digit_simd;

    #[test]
    fn simd_compute_check_digit_static_data() {
        assert_eq!(compute_check_digit_simd(packed_simd::u8x16::splat(0)), 0);
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 0
            )),
            2
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 8, 0
            )),
            1
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 0, 0, 3, 6, 0, 0, 0, 2, 9, 1, 4, 5, 0
            )),
            2
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 0, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 0
            )),
            3
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 9, 2, 4, 9, 8, 7, 4, 3, 1, 3, 5, 4, 0
            )),
            5
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0
            )),
            4
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 9, 2, 4, 9, 8, 7, 4, 3, 1, 3, 5, 4, 4, 0
            )),
            7
        );
        assert_eq!(
            compute_check_digit_simd(packed_simd::u8x16::new(
                0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0
            )),
            4
        );
    }

    #[test]
    fn simd_is_ascii_numeric_valid_numbers() {
        assert!(check_ascii_simd(packed_simd::u8x16::splat(48)));
        assert!(check_ascii_simd(packed_simd::u8x16::splat(49)));
        assert!(check_ascii_simd(packed_simd::u8x16::splat(56)));
        assert!(check_ascii_simd(packed_simd::u8x16::splat(57)));
    }

    #[test]
    fn simd_is_ascii_numeric_invalid_numbers() {
        assert!(!check_ascii_simd(packed_simd::u8x16::splat(0)));
        assert!(!check_ascii_simd(packed_simd::u8x16::splat(47)));
        assert!(!check_ascii_simd(packed_simd::u8x16::splat(58)));
        assert!(!check_ascii_simd(packed_simd::u8x16::splat(128)));
        assert!(!check_ascii_simd(packed_simd::u8x16::splat(255)));
    }
}
