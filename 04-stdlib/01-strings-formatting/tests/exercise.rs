// DO NOT EDIT — implement the solution in src/lib.rs

use strings_formatting::{format_person, join_words, Person};

#[test]
fn format_person_works() {
    assert_eq!(format_person("Ada", 36), "Ada (age 36)");
}

#[test]
fn join_words_works() {
    assert_eq!(join_words(&["a", "b", "c"]), "a, b, c");
}

#[test]
fn join_words_empty() {
    assert_eq!(join_words(&[]), "");
}

#[test]
fn display_person() {
    let p = Person {
        name: "Grace".into(),
        age: 37,
    };
    assert_eq!(p.to_string(), "Grace (age 37)");
}
