# Concept 20: `RefCell<T>` — Interior Mutability

## Plain English

`RefCell<T>` lets you **mutate a value even when you only have an immutable reference** to it. This is called **interior mutability**.

Normally Rust enforces borrow rules at compile time. `RefCell<T>` moves those checks to **runtime** — if you break the rules, it panics instead of failing to compile.

Use it when you know the borrow rules are being followed but the compiler can't prove it.

---

## Code Example

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    data.borrow_mut().push(4);  // mutably borrow at runtime
    println!("{:?}", data.borrow()); // [1, 2, 3, 4]
}
```

- `.borrow()` — immutable borrow (runtime checked)
- `.borrow_mut()` — mutable borrow (runtime checked)

---

## Comparison Table

| | Normal borrow | `RefCell<T>` |
|---|---|---|
| Borrow rules checked | Compile time | Runtime |
| Violation result | Compile error | Panic |
| Use when | Always prefer this | Compiler can't prove safety |

---

## Mini Quiz

**Q1.** What does `RefCell<T>` allow that normal references don't?
- A) Multiple thread access to the same value
- B) Mutating a value through an immutable reference (interior mutability) ✓

**Q2.** What happens if you violate borrow rules with `RefCell<T>`?
- A) Compile error
- B) Runtime panic ✓

---

## Quiz Answers

**Q1 → B.** `RefCell<T>` provides interior mutability — you can mutate the inner value even when holding only an immutable reference to the `RefCell`. Thread safety is a different concern (`Mutex` handles that).

**Q2 → B.** `RefCell<T>` moves borrow checking to runtime. Violating the rules (e.g. two mutable borrows at once) causes a panic, not a compile error.

---

## Next

→ [21-impl-trait-vs-dyn-trait.md](21-impl-trait-vs-dyn-trait.md)
