//! Validate GTIN-style barcodes, including the commonly-used UPC-A and
//! EAN-13 codes.
//!
//! In addition to validating the correctness of the codes, you can
//! attempt to automatically repair the most common errors, such as the
//! removal of leading zeroes by spreadsheet software (because the codes
//! are sometimes treated as integers) and the removal of whitespace
//! that can be accidentally introduced during data entry or spreadsheet
//! conversion.

#![forbid(unsafe_code)]

#[cfg(test)]
#[macro_use]
extern crate proptest;

// private modules for internal use
mod utils;

// public modules
pub mod gtin12;
pub mod gtin13;
pub mod gtin14;
pub mod gtin8;
