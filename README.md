# constcat

[![Crates.io Version](https://img.shields.io/crates/v/constcat.svg)](https://crates.io/crates/constcat)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/constcat)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/constcat/build/trunk)](https://github.com/rossmacarthur/constcat/actions?query=workflow%3Abuild)

`std::concat!` with support for `const` variables and expressions.

Works on stable Rust ✨.

## 🚀 Getting started

Add `constcat` to your Cargo manifest.

```toml
[dependencies]
constcat = "0.2.0"
```

Import the macro using the following.

```rust
use constcat::concat;
```

## 🤸 Usage

`concat!` works exactly like `std::concat!` except you can now pass variables and
constant expressions.

```rust
use constcat::concat;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const fn tada() -> &'static str { "🎉" }
const VERSION: &str = concat!(CRATE_NAME, " ", CRATE_VERSION, tada());
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
