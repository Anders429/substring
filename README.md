[![Crates.io](https://img.shields.io/crates/v/substring)](https://crates.io/crates/substring)
![Crates.io](https://img.shields.io/crates/l/substring)
[![Docs.rs](https://docs.rs/substring/badge.svg)](https://docs.rs/substring)

Substring method for string types.

This crate provides a `substring` method on Rust string types. The method takes a start and end
character index and returns a string slice of the characters within that range.

The method is provided via the `Substring` trait which is implemented on the `&str` primitive.

# Usage

To use this crate, simply bring the `Substring` trait into scope and call the `substring` method on
your string types.

```rust
use substring::Substring;

assert_eq!("hello, world!".substring(7, 12), "world");
```

# Performance

As Rust strings are UTF-8 encoded, the algorithm for finding a character substring has temporal
complexity `O(n)`, where `n` is the byte length of the string. This is due to characters not being
of predictible byte lengths.

# Caution
The indexing of substrings is based on Unicode Scalar Value. As such, substrings may not always
match your intuition:

```rust
use substring::Substring;

assert_eq!("y̆".substring(0, 1), "y");  // As opposed to "y̆".
assert_eq!("y̆".substring(1, 2), "\u{0306}")  // The diacritical mark counts as its own character.
```

The above example occurs because "y̆"" is technically made up of two UTF-8 scalar values.
