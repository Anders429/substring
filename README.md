[![Travis-ci.org](https://img.shields.io/travis/Anders429/substring)](https://travis-ci.org/github/Anders429/substring)
[![Crates.io](https://img.shields.io/crates/v/substring)](https://crates.io/crates/substring)
[![Docs.rs](https://docs.rs/substring/badge.svg)](https://docs.rs/substring)
[![MSRV](https://img.shields.io/badge/rustc-1.6+-green.svg)](https://github.com/Anders429/substring#minimum-supported-rust-version)
![License](https://img.shields.io/crates/l/substring)

# substring

Substring method for string types.

This crate provides a `substring` method on Rust string types. The method takes a start and end
character index and returns a string slice of the characters within that range.

The method is provided via the `Substring` trait which is implemented on the `&str` primitive.

## Usage

To use this crate, simply bring the `Substring` trait into scope and call the `substring` method on
your string types.

```rust
use substring::Substring;

assert_eq!("hello, world!".substring(7, 12), "world");
```

Notet that the indexing of substrings is based on Unicode Scalar Value. As such, substrings may not
always match your intuition:

```rust
use substring::Substring;

assert_eq!("ã".substring(0, 1), "a");  // As opposed to "ã".
assert_eq!("ã".substring(1, 2), "\u{0303}")
```

The above example occurs because "ã" is technically made up of two UTF-8 scalar values: the letter
"a" and a combining tilde.


## Performance

As Rust strings are UTF-8 encoded, the algorithm for finding a character substring has temporal
complexity `O(n)`, where `n` is the byte length of the string. This is due to characters not being
of predictible byte lengths.

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable Rust 1.6 and up.
