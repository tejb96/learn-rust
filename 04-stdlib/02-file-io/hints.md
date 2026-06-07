Try for at least 30 minutes before reading this.

## Hint 1

Use `fs::write` or `writeln!` to a `File`. Read with `fs::read_to_string` and split lines.

---

## Hint 2

Trim each line with `.trim()`. Skip empty after trim. Filter grep with `.contains(needle)`.

---

## Hint 3

```rust
pub fn write_lines(path: &Path, lines: &[&str]) -> io::Result<()> {
    fs::write(path, lines.join("\n"))
}
```
