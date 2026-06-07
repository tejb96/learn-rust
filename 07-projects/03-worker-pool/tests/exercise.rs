// DO NOT EDIT — implement the solution in src/lib.rs

use worker_pool_project::run_indexed_jobs;

#[test]
fn run_indexed_jobs_order() {
    let results = run_indexed_jobs(2, 5);
    assert_eq!(results, vec![(0, 0), (1, 2), (2, 4), (3, 6), (4, 8)]);
}

#[test]
fn run_indexed_jobs_single_worker() {
    let results = run_indexed_jobs(1, 3);
    assert_eq!(results, vec![(0, 0), (1, 2), (2, 4)]);
}
