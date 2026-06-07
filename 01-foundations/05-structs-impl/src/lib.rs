#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width: 0, height: 0 }
    }

    pub fn area(&self) -> u32 {
        0
    }

    pub fn is_square(&self) -> bool {
        false
    }

    pub fn scale(&mut self, factor: u32) {
    }
}
