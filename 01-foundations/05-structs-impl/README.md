# Structs and impl

## What you'll learn

- Defining structs with named fields
- `impl` blocks and methods
- `Self` as shorthand for the struct type

## API spec

| Method | Behavior |
|--------|----------|
| `new(w, h)` | Construct a `Rectangle` |
| `area` | `width * height` |
| `is_square` | true when width == height |
| `scale(factor)` | Multiply width and height in place |

## Before moving on

- [ ] All tests pass: `cargo test`
