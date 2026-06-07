// DO NOT EDIT — implement the solution in src/lib.rs

use workspaces_crates::{mul_then_add, sum_then_mul};

#[test]
fn sum_then_mul_works() {
    assert_eq!(sum_then_mul(2, 3, 4), 20);
}

#[test]
fn mul_then_add_works() {
    assert_eq!(mul_then_add(2, 3, 4), 10);
}
