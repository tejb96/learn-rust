// DO NOT EDIT — implement the solution in src/lib.rs

use ownership_basics::{concat_owned, len_copy, shout};

#[test]
fn shout_uppercases() {
    assert_eq!(shout("hello".to_string()), "HELLO");
}

#[test]
fn concat_owned_joins() {
    assert_eq!(concat_owned("foo".to_string(), "bar".to_string()), "foobar");
}

#[test]
fn len_copy_counts() {
    assert_eq!(len_copy("rust"), 4);
}
