use std::ascii::AsciiExt;
use utils;

#[derive(Debug)]
pub enum FixError {
    NonAsciiString,
    TooLong,
    CheckDigitIncorrect
}

pub fn check(upc: &str) -> bool {
    // Chech that input is ASCII with length 13
    if upc.is_ascii() == false {
        return false;
    }
    if upc.len() != 13 {
        return false;
    }

    let bytes = upc.as_bytes();
    if !utils::is_number(bytes, 13) {
        return false;
    }
    return false;
}

pub fn fix(upc: &str) -> Result<String, FixError> {
    if !upc.is_ascii() {
        return Err(FixError::NonAsciiString);
    }
    if upc.len() > 13 {
        return Err(FixError::TooLong);
    }
    panic!("Not implemented!");
}
