# Rust GTIN Validator
[![Build Status](https://travis-ci.org/austinhartzheim/rust-gtin-validate.svg?branch=master)](https://travis-ci.org/austinhartzheim/rust-gtin-validate)
[![Coverage Status](https://coveralls.io/repos/github/austinhartzheim/rust-gtin-validate/badge.svg)](https://coveralls.io/github/austinhartzheim/rust-gtin-validate)
[![Documentation](https://docs.rs/gtin-validate/badge.svg)](https://docs.rs/gtin-validate/)

Validate and correct GTIN codes, such as UPC-A and EAN-13, in Rust.

## Features
Currently supported types:
* GTIN-8
* GTIN-12 (UPC-A)
* GTIN-13 (EAN-13)
* GTIN-14

Validation features include:
* Check that the string contains the correct number of digits
* Compute the check-digit and confirm that it matches

Correction features include:
* Add zero-padding (some software treats these codes as integers)
* Strip whitespace from both sides of the code

## How to use
For full details, check [the documentation](https://docs.rs/gtin-validate/).

Briefly, here are the function declarations:
```rust
// return true if the code is valid, false otherwise
fn gtin8::check(code: &str) -> bool;
fn gtin12::check(code: &str) -> bool;
fn gtin13::check(code: &str) -> bool;
fn gtin14::check(code: &str) -> bool;

// return a corrected String or Err
fn gtin8::fix(code: &str) -> Result<String, FixError>;
fn gtin12::fix(code: &str) -> Result<String, FixError>;
fn gtin13::fix(code: &str) -> Result<String, FixError>;
fn gtin14::fix(code: &str) -> Result<String, FixError>;
```

For example, you can validate UPC-A codes:
```rust
use gtin_validate::gtin12;

assert_eq!(gtin12::check("000000000000"), true);
assert_eq!(gtin12::check("000000000001"), false);
```

You can add this line to your Cargo.toml file:
```toml
[dependencies]
gtin-validate = "1.2.0"
```

## Contributing
Found a bug? Report an issue through GitHub.

Want to hack on the code? Submit a pull request.
