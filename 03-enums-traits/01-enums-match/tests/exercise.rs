// DO NOT EDIT — implement the solution in src/lib.rs

use enums_match::{apply_move, describe, Message};

#[test]
fn describe_quit() {
    assert_eq!(describe(&Message::Quit), "quit");
}

#[test]
fn describe_move() {
    let msg = Message::Move { x: 1, y: 2 };
    assert_eq!(describe(&msg), "move to (1, 2)");
}

#[test]
fn describe_write() {
    assert_eq!(describe(&Message::Write("hi".into())), "write: hi");
}

#[test]
fn apply_move_updates() {
    let msg = Message::Move { x: 3, y: -1 };
    assert_eq!(apply_move((0, 0), &msg), (3, -1));
}

#[test]
fn apply_move_ignores_non_move() {
    assert_eq!(apply_move((1, 1), &Message::Quit), (1, 1));
}
