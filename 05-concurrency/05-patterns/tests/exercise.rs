// DO NOT EDIT — implement the solution in src/lib.rs

use concurrency_patterns::run_jobs;

#[test]
fn run_jobs_basic() {
    let results = run_jobs(2, 5, |i| (i as i32) * 2);
    assert_eq!(results, vec![0, 2, 4, 6, 8]);
}

#[test]
fn run_jobs_single_worker() {
    let results = run_jobs(1, 3, |i| i as i32 + 1);
    assert_eq!(results, vec![1, 2, 3]);
}
