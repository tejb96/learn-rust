// DO NOT EDIT — implement the solution in src/lib.rs

use result_option::{first_even, parse_port, safe_divide};

#[test]
fn parse_port_valid() {
    assert_eq!(parse_port("8080"), Some(8080));
}

#[test]
fn parse_port_invalid() {
    assert_eq!(parse_port("abc"), None);
}

#[test]
fn parse_port_empty() {
    assert_eq!(parse_port(""), None);
}

#[test]
fn first_even_found() {
    assert_eq!(first_even(&[1, 3, 4, 5]), Some(4));
}

#[test]
fn first_even_none() {
    assert_eq!(first_even(&[1, 3, 5]), None);
}

#[test]
fn safe_divide_ok() {
    assert!((safe_divide(10.0, 2.0).unwrap() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn safe_divide_zero() {
    assert!(safe_divide(1.0, 0.0).is_err());
}
