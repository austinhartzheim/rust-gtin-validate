# Rust GTIN Validator
[![Build Status](https://travis-ci.org/austinhartzheim/rust-upc-validate.svg?branch=master)](https://travis-ci.org/austinhartzheim/rust-upc-validate)

Validate and correct GTIN codes, such as UPC-A and EAN13, in Rust.

## Features
Currenly supported types:
* UPC-A (GTIN-12)

Validation features include:
* Checking UPC-A code length (should be 12 digits)
* Computing the check-digit and confirming match

Correction features include:
* Zero-padding short UPC-A codes (some software treat UPCs as integers)
* Strip whitespace from both sides of the code

## How to use
Here are the function declarations:
```rust
// return true if UPC-A is valid
fn check_upca(upc: &str) -> bool

// return a corrected UPC-A String or Err
pub fn fix_upca(upc: &str) -> Result<String, UpcAFixError>
```

For example, you can validate UPC-A codes:
```rust
use gtin_validate::gtin12;
assert_eq!(gtin12::check_upca("000000000000"), true);
```

You can add this line to your Cargo.toml file:
```toml
[dependencies]
gtin-validate = { git = "https://github.com/austinhartzheim/rust-gtin-validate.git" }
```

## Contributing
Found a bug? Report an issue through GitHub.

Want to hack on the code? Submit a pull request.