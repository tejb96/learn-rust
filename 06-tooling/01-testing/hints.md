Try for at least 30 minutes before reading this.

## Hint 1

Read each failing test carefully — the bugs are in `src/lib.rs`, not the tests.

---

## Hint 2

`i32::MIN` cannot be negated in signed 32-bit. Handle that edge case. Remove `"YES"` and `"NO"` from `parse_bool`.

---

## Hint 3

For `abs_value`, return `i32::MAX` or use `n.abs()` which saturates — tests expect `abs_value(i32::MIN + 1)` only, so `-n` works for values > MIN.
