Try for at least 30 minutes before reading this.

## Hint 1

Split on first whitespace: `splitn(2, char::is_whitespace)` or find first space index.

---

## Hint 2

Reject lines that trim to empty. Compare levels with `.eq_ignore_ascii_case`.

---

## Hint 3

`format_json` uses `serde_json::to_string(entries)`.
