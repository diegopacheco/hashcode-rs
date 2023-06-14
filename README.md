## hashcode-rs

Simple lib experiment with Rust.
Inpired on Java's String.hashCode().

## Build
```bash
cargo build
```

## Usage
Cargo.toml
```Rust
[dependencies]
hashcode-rs = {version = "0.1.0", path = "../hashcode-rs/"}
```
main.rs
```Rust
use hashcode_rs::*;
fn main() {
    let hash_code = hash_code_str("Test");
    println!("hashcode of Test is {}",hash_code);
}
```