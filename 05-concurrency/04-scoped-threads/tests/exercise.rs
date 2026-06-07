// DO NOT EDIT — implement the solution in src/lib.rs

use scoped_threads::{parallel_any, scoped_double};

#[test]
fn scoped_double_works() {
    let mut nums = vec![1, 2, 3, 4];
    scoped_double(&mut nums);
    assert_eq!(nums, vec![2, 4, 6, 8]);
}

#[test]
fn parallel_any_found() {
    assert!(parallel_any(&[1, 2, 10], 5));
}

#[test]
fn parallel_any_not_found() {
    assert!(!parallel_any(&[1, 2, 3], 5));
}
