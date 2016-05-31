# Rust GTIN Validator
[![Build Status](https://travis-ci.org/austinhartzheim/rust-gtin-validate.svg?branch=master)](https://travis-ci.org/austinhartzheim/rust-gtin-validate)

Validate and correct GTIN codes, such as UPC-A and EAN-13, in Rust.

## Features
Currenly supported types:
* GTIN-12 (UPC-A)
* GTIN-13 (EAN-13)

Validation features include:
* Check that the string contains the correct number of digits
* Compute the check-digit and confirming that it matches

Correction features include:
* Add zero-padding (some software treat these codes as integers)
* Strip whitespace from both sides of the code

## How to use
Here are the function declarations:
```rust
// return true if the GTIN-12 code is valid
fn gtin12::check(upc: &str) -> bool
fn gtin13::check(upc: &str) -> bool
fn gtin14::check(upc: &str) -> bool

// return a corrected GTIN-12 String or Err
fn gtin12::fix(upc: &str) -> Result<String, UpcAFixError>
fn gtin13::fix(upc: &str) -> Result<String, UpcAFixError>
fn gtin14::fix(code: &str) -> Result<String, UpcAFixError>
```

For example, you can validate UPC-A codes:
```rust
extern create gtin_validate;
use gtin_validate::gtin12;

assert_eq!(gtin12::check("000000000000"), true);
assert_eq!(gtin12::check("000000000001"), false);
```

You can add this line to your Cargo.toml file:
```toml
[dependencies]
gtin-validate = "0.5.0"
```

## Contributing
Found a bug? Report an issue through GitHub.

Want to hack on the code? Submit a pull request.