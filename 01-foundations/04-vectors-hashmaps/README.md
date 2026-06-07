# Vectors and HashMaps

## What you'll learn

- `Vec<T>` for ordered collections
- `HashMap<K, V>` for key-value lookup
- Basic iterator patterns

## API spec

| Function | Behavior |
|----------|----------|
| `filter_long_words` | Words with length > `min_len`, first-seen order |
| `word_counts` | Frequency map of each word |
| `top_key` | Key with highest count; tie → smallest key lexicographically |

## Before moving on

- [ ] All tests pass: `cargo test`
