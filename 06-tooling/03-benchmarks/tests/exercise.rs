// DO NOT EDIT — implement the solution in src/lib.rs

use benchmarks_lesson::{fib, fib_iter};

#[test]
fn fib_base_cases() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}

#[test]
fn fib_small() {
    assert_eq!(fib(10), 55);
}

#[test]
fn fib_iter_matches() {
    assert_eq!(fib_iter(20), fib(20));
}
