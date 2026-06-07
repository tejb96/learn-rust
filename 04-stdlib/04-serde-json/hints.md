Try for at least 30 minutes before reading this.

## Hint 1

Add `#[derive(Serialize, Deserialize)]` on the struct. Use `serde_json::to_string` and `from_str`.

---

## Hint 2

`active_users`: `.iter().filter(|u| u.active).cloned().collect()`.

---

## Hint 3

```rust
pub fn to_json(user: &User) -> Result<String, serde_json::Error> {
    serde_json::to_string(user)
}
```
