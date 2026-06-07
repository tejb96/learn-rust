// DO NOT EDIT — implement the solution in src/lib.rs

use collections::{filter_long_words, top_key, word_counts};
use std::collections::HashMap;

#[test]
fn filter_long_words_basic() {
    let words = ["a", "hi", "hello", "go"];
    assert_eq!(
        filter_long_words(&words, 2),
        vec!["hello".to_string()]
    );
}

#[test]
fn word_counts_basic() {
    let words = ["go", "rust", "go"];
    let counts = word_counts(&words);
    assert_eq!(counts.get("go"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&1));
}

#[test]
fn top_key_basic() {
    let mut counts = HashMap::new();
    counts.insert("go".to_string(), 2);
    counts.insert("rust".to_string(), 5);
    assert_eq!(top_key(&counts), Some("rust".to_string()));
}

#[test]
fn top_key_tie() {
    let mut counts = HashMap::new();
    counts.insert("b".to_string(), 1);
    counts.insert("a".to_string(), 1);
    assert_eq!(top_key(&counts), Some("a".to_string()));
}

#[test]
fn top_key_empty() {
    let counts = HashMap::new();
    assert_eq!(top_key(&counts), None);
}
