use std::sync::mpsc;
use std::thread;

/// Sends each number on a channel from a producer thread and collects them in order sent.
pub fn pipeline(nums: Vec<i32>) -> Vec<i32> {
    vec![]
}

/// Returns the sum produced by a worker thread receiving nums over a channel.
pub fn sum_via_channel(nums: Vec<i32>) -> i32 {
    0
}
