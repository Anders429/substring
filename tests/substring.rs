extern crate more_ranges;
extern crate substring;

use more_ranges::RangeFromExclusive;
use std::usize;
use substring::Substring;

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
