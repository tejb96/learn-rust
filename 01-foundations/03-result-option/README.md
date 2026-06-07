# Result and Option

## What you'll learn

- `Option<T>` for optional values
- `Result<T, E>` for operations that can fail
- `match`, `if let`, and the `?` operator basics

## API spec

| Function | Behavior |
|----------|----------|
| `parse_port` | Parse `u16`; `None` on empty or invalid |
| `first_even` | First even number in slice, or `None` |
| `safe_divide` | Float division; `Err` on divide by zero |

## Before moving on

- [ ] All tests pass: `cargo test`
