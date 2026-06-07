// DO NOT EDIT — implement the solution in src/lib.rs

use variables::{default_greeting, describe_default, swap};

#[test]
fn swap_two_names() {
    let (a, b) = swap("hello", "world");
    assert_eq!(a, "world");
    assert_eq!(b, "hello");
}

#[test]
fn swap_empty_first() {
    let (a, b) = swap("", "go");
    assert_eq!(a, "go");
    assert_eq!(b, "");
}

#[test]
fn swap_both_empty() {
    let (a, b) = swap("", "");
    assert_eq!(a, "");
    assert_eq!(b, "");
}

#[test]
fn describe_default_string() {
    let got = describe_default("string").unwrap();
    assert_eq!(got, r#"default value is """#);
}

#[test]
fn describe_default_int() {
    let got = describe_default("int").unwrap();
    assert_eq!(got, "default value is 0");
}

#[test]
fn describe_default_bool() {
    let got = describe_default("bool").unwrap();
    assert_eq!(got, "default value is false");
}

#[test]
fn describe_default_slice() {
    let got = describe_default("slice").unwrap();
    assert_eq!(got, "default value is empty");
}

#[test]
fn describe_default_unknown() {
    assert!(describe_default("chan").is_err());
}

#[test]
fn default_greeting_with_name() {
    assert_eq!(default_greeting("Ada"), "Hello, Ada!");
}

#[test]
fn default_greeting_empty() {
    assert_eq!(default_greeting(""), "Hello, stranger!");
}
