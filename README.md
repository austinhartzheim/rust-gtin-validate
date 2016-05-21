# upc_validate
Validate and correct UPC-A codes in Rust.

Validation features include:
* Checking UPC-A length (should be 12 digits)
* Computing the check-digit and confirming match

Correction features include:
* Zero-padding short UPC-A codes (some software treat UPCs as integers)
* Strip whitespace from both sides of the code

## how to use
Here are the function declarations:
```rust
fn check_upca(upc: &str) -> bool
fn fix_upca(upc: &str) -> String
```