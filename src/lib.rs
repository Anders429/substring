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
//! use substring::CharSubstring;
//!
//! // Works on a string slice.
//! assert_eq!("foobar".char_substring(2..5), "oba");
//!
//! // Also works on a String.
//! assert_eq!("foobar".to_string().char_substring(1..6), "oobar");
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
//! use substring::CharSubstring;
//!
//! assert_eq!("ã".char_substring(0..1), "a");  // As opposed to "ã".
//! assert_eq!("ã".char_substring(1..2), "\u{0303}")
//! ```
//!
//! The above example occurs because "ã" is technically made up of two UTF-8 scalar values.
//!
//! [`str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`Substring`]: trait.Substring.html
//! [`substring()`]: trait.Substring.html#tymethod.substring
//!
//! [*Unicode Scalar Value*]: http://www.unicode.org/glossary/#unicode_scalar_value

//#![deny(missing_docs)]
// Since the MSRV is 1.0.0, allowing usage of deprecated items is ok, as the replacements are likely
// not available in early versions.
#![allow(deprecated)]
#![no_std]

#[cfg(feature = "unicode-segmentation")]
extern crate unicode_segmentation;

use core::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};
#[cfg(feature = "unicode-segmentation")]
use unicode_segmentation::UnicodeSegmentation;

/// Extract a substring using the bounds of `index`, guided by the values yielded from `indices`.
///
/// # Safety
/// The caller must ensure that the values in `indices` are within the bounds of `this` and lie on
/// char boundaries of `this`.
unsafe fn substring_from_indices<I: RangeBounds<usize>, J: Iterator<Item = usize>>(
    this: &str,
    index: I,
    mut indices: J,
) -> &str {
    let len = this.len();
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

    this.get_unchecked(
        indices.nth(start).unwrap_or(len)..indices.nth(end - start - 1).unwrap_or(len),
    )
}

/// Provides a [`substring()`] method.
///
/// The [`substring()`] method obtains a string slice of characters within the range specified by
/// `start_index` and `end_index`.
///
/// [`substring()`]: trait.Substring.html#tymethod.substring
pub trait CharSubstring {
    /// Obtains a string slice containing the characters within the range specified by
    /// `start_index` and `end_index`.
    ///
    /// The range specified is a character range, not a byte range.
    fn char_substring<I: RangeBounds<usize>>(&self, index: I) -> &str;
}

/// Implements a [`substring()`] method for [`str`].
///
/// Note that structs which implement `Deref<Target=str>` (such as [`String`]) will also have
/// access to this implementation.
///
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`substring()`]: trait.Substring.html#method.substring
impl CharSubstring for str {
    /// Obtain a slice of the characters within the range of `start_index` and `end_index`.
    ///
    /// As this is by character index, rather than byte index, the temporal complexity of finding a
    /// substring is *O(n)*, where *n* is the byte length of the string.
    ///
    /// Example:
    /// ```
    /// use substring::CharSubstring;
    ///
    /// assert_eq!("foobar".char_substring(2..5), "oba");
    /// ```
    #[inline]
    #[must_use]
    fn char_substring<I: RangeBounds<usize>>(&self, index: I) -> &str {
        unsafe {
            // SAFETY: `self.char_indices()` will always return valid indices for char boundaries
            // within `self`.
            substring_from_indices(self, index, self.char_indices().map(|(i, _c)| i))
        }
    }
}

pub trait GraphemeSubstring {
    fn grapheme_substring<I: RangeBounds<usize>>(&self, index: I) -> &str;
}

#[cfg(feature = "grapheme")]
impl GraphemeSubstring for str {
    #[inline]
    #[must_use]
    fn grapheme_substring<I: RangeBounds<usize>>(&self, index: I) -> &str {
        unsafe {
            // SAFETY: `self.grapheme_indices()` will always return valid indices for char
            // boundaries within `self`.
            substring_from_indices(self, index, self.grapheme_indices(true).map(|(i, _c)| i))
        }
    }
}
