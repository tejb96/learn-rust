// DO NOT EDIT — implement the solution in src/lib.rs

use functions::{divide, sum};

#[test]
fn sum_empty() {
    assert_eq!(sum(&[]), 0);
}

#[test]
fn sum_single() {
    assert_eq!(sum(&[5]), 5);
}

#[test]
fn sum_multiple() {
    assert_eq!(sum(&[1, 2, 3, 4]), 10);
}

#[test]
fn sum_negatives() {
    assert_eq!(sum(&[-1, 1]), 0);
}

#[test]
fn divide_even() {
    assert_eq!(divide(10, 2).unwrap(), 5);
}

#[test]
fn divide_truncates_toward_zero() {
    assert_eq!(divide(7, 2).unwrap(), 3);
}

#[test]
fn divide_negative() {
    assert_eq!(divide(-7, 2).unwrap(), -3);
}

#[test]
fn divide_by_zero() {
    assert!(divide(1, 0).is_err());
}
