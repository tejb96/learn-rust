// DO NOT EDIT — implement the solution in src/lib.rs

use borrowing::{append_in_place, double_in_place, longest};

#[test]
fn append_in_place_works() {
    let mut s = String::from("hello");
    append_in_place(&mut s, " world");
    assert_eq!(s, "hello world");
}

#[test]
fn longest_first() {
    assert_eq!(longest("short", "longer"), "longer");
}

#[test]
fn longest_tie() {
    assert_eq!(longest("abc", "xyz"), "abc");
}

#[test]
fn double_in_place_works() {
    let mut nums = vec![1, 2, 3];
    double_in_place(&mut nums);
    assert_eq!(nums, vec![2, 4, 6]);
}
