use std::ascii::AsciiExt;

fn check_upca(upc: &str) -> bool {
    let mut even: u8 = 0;
    let mut odd: u8 = 0;
    let check: i16;
    let mut curr: u8;
    
    // Check that input is ASCII with length 12
    if upc.is_ascii() == false {
        return false;
    }
    if upc.len() != 12 {
        return false;
    }

    let bytes = upc.as_bytes();

    // Calculate and compare check digit 
    for i in 0..10 {
        curr = bytes[i];

        // Handle ASCII range for numbers. Convert to u8.
        if curr < 48 || curr > 48 + 9 {
            return false;
        }
        curr -= 48;
        
        if i % 2 == 0 {
            even += curr;
        } else {
            odd += curr;
        }
    }
    if bytes[11] < 48 || bytes[11] > 48 + 9 {
        return false;
    }
    check = -1 * ((odd + even) as i16) % 10;
    if check != ((bytes[11] - 48) as i16) {
        return false;
    }
    
    return true;
}

fn fix_upca(upc: &str) -> &str {
    let mut fixed : &str = upc.trim_left().trim_right();

    if upc.is_ascii() == false {
        panic!("Cannot operate on non-ASCII data");
    }
    if fixed.len() > 12 {
        panic!("Could not fix UPC-A; length did not match")
    } else if fixed.len() < 12 {
        
    }
    &"00000000000"
}

#[cfg(test)]
mod tests {
    use super::check_upca;
    use super::fix_upca;
    
    #[test]
    fn check_upca_valid() {
        assert!(check_upca(&"000000000000") == true);
    }

    #[test]
    fn check_upca_invalid_length() {
        assert!(check_upca(&"000") == false);
    }

    #[test]
    fn check_upca_non_ascii() {
        assert!(check_upca(&"❤") == false);
    }

    #[test]
    fn check_upca_invalid_check_digit() {
        assert!(check_upca(&"000000000001") == false);
        assert!(check_upca(&"000000000002") == false);
        assert!(check_upca(&"000000000003") == false);
        assert!(check_upca(&"000000000004") == false);
        assert!(check_upca(&"000000000005") == false);
        assert!(check_upca(&"000000000006") == false);
        assert!(check_upca(&"000000000007") == false);
        assert!(check_upca(&"000000000008") == false);
        assert!(check_upca(&"000000000009") == false);
    }


    #[test]
    #[should_panic]
    fn fix_upca_non_ascii() {
        fix_upca(&"❤");
    }
}
