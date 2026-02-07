//! # tinyinput
//!
//! `tinyinput` is a tiny, dependency-free helper crate for **reading and parsing
//! user input from stdin in Rust**.
//!
//! ## Why tinyinput?
//!
//! Taking user input in Rust typically involves reading from `stdin`, trimming
//! the input, and manually parsing it into the desired type. While this is
//! explicit and correct, it often leads to repetitive boilerplate in small
//! programs, examples, and learning projects.
//!
//! `tinyinput` provides a simpler alternative by leveraging **Rust’s compile-time
//! type inference**. The target type is inferred from the assignment context,
//! so callers usually do not need to specify the type explicitly at the call site.
//!
//! ## Example: reading user input in Rust
//!
//! ```no_run
//! let count: i32 = tinyinput::read("Enter count: ").unwrap();
//! let ratio: f64 = tinyinput::read("Enter ratio: ").unwrap_or_default();
//! let name: String = tinyinput::read("Enter name: ").unwrap();
//! ```
//!
//! ## What this crate is (and is not)
//!
//! - ✔ Minimal and lightweight
//! - ✔ No macros
//! - ✔ No global state
//! - ✔ No hidden panics
//! - ✔ No dependencies
//! - ✔ Suitable for small CLI tools, scripts, and learning Rust
//!
//! - ✘ Not a replacement for full command-line parsers like `clap`
//! - ✘ Not intended for complex argument handling
//!
//! ## Design philosophy
//!
//! `tinyinput` focuses on doing **one thing well**: reading a single line of
//! input from standard input and parsing it into a Rust type using `FromStr`.
//! Error handling is explicit and returned to the caller, allowing each program
//! to decide how to handle invalid input.

use std::io::{self, Write};
use std::str::FromStr;

/// Errors that can occur while reading or parsing user input.
#[derive(Debug)]
pub enum ReadError {
    /// An I/O error occurred while reading from standard input.
    Io(io::Error),
    /// The input could not be parsed into the requested type.
    Parse,
}

/// Read a line of input from standard input and parse it into type `T`.
///
/// This function prints the provided `prompt` (if non-empty), reads a single
/// line from `stdin`, trims whitespace, and attempts to parse the input using
/// the `FromStr` implementation of `T`.
///
/// The return type is inferred from the assignment context, which allows
/// ergonomic usage without explicit type annotations at the call site.
///
/// ## Example
///
/// ```no_run
/// let x: i32 = tinyinput::read("Enter number: ").unwrap();
/// ```
///
/// ## Behavior
///
/// - If `prompt` is empty, no prompt is printed.
/// - Input is read from standard input using `stdin`.
/// - Whitespace is trimmed before parsing.
/// - Errors are returned as `ReadError` instead of panicking.
///
/// ## Errors
///
/// - Returns `ReadError::Io` if reading from `stdin` fails.
/// - Returns `ReadError::Parse` if parsing into `T` fails.
pub fn read<T>(prompt: &str) -> Result<T, ReadError>
where
    T: FromStr,
{
    let mut temp = String::new();

    if !prompt.is_empty() {
        print!("{}", prompt);
        io::stdout().flush().map_err(ReadError::Io)?;
    }

    io::stdin().read_line(&mut temp).map_err(ReadError::Io)?;

    temp.trim().parse::<T>().map_err(|_| ReadError::Parse)
}
