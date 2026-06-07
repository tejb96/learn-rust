// DO NOT EDIT — implement the solution in src/lib.rs

use mutex_arc::{parallel_increment, parallel_insert};

#[test]
fn parallel_increment_works() {
    assert_eq!(parallel_increment(10), 10);
}

#[test]
fn parallel_insert_works() {
    let keys = vec!["a".into(), "b".into(), "c".into()];
    let map = parallel_insert(keys);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&1));
    assert_eq!(map.get("c"), Some(&1));
}
