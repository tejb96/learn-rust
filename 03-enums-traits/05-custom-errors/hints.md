Try for at least 30 minutes before reading this.

## Hint 1

`thiserror` derives `Error` and `Display` from enum variants. Use `#[error("...")]` on each variant.

---

## Hint 2

Check `id.is_empty()` first for Validation. Use `store.get(id).cloned().ok_or(AppError::NotFound)`.

---

## Hint 3

```rust
pub fn wrap(err: AppError, msg: &str) -> String {
    format!("{msg}: {err}")
}
```
