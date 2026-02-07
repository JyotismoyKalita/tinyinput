# tinyinput

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-2021-orange.svg)
[![Crates.io](https://img.shields.io/crates/v/tinyinput.svg)](https://crates.io/crates/tinyinput)
[![Docs.rs](https://docs.rs/tinyinput/badge.svg)](https://docs.rs/tinyinput)

A tiny, dependency-free helper crate for **reading and parsing user input from `stdin` in Rust**.

`tinyinput` is designed for **small CLI tools, scripts, and learning projects** where you want to read user input without repeating the usual `stdin` + `parse` boilerplate.

---

## Why tinyinput?

In Rust, taking user input typically involves:

1. Reading a line from `stdin`
2. Trimming whitespace
3. Parsing the input into the desired type

While explicit and correct, this pattern quickly becomes repetitive in small programs.

`tinyinput` provides a **minimal and type-safe alternative** by leveraging **Rust’s compile-time type inference**.  
The target type is inferred from the assignment context, so no explicit parsing logic is required at the call site.

---

## How to take user input in Rust

### Standard Rust approach

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let x: i32 = input.trim().parse().unwrap();
```

### Using `tinyinput`

```rust
use tinyinput::read;

let x: i32 = read("Enter number: ").unwrap();
```

This reduces boilerplate while remaining explicit, type-safe, and idiomatic.

---

## Installation

Add `tinyinput` to your `Cargo.toml`:

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

```rust
use tinyinput::read;

fn main() {
    let count: i32 = read("Enter count: ").unwrap();
    let ratio: f64 = read("Enter ratio: ").unwrap_or_default();
    let name: String = read("Enter name: ").unwrap();

    println!("{count}, {ratio}, {name}");
}
```

---

## API Overview

### `read`

```rust
pub fn read<T>(prompt: &str) -> Result<T, ReadError>
where
    T: FromStr,
```

- Prints the prompt (if non-empty)
- Reads a single line from `stdin`
- Trims whitespace
- Parses the input into type `T`
- Returns errors instead of panicking

```rust
let value: usize = read("Enter value: ").unwrap();
```

---

## Error Handling

```rust
pub enum ReadError {
    Io(std::io::Error),
    Parse,
}
```

- `Io` — reading from standard input failed
- `Parse` — input could not be parsed into the requested type

---

## Design Philosophy

- Tiny and focused
- No macros
- No global state
- No hidden panics
- No dependencies
- Explicit error handling
- Leverages Rust’s type inference

This crate is **not** a replacement for full command-line parsers like `clap`.

---

## When should you use tinyinput?

- Learning Rust
- Small CLI tools and scripts
- Teaching stdin input handling
- Reducing boilerplate in examples

---

## License

MIT License

## Author

Jyotismoy Kalita
