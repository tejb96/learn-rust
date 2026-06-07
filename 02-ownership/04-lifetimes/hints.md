Try for at least 30 minutes before reading this.

## Hint 1

Lifetime `'a` on a struct field means the struct cannot outlive the data it borrows.

---

## Hint 2

For `first_sentence`, slice up to the first `.` inclusive: `text.find('.').map(|i| &text[..=i]).unwrap_or(text)`.

---

## Hint 3

```rust
pub fn first_sentence(text: &'a str) -> Self {
    let part = text.find('.').map(|i| &text[..=i]).unwrap_or(text);
    Self { part }
}
```
