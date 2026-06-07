Try for at least 30 minutes before reading this.

## Hint 1

`Box` owns heap data; `Rc` enables multiple owners of the same data (single-threaded).

---

## Hint 2

Build the list from tail to head: `prepend(3, prepend(2, prepend(1, Nil)))`. `sum` uses `match` recursively.

---

## Hint 3

```rust
pub fn sum(list: &List) -> i32 {
    match list {
        List::Cons(v, tail) => v + Self::sum(tail),
        List::Nil => 0,
    }
}
```
