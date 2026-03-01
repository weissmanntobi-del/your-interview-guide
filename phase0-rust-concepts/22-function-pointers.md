# Concept 22: Function Pointers

## Plain English

A function pointer is a variable that **holds a reference to a function** — not a closure, but a plain named function. You can pass functions around as values.

The type is written as `fn(ParamType) -> ReturnType`.

---

## Code Example

```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn multiply(a: i32, b: i32) -> i32 { a * b }

fn apply(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)  // call whichever function was passed in
}

fn main() {
    println!("{}", apply(add, 3, 4));       // 7
    println!("{}", apply(multiply, 3, 4));  // 12
}
```

---

## Function Pointer vs Closure

| | Function pointer `fn()` | Closure |
|---|---|---|
| Captures environment | No | Yes |
| Syntax | `fn(T) -> T` | `\|x\| ...` |
| Stored as type | `fn(T) -> T` | `Fn`/`FnMut`/`FnOnce` |

```rust
let x = 10;
let closure = |a| a + x;          // closure — captures x
fn func(a: i32) -> i32 { a + 1 }  // fn pointer — captures nothing
```

---

## Mini Quiz

**Q1.** What is a function pointer in Rust?
- A) A closure that captures variables
- B) A variable that holds a reference to a named function ✓

**Q2.** What is the main difference between a function pointer and a closure?
- A) Function pointers can't take arguments
- B) Function pointers can't capture variables from the surrounding scope ✓

---

## Quiz Answers

**Q1 → B.** A function pointer holds a reference to a named `fn` — not a closure. It has no captured state.

**Q2 → B.** Function pointers are stateless — they can't capture surrounding variables. Closures can.

---

## Next

→ [23-error-handling.md](23-error-handling.md)
