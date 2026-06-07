// DO NOT EDIT — implement the solution in src/lib.rs

use channels_lesson::{pipeline, sum_via_channel};

#[test]
fn pipeline_preserves_order() {
    assert_eq!(pipeline(vec![1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn sum_via_channel_works() {
    assert_eq!(sum_via_channel(vec![1, 2, 3, 4]), 10);
}
