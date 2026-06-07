// DO NOT EDIT — implement the solution in src/lib.rs

use threads_lesson::{parallel_double, parallel_sum};

#[test]
fn parallel_double_works() {
    assert_eq!(parallel_double(vec![1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn parallel_sum_works() {
    assert_eq!(parallel_sum(vec![1, 2, 3, 4]), 10);
}

#[test]
fn parallel_sum_empty() {
    assert_eq!(parallel_sum(vec![]), 0);
}
