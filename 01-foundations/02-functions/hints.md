Try for at least 30 minutes before reading this.

## Hint 1

Rust slices `&[i32]` replace Go's variadic `...int`. Use `.iter().sum()` or a for loop to add numbers.

---

## Hint 2

For `divide`, check `b == 0` first and return `Err("division by zero".to_string())`. Integer division in Rust truncates toward zero automatically.

---

## Hint 3

```rust
pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("division by zero".to_string());
    }
    Ok(a / b)
}
```
