//! tinyinput
//!
//! A tiny, generic helper for reading and parsing user input from stdin.
//!
//! The `read` function is fully generic and relies on Rust’s type inference:
//! the target type is inferred from the variable it is assigned to.
//!
//! # Example
//! ```no_run
//! let count: i32 = tinyinput::read("Enter count: ").unwrap();
//! let ratio: f64 = tinyinput::read("Enter ratio: ").unwrap_or_default();
//! let name: String = tinyinput::read("Enter name: ").unwrap();
//! ```

use std::io::{self, Write};
use std::str::FromStr;

/// Errors that can occur while reading input.
#[derive(Debug)]
pub enum ReadError {
    /// I/O error while reading from stdin
    Io(io::Error),
    /// Failed to parse input into requested type
    Parse,
}

/// Read input from stdin and parse into `T`.
///
/// # Example
/// ```no_run
/// let x: i32 = tinyinput::read("Enter number: ").unwrap();
/// ```
pub fn read<T>(prompt: &str) -> Result<T, ReadError>
where
    T: FromStr,
{
    let mut temp = String::new();

    print!("{}", prompt);
    io::stdout().flush().map_err(ReadError::Io)?;

    io::stdin().read_line(&mut temp).map_err(ReadError::Io)?;

    temp.trim().parse::<T>().map_err(|_| ReadError::Parse)
}
