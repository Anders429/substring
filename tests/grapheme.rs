#![cfg(feature = "grapheme")]

extern crate more_ranges;
extern crate substring;

use more_ranges::RangeFromExclusive;
use std::usize;
use substring::GraphemeSubstring;

#[test]
fn grapheme_substring() {
    assert_eq!("foobar".grapheme_substring(0..3), "foo");
}

#[test]
fn out_of_bounds() {
    assert_eq!("foobar".grapheme_substring(0..10), "foobar");
    assert_eq!("foobar".grapheme_substring(6..10), "");
}

#[test]
fn start_less_than_end() {
    assert_eq!("foobar".grapheme_substring(3..2), "");
}

#[test]
fn start_and_end_equal() {
    assert_eq!("foobar".grapheme_substring(3..3), "");
}

#[test]
fn multiple_char_graphemes() {
    assert_eq!("foobãr".grapheme_substring(3..5), "bã");
}

#[test]
fn unbounded() {
    assert_eq!("foobar".grapheme_substring(..), "foobar");
}

#[test]
fn unbounded_start() {
    assert_eq!("foobar".grapheme_substring(..3), "foo");
}

#[test]
fn unbounded_end() {
    assert_eq!("foobar".grapheme_substring(3..), "bar");
}

#[test]
fn exclusive_start() {
    assert_eq!("foobar".grapheme_substring(RangeFromExclusive { start: 3 }), "ar");
}

#[test]
fn exclusive_start_max() {
    assert_eq!(
        "foobar".grapheme_substring(RangeFromExclusive { start: usize::MAX }),
        ""
    );
}

#[test]
fn inclusive_end() {
    assert_eq!("foobar".grapheme_substring(..=3), "foob");
}

#[test]
fn inclusive_end_max() {
    assert_eq!("foobar".grapheme_substring(..=usize::MAX), "foobar");
}
