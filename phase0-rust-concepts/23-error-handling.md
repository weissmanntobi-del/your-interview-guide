# Concept 23: Error Handling — `Result`, `?`, Custom Errors

## Plain English

Rust doesn't have exceptions. Instead, functions that can fail return a `Result<T, E>`:
- `Ok(T)` — success, contains the value
- `Err(E)` — failure, contains the error

You handle errors explicitly — the compiler forces you to deal with them.

---

## Code Example

```rust
use std::num::ParseIntError;

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.trim().parse::<i32>()?;  // ? returns early if error
    Ok(n * 2)
}

fn main() {
    match parse_number("21") {
        Ok(n) => println!("Got: {}", n),    // Got: 42
        Err(e) => println!("Error: {}", e),
    }
}
```

- `?` operator — if the result is `Err`, return it immediately from the function
- `Ok(...)` — wrap a success value
- `match` — handle both cases explicitly

---

## Comparison Table

| | `unwrap()` | `?` operator | `match` |
|---|---|---|---|
| On error | Panics | Returns early | You decide |
| Use in production | Never | Yes | Yes |
| Verbosity | Low | Low | High |

---

## Mini Quiz

**Q1.** What does the `?` operator do?
- A) It panics if the result is `Err`
- B) It returns the `Err` early from the function if the result is `Err` ✓

**Q2.** What is the difference between `unwrap()` and `?`?
- A) `unwrap()` panics on error, `?` propagates the error to the caller ✓
- B) They do the same thing

---

## Quiz Answers

**Q1 → B.** `?` is shorthand for "if this is `Err`, return it immediately from the current function." It propagates errors up the call stack.

**Q2 → A.** `unwrap()` panics if the value is `Err` — never use it in production. `?` propagates the error gracefully to the caller.

---

## Next

→ [24-from-into-conversions.md](24-from-into-conversions.md)
