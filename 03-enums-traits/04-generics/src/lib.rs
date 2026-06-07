/// Returns the larger of two values. Requires `PartialOrd`.
pub fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    a
}

/// A pair of two values of possibly different types.
#[derive(Debug, PartialEq, Eq)]
pub struct Pair<A, B> {
    pub first: A,
    pub second: B,
}

impl<A, B> Pair<A, B> {
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }

    pub fn swap(self) -> Pair<B, A> {
        Pair::new(self.second, self.first)
    }
}

/// Returns the first element of a slice, or `None` if empty.
pub fn first<T>(items: &[T]) -> Option<&T> {
    None
}
