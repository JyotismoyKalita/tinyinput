# tinyinput

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)

A tiny, generic utility crate for reading and parsing user input from **stdin** in Rust.

The `read` function is fully generic and relies on Rust’s type inference: the target type is inferred from the variable it is assigned to, removing the need for explicit parsing boilerplate.

`tinyinput` is designed to stay minimal and explicit:
- No macros
- No dependencies
- No hidden panics
- Caller controls error handling policy

It is ideal for small CLI tools, learning projects, and competitive programming utilities.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tinyinput = "0.1"
```

---

## Usage

### Basic usage

```rust,no_run
use tinyinput::read;

fn main() {
    let x: i32 = read("Enter integer: ").unwrap();
    println!("You entered {x}");
}
```

### Using a default value on failure

```rust,no_run
use tinyinput::read;

fn main() {
    let x: i32 = read("Enter integer: ").unwrap_or_default();
    println!("Value: {x}");
}
```

### Custom error handling

```rust,no_run
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
