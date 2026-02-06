# tinyinput

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)
[![Crates.io](https://img.shields.io/crates/v/tinyinput.svg)](https://crates.io/crates/tinyinput)

A tiny, generic utility crate for reading and parsing user input from **stdin** in Rust.

`read` is fully generic and relies on Rust’s compile-time type inference:
the target type is inferred from the assignment context.

`tinyinput` is designed to stay minimal and explicit.

## Why tinyinput?

- Generic and type-safe
- Uses Rust’s type inference — no explicit parsing
- No macros, no dependencies
- Caller-controlled error handling

### Without tinyinput

```rust
let mut s = String::new();
std::io::stdin().read_line(&mut s).unwrap();
let x: i32 = s.trim().parse().unwrap();
```

### With tinyinput

```rust
let x: i32 = tinyinput::read("Enter number: ").unwrap();
```

It is ideal for small CLI tools, learning projects, and competitive programming utilities.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tinyinput = "0.1"
```

---

## Example

Run the included example:

```bash
cargo run --example demo
```

---

## Usage

### Basic usage

```rust
use tinyinput::read;

fn main() {
    let x: i32 = read("Enter integer: ").unwrap();
    println!("You entered {x}");
}
```

### Using a default value on failure

```rust
use tinyinput::read;

fn main() {
    let x: i32 = read("Enter integer: ").unwrap_or_default();
    println!("Value: {x}");
}
```

### Custom error handling

```rust
use tinyinput::read;

fn main() {
    match read::<f32>("Enter float: ") {
        Ok(v) => println!("Value: {v}"),
        Err(e) => eprintln!("Input error: {e:?}"),
    }
}
```

---

## API

### `read`

```rust
pub fn read<T>(prompt: &str) -> Result<T, ReadError>
where
    T: FromStr,
```

Reads a line from standard input, trims it, and parses it into `T`.

### Errors

```rust
pub enum ReadError {
    Io(std::io::Error),
    Parse,
}
```

- `Io` – failed to read from stdin
- `Parse` – failed to parse input into the requested type

---

## Design Philosophy

- Minimal API surface
- Explicit error handling
- No opinionated defaults
- Caller decides how to handle failures

---

## License

MIT License

## Author

Jyotismoy Kalita
