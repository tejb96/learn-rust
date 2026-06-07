// DO NOT EDIT — implement the solution in src/lib.rs

use slices_lesson::{first_word, safe_slice, sum_slice};

#[test]
fn safe_slice_valid() {
    let data = [1, 2, 3, 4];
    assert_eq!(safe_slice(&data, 1, 3), Some(&[2, 3][..]));
}

#[test]
fn safe_slice_invalid() {
    let data = [1, 2, 3];
    assert_eq!(safe_slice(&data, 0, 4), None);
}

#[test]
fn first_word_with_space() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn first_word_no_space() {
    assert_eq!(first_word("hello"), "hello");
}

#[test]
fn sum_slice_works() {
    assert_eq!(sum_slice(&[1, 2, 3]), 6);
}
