# Functions

## What you'll learn

- Function signatures and return types
- Slices as flexible input (`&[T]`)
- Returning `Result` for fallible operations

## Concept

Rust functions declare input types and a return type after `->`. The last expression (no semicolon) is the return value.

```rust
pub fn double(n: i32) -> i32 {
    n * 2
}
```

Use `Result<T, E>` when an operation can fail instead of panicking.

## API spec

| Function | Behavior |
|----------|----------|
| `sum(&[i32]) -> i32` | Sum all elements; empty slice → 0 |
| `divide(i32, i32) -> Result<i32, String>` | Integer division; error on divide by zero |

## Before moving on

- [ ] All tests pass: `cargo test`
