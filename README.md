# constcat

[![Crates.io Version](https://img.shields.io/crates/v/constcat.svg)](https://crates.io/crates/constcat)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/constcat)

`std::concat!` with support for `const` variables and expressions.

Works on stable Rust ✨.

## 🚀 Getting started

Add `constcat` to your Cargo manifest.

```toml
[dependencies]
constcat = "0.1.0"
```

Import the macro using the following.

```rust
use constcat::constcat;
```

## 🤸 Usage

```rust
use constcat::constcat;

const EX: &str = constcat!("string", 10, 'c', true, 3.14, VARIABLE, expr());
assert_eq!(EX, "string10ctrue3.14constcat🎉");

const VARIABLE: &str = env!("CARGO_PKG_NAME");

const fn expr() -> &'static str {
    "🎉"
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
