Substring method for string types.

This crate provides a `substring` method on both `String` and `&str` types. The method takes a
start and end character index and returns a string slice of the characters within that range.

The method is provided via the `Substring` trait which is implemented on both `String` and `&str`.

# Usage

To use this crate, simply bring the `Substring` trait into scope and call the `substring` method on
your string types.

```
use substring::Substring;

assert_eq!("hello, world!".substring(7, 12), "world");
```

# Performance

As Rust strings are UTF-8, the algorithm for finding a character substring is `O(n)`, where `n` is
the byte length of the string. This is due to characters not being of predictible byte lengths.
