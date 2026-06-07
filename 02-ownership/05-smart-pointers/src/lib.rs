use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    pub fn new() -> Rc<Self> {
        Rc::new(List::Nil)
    }

    pub fn prepend(value: i32, tail: Rc<List>) -> Rc<List> {
        Rc::new(List::Nil)
    }

    pub fn sum(list: &List) -> i32 {
        0
    }
}

/// Builds a shared list `1 -> 2 -> 3` using `Rc`.
pub fn shared_list() -> Rc<List> {
    List::new()
}
