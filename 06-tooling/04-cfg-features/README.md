# cfg Features

## What you'll learn

- Cargo features in `[features]`
- `#[cfg(feature = "...")]` conditional compilation
- Running tests with `--features`

Default tests pass with `cargo test`. Feature tests require:

```bash
cargo test --features enabled
```

## Before moving on

- [ ] Default tests pass: `cargo test`
- [ ] Feature tests pass: `cargo test --features enabled`
