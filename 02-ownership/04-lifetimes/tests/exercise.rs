// DO NOT EDIT — implement the solution in src/lib.rs

use lifetimes_lesson::{pick_longer, Excerpt};

#[test]
fn excerpt_new() {
    let text = "hello world";
    let ex = Excerpt::new(&text[0..5]);
    assert_eq!(ex.part, "hello");
}

#[test]
fn first_sentence() {
    let text = "Hello. World.";
    let ex = Excerpt::first_sentence(text);
    assert_eq!(ex.part, "Hello.");
}

#[test]
fn pick_longer_second() {
    assert_eq!(pick_longer("hi", "hello"), "hello");
}

#[test]
fn pick_longer_equal() {
    assert_eq!(pick_longer("ab", "xy"), "ab");
}
