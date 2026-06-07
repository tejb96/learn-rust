Try for at least 30 minutes before reading this.

## Hint 1

`String` owns its data. When a function takes `String` (not `&str`), the caller transfers ownership.

---

## Hint 2

Use `s.to_uppercase()` in `shout`. In `concat_owned`, format or push strings together.

---

## Hint 3

```rust
pub fn shout(s: String) -> String {
    s.to_uppercase()
}

pub fn concat_owned(a: String, b: String) -> String {
    format!("{a}{b}")
}

pub fn len_copy(s: &str) -> usize {
    s.len()
}
```
