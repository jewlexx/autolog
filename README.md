# AutoLog

![docs.rs](https://img.shields.io/docsrs/autolog)
![Crates.io](https://img.shields.io/crates/d/autolog)
![Crates.io](https://img.shields.io/crates/v/autolog)
![GitHub last commit](https://img.shields.io/github/last-commit/jewlexx/autolog)

A mini Rust library for logging when a function is called.

## Features

- Tracing support
- Custom log messages
- Variables in log messages (function name and function arguments)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
autolog = "0.1"
```

## Example

```rust
use autolog::autolog;
// or
#[macro_use]
extern crate autolog;

#[autolog]
fn main() {
    println!("Hello, world!");
}
```

## License

![Crates.io](https://img.shields.io/crates/l/autolog)

**Made with ❤️ by [Juliette Cordor](https://github.com/jewlexx)**
