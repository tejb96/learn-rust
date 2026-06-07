// DO NOT EDIT — fix bugs in src/lib.rs until these pass

use testing_lesson::{abs_value, divide, parse_bool};

#[test]
fn abs_value_positive() {
    assert_eq!(abs_value(5), 5);
}

#[test]
fn abs_value_negative() {
    assert_eq!(abs_value(-5), 5);
}

#[test]
fn abs_value_min_safe() {
    assert_eq!(abs_value(i32::MIN + 1), 2147483647);
}

#[test]
fn divide_ok() {
    assert_eq!(divide(10, 2), Some(5));
}

#[test]
fn divide_by_zero() {
    assert_eq!(divide(1, 0), None);
}

#[test]
fn parse_bool_true() {
    assert_eq!(parse_bool("true"), Some(true));
}

#[test]
fn parse_bool_false() {
    assert_eq!(parse_bool("false"), Some(false));
}

#[test]
fn parse_bool_rejects_yes() {
    assert_eq!(parse_bool("YES"), None);
}
