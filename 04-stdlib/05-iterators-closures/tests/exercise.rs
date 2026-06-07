// DO NOT EDIT — implement the solution in src/lib.rs

use iterators_closures::{make_adder, partition_evens_odds, squares_above, total_trimmed_len};

#[test]
fn squares_above_works() {
    assert_eq!(squares_above(&[1, 2, 3, 4, 5], 2), vec![9, 16, 25]);
}

#[test]
fn total_trimmed_len_works() {
    assert_eq!(total_trimmed_len(&[" hi ", "go"]), 4);
}

#[test]
fn make_adder_works() {
    let add3 = make_adder(3);
    assert_eq!(add3(10), 13);
}

#[test]
fn partition_works() {
    assert_eq!(
        partition_evens_odds(&[1, 2, 3, 4]),
        (vec![2, 4], vec![1, 3])
    );
}
