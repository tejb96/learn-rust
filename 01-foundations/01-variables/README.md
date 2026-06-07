# Variables and Default Values

## What you'll learn

- Rust's basic types and their default values
- Mutability with `let` and `let mut`
- Returning multiple values with tuples

## Concept

Every type in Rust has a known default. Numbers default to zero, `bool` to `false`, `String` to empty, and collections like `Vec` start empty when created with `Vec::new()` or `Default::default()`.

Functions return a single value, but that value can be a **tuple** — Rust's way to return multiple results without out-parameters.

```rust
fn swap(a: &str, b: &str) -> (String, String) {
    (b.to_string(), a.to_string())
}
```

## Common mistakes

- Assuming `String` can be `null` — use `Option<String>` for optional text.
- Forgetting `.to_string()` when converting `&str` to owned `String`.
- Using `== ""` when `is_empty()` is clearer.

## Further reading

- [The Rust Book — Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

## API spec

Implement in `src/lib.rs`:

| Function | Signature | Behavior |
|----------|-----------|----------|
| `swap` | `(a: &str, b: &str) -> (String, String)` | Return `(b, a)` as owned strings |
| `describe_default` | `(kind: &str) -> Result<String, String>` | For `"string"`, `"int"`, `"bool"`, `"slice"` return exact description strings; otherwise `Err` |
| `default_greeting` | `(name: &str) -> String` | Empty name → `"Hello, stranger!"`; else `"Hello, <name>!"` |

## Before moving on

- [ ] I can name the default for `i32`, `String`, `bool`, and `Vec`
- [ ] I can return a tuple from a function
- [ ] All tests pass: `cargo test`
