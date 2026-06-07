// DO NOT EDIT — implement the solution in src/lib.rs

use structs_impl::Rectangle;

#[test]
fn new_and_area() {
    let rect = Rectangle::new(4, 5);
    assert_eq!(rect.area(), 20);
}

#[test]
fn is_square_true() {
    let rect = Rectangle::new(3, 3);
    assert!(rect.is_square());
}

#[test]
fn is_square_false() {
    let rect = Rectangle::new(3, 4);
    assert!(!rect.is_square());
}

#[test]
fn scale_mutates() {
    let mut rect = Rectangle::new(2, 3);
    rect.scale(2);
    assert_eq!(rect, Rectangle::new(4, 6));
}
