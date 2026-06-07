# Ownership Basics

## What you'll learn

- Move semantics for owned types like `String`
- When ownership transfers to a function
- Borrowing with `&str` to read without taking ownership

## API spec

| Function | Behavior |
|----------|----------|
| `shout(String) -> String` | Uppercase the owned string |
| `concat_owned(String, String) -> String` | Concatenate two owned strings |
| `len_copy(&str) -> usize` | Return byte length |

## Before moving on

- [ ] All tests pass: `cargo test`
