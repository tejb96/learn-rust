// DO NOT EDIT — implement the solution in src/lib.rs

use generics_lesson::{first, max_of, Pair};

#[test]
fn max_of_ints() {
    assert_eq!(max_of(3, 7), 7);
}

#[test]
fn max_of_chars() {
    assert_eq!(max_of('a', 'z'), 'z');
}

#[test]
fn pair_swap() {
    let p = Pair::new(1, "hi");
    assert_eq!(p.swap(), Pair::new("hi", 1));
}

#[test]
fn first_some() {
    let data = [10, 20];
    assert_eq!(first(&data), Some(&10));
}

#[test]
fn first_none() {
    let data: [i32; 0] = [];
    assert_eq!(first(&data), None);
}
