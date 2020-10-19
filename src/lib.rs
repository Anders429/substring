//! Substring method for string types.
//!
//! This crate provides a `substring` method on Rust string types. The method takes a start and end
//! character index and returns a string slice of the characters within that range.
//!
//! The method is provided via the `Substring` trait which is implemented on the `&str` primitive.
//!
//! # Example
//! ```
//! use substring::Substring;
//!
//! // Works on a string slice.
//! assert_eq!("foobar".substring(2,5), "oba");
//!
//! // Also works on a String.
//! assert_eq!("foobar".to_string().substring(1,6), "oobar");
//! ```
//!
//! As Rust strings are UTF-8 encoded, the algorithm for finding a character substring is `O(n)`,
//! where `n` is the byte length of the string. This is due to characters not being of predictible
//! byte lengths.

/// Provides a `substring` method.
///
/// The `substring` method obtains a string slice of characters within the range specified by
/// `start_index` and `end_index`.
pub trait Substring {
    /// Obtains a string slice containing the characters within the range specified by
    /// `start_index` and `end_index`.
    ///
    /// The range specified is a character range, not a byte range.
    fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

/// Provides a `substring` method for `&str`.
impl Substring for str {
    /// Obtain a slice of the characters within the range of `start_index` and `end_index`.
    ///
    /// As this is by character index, rather than byte index, the temporal complexity of finding a
    /// substring is `O(n)`.
    ///
    /// Example:
    /// ```
    /// use substring::Substring;
    ///
    /// assert_eq!("foobar".substring(2,5), "oba");
    /// ```
    fn substring(&self, start_index: usize, end_index: usize) -> &str {
        if end_index <= start_index {
            return "";
        }

        let mut indices = self.char_indices();

        let obtain_index = |(index, _char)| index;

        &self[indices
            .nth(start_index)
            .map(&obtain_index)
            .unwrap_or(self.len())
            ..indices
                .nth(end_index - start_index - 1)
                .map(&obtain_index)
                .unwrap_or(self.len())]
    }
}

#[cfg(test)]
mod tests {
    use crate::Substring;

    #[test]
    fn test_substring() {
        assert_eq!("foobar".substring(0, 3), "foo");
    }

    #[test]
    fn test_out_of_bounds() {
        assert_eq!("foobar".substring(0, 10), "foobar");
        assert_eq!("foobar".substring(6, 10), "");
    }

    #[test]
    fn test_start_less_than_end() {
        assert_eq!("foobar".substring(3, 2), "");
    }

    #[test]
    fn test_start_and_end_equal() {
        assert_eq!("foobar".substring(3, 3), "");
    }

    #[test]
    fn test_multiple_byte_characters() {
        assert_eq!("fõøbα®".substring(2, 5), "øbα");
    }
}
