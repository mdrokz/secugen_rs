# secugen_rs

[![Docs](https://docs.rs/secugen_rs/badge.svg)](https://docs.rs/secugen_rs)
[![Crates.io](https://img.shields.io/crates/v/secugen_rs.svg?maxAge=2592000)](https://crates.io/crates/secugen_rs)

The rust bindings to the secugen SDK allow you to use Secugen fingerprint scanner line of products https://secugen.com/products/#fingerprint


## Installation

```toml
[dependencies]
secugen_rs = "0.2.0"
```

## Usage

```rust
use secugen_rs::sgfpm::FPM;

fn main() {
    let mut fpm = FPM::new();

    let res = fpm.init_device(None, None, None, None);

    match res {
        Ok(_) => println!("Device initialized"),
        Err(e) => println!("Error: {}", e),
    }

    let res = fpm.capture_image();

    match res {
        Ok(b) => println!("Image captured {:?}",b),
        Err(e) => println!("Error: {}", e),
    }

    println!("Hello, world!");
}
```

```bash
LD_LIBRARY_PATH=/usr/local/lib/ cargo run
```
