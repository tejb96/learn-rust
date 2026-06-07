// DO NOT EDIT — implement the solution in src/lib.rs

use custom_errors::{find, wrap, AppError};
use std::collections::HashMap;

#[test]
fn find_happy_path() {
    let mut store = HashMap::new();
    store.insert("u1".into(), "Ada".into());
    assert_eq!(find(&store, "u1").unwrap(), "Ada");
}

#[test]
fn find_not_found() {
    let store = HashMap::new();
    assert_eq!(find(&store, "missing"), Err(AppError::NotFound));
}

#[test]
fn find_empty_id() {
    let mut store = HashMap::new();
    store.insert("u1".into(), "Ada".into());
    assert_eq!(
        find(&store, ""),
        Err(AppError::Validation {
            field: "id".into(),
            code: "required".into(),
        })
    );
}

#[test]
fn wrap_formats() {
    let msg = wrap(AppError::NotFound, "lookup user");
    assert_eq!(msg, "lookup user: not found");
}
