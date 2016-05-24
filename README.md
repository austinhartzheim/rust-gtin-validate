# upc_validate
[![Build Status](https://travis-ci.org/austinhartzheim/rust-upc-validate.svg?branch=master)](https://travis-ci.org/austinhartzheim/rust-upc-validate)
Validate and correct UPC-A codes in Rust.

Validation features include:
* Checking UPC-A length (should be 12 digits)
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
pub fn fix_upca(upc: &str) -> Result<String, &str>
```

You can add this line to your Cargo.toml file:
```toml
[dependencies]
rand = { git = "https://github.com/austinhartzheim/rust-upc-validate.git" }
```

## Note about efficiency
This is my first Rust project. There are certainly more efficient ways to implement this code.

Pull requests are greatly appreciated.