#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// Returns a short description of the message.
pub fn describe(msg: &Message) -> String {
    String::new()
}

/// Applies a message to a position `(x, y)`, returning the new position.
pub fn apply_move(pos: (i32, i32), msg: &Message) -> (i32, i32) {
    pos
}
