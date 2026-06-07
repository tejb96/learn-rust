// DO NOT EDIT — implement the solution in src/lib.rs

use smart_pointers::{shared_list, List};

#[test]
fn shared_list_sum() {
    let list = shared_list();
    assert_eq!(List::sum(list.as_ref()), 6);
}

#[test]
fn prepend_single() {
    let tail = List::new();
    let list = List::prepend(42, tail);
    assert_eq!(List::sum(list.as_ref()), 42);
}
