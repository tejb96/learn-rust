Try for at least 30 minutes before reading this.

## Hint 1

`Vec` is growable; `HashMap::new()` then `.entry(key).and_modify(...).or_insert(...)` is idiomatic for counting.

---

## Hint 2

For `filter_long_words`, iterate and push when `word.len() > min_len`. For `top_key`, iterate entries and track max count with tie-break using string comparison.

---

## Hint 3

Use `.iter().filter(...).map(|s| s.to_string()).collect()` for filtering. For counts, a simple loop with `*counts.entry(w.to_string()).or_insert(0) += 1` works well.
