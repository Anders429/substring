![Crates.io](https://img.shields.io/crates/v/substring)
![Docs.rs](https://docs.rs/substring/badge.svg)

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

As Rust strings are UTF-8, the algorithm for finding a character substring has temporal complexity
`O(n)`, where `n` is the byte length of the string. This is due to characters not being of
predictible byte lengths.
