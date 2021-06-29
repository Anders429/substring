//! Substring method for string types.
//!
//! This crate provides a [`substring()`] method on Rust string types. The method takes a start and
//! end character index and returns a string slice of the characters within that range.
//!
//! The method is provided via the [`Substring`] trait which is implemented on the [`str`]
//! primitive.
//!
//! # Example
//! ```
//! use substring::Substring;
//!
//! // Works on a string slice.
//! assert_eq!("foobar".substring(2..5), "oba");
//!
//! // Also works on a String.
//! assert_eq!("foobar".to_string().substring(1..6), "oobar");
//! ```
//!
//! As Rust strings are UTF-8 encoded, the algorithm for finding a character substring is `O(n)`,
//! where `n` is the byte length of the string. This is due to characters not being of predictible
//! byte lengths.
//!
//! # Note
//! The indexing of substrings is based on [*Unicode Scalar Value*]. As such, substrings may not
//! always match your intuition:
//!
//! ```
//! use substring::Substring;
//!
//! assert_eq!("ã".substring(0..1), "a");  // As opposed to "ã".
//! assert_eq!("ã".substring(1..2), "\u{0303}")
//! ```
//!
//! The above example occurs because "ã" is technically made up of two UTF-8 scalar values.
//!
//! [`str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`Substring`]: trait.Substring.html
//! [`substring()`]: trait.Substring.html#tymethod.substring
//!
//! [*Unicode Scalar Value*]: http://www.unicode.org/glossary/#unicode_scalar_value

#![deny(missing_docs)]
// Since the MSRV is 1.0.0, allowing usage of deprecated items is ok, as the replacements are likely
// not available in early versions.
#![allow(deprecated)]
#![no_std]

#[cfg(test)]
extern crate more_ranges;

use core::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};

/// Provides a [`substring()`] method.
///
/// The [`substring()`] method obtains a string slice of characters within the range specified by
/// `start_index` and `end_index`.
///
/// [`substring()`]: trait.Substring.html#tymethod.substring
pub trait Substring {
    /// Obtains a string slice containing the characters within the range specified by
    /// `start_index` and `end_index`.
    ///
    /// The range specified is a character range, not a byte range.
    fn substring<I: RangeBounds<usize>>(&self, index: I) -> &str;
}

/// Implements a [`substring()`] method for [`str`].
///
/// Note that structs which implement `Deref<Target=str>` (such as [`String`]) will also have
/// access to this implementation.
///
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`substring()`]: trait.Substring.html#method.substring
impl Substring for str {
    /// Obtain a slice of the characters within the range of `start_index` and `end_index`.
    ///
    /// As this is by character index, rather than byte index, the temporal complexity of finding a
    /// substring is *O(n)*, where *n* is the byte length of the string.
    ///
    /// Example:
    /// ```
    /// use substring::Substring;
    ///
    /// assert_eq!("foobar".substring(2..5), "oba");
    /// ```
    #[must_use]
    fn substring<I: RangeBounds<usize>>(&self, index: I) -> &str {
        let len = self.len();
        let start = match index.start_bound() {
            Excluded(&start) => start.saturating_add(1),
            Included(&start) => start,
            Unbounded => 0,
        };
        let end = match index.end_bound() {
            Excluded(&end) => end,
            Included(&end) => end.saturating_add(1),
            Unbounded => len,
        };
        if end <= start {
            return "";
        }
        let mut indices = self.char_indices().map(|(i, _c)| i);

        unsafe {
            // SAFETY: Since `indices` iterates over the `CharIndices` of `self`, we can guarantee
            // that the indices obtained from it will always be within the bounds of `self` and they
            // will always lie on UTF-8 sequence boundaries.// SAFETY: Since `indices` iterates over the `CharIndices` of `self`, we can guarantee
            // that the indices obtained from it will always be within the bounds of `self` and they
            // will always lie on UTF-8 sequence boundaries.
            self.get_unchecked(
                indices.nth(start).unwrap_or(len)..indices.nth(end - start - 1).unwrap_or(len),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use core::usize;
    use more_ranges::RangeFromExclusive;
    use Substring;

    #[test]
    fn test_substring() {
        assert_eq!("foobar".substring(0..3), "foo");
    }

    #[test]
    fn test_out_of_bounds() {
        assert_eq!("foobar".substring(0..10), "foobar");
        assert_eq!("foobar".substring(6..10), "");
    }

    #[test]
    fn test_start_less_than_end() {
        assert_eq!("foobar".substring(3..2), "");
    }

    #[test]
    fn test_start_and_end_equal() {
        assert_eq!("foobar".substring(3..3), "");
    }

    #[test]
    fn test_multiple_byte_characters() {
        assert_eq!("fõøbα®".substring(2..5), "øbα");
    }

    #[test]
    fn test_unbounded() {
        assert_eq!("foobar".substring(..), "foobar");
    }

    #[test]
    fn test_unbounded_start() {
        assert_eq!("foobar".substring(..3), "foo");
    }

    #[test]
    fn test_unbounded_end() {
        assert_eq!("foobar".substring(3..), "bar");
    }

    #[test]
    fn test_exclusive_start() {
        assert_eq!("foobar".substring(RangeFromExclusive { start: 3 }), "ar");
    }

    #[test]
    fn test_exclusive_start_max() {
        assert_eq!(
            "foobar".substring(RangeFromExclusive { start: usize::MAX }),
            ""
        );
    }

    #[test]
    fn test_inclusive_end() {
        assert_eq!("foobar".substring(..=3), "foob");
    }

    #[test]
    fn test_inclusive_end_max() {
        assert_eq!("foobar".substring(..=usize::MAX), "foobar");
    }
}
