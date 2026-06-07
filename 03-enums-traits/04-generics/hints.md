Try for at least 30 minutes before reading this.

## Hint 1

Generic functions use `<T: Trait>` bounds. Compare with `>` only when `PartialOrd` is implemented.

---

## Hint 2

`max_of`: `if a >= b { a } else { b }`. `first`: `items.first()`.

---

## Hint 3

```rust
pub fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

pub fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```
