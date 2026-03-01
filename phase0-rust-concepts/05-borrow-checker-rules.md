# Concept 5: The Borrow Checker Rules (Complete Summary)

This is your cheat sheet. Everything from Concepts 1–4 in one place.

---

## The 7 Rules

### Ownership (Concepts 1–2)

| # | Rule |
|---|------|
| 1 | Every value has exactly **one owner** |
| 2 | There can only be **one owner at a time** |
| 3 | When the owner goes out of scope, the value is **dropped** (freed) |
| 4 | Assignment, passing to a function, or returning moves ownership for heap types (`String`, `Vec`) |
| 5 | Simple types (`i32`, `u32`, `bool`, `char`, `f64`) are **copied** instead of moved |

### Borrowing (Concepts 3–4)

| # | Rule |
|---|------|
| 6 | You can have **either** many `&` (immutable borrows) **OR** one `&mut` (mutable borrow) — **never both** |
| 7 | References must always be **valid** — they cannot outlive the data they point to |

---

## Rule 7: No Dangling References

```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s   // COMPILER ERROR — s is about to be dropped!
}        // s is dropped here — the reference would point to freed memory
```

Rust will not let you return a reference to something that is about to be destroyed. Fix: return the owned value instead.

```rust
fn not_dangling() -> String {
    let s = String::from("hello");
    s    // move ownership out — caller now owns it
}
```

---

## Decision Flowchart

When writing a function parameter, ask yourself:

```
Do I need to OWN this data?
├── Yes → use String / Vec<T> / T
│         (caller loses it)
│
└── No, just borrow
    │
    ├── Do I need to MODIFY it?
    │   ├── Yes → use &mut T
    │   └── No  → use &T (or &str for strings)
```

---

## Common Mistakes and Fixes

| Mistake | Error you get | Fix |
|---------|--------------|-----|
| Using a variable after moving it | `use of moved value` | Borrow (`&`) instead of move |
| Taking `&mut` without `let mut` | `cannot borrow as mutable` | Add `mut` to the variable declaration |
| Mixing `&` and `&mut` | `cannot borrow as mutable because it is also borrowed as immutable` | Finish using `&` before taking `&mut` |
| Returning a reference to a local variable | `returns a reference to data owned by the current function` | Return the owned value instead |

---

## Borrows Have Lifetimes

A borrow is only "alive" from where it is created to where it is **last used**:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);
// r1 and r2 are DONE here — never used again

let m1 = &mut s;    // WORKS — r1 and r2 are no longer alive
m1.push_str("!");
```

This compiles! Even though `r1` and `&mut` are in the same block, `r1` is **no longer used** after the `println!`. Rust sees that the immutable borrows ended before the mutable borrow starts.

---

## All 7 Rules — One Line Each

1. One owner per value
2. One owner at a time
3. Owner drops → value freed
4. Heap types move, not copy
5. Primitive types copy, not move
6. Many `&` OR one `&mut`, never both
7. References cannot outlive their data

---

## Reference Card

| Syntax | Name | Can read? | Can modify? | How many at once? |
|--------|------|-----------|-------------|-------------------|
| `T` (owned) | Owner | Yes | Yes | One |
| `&T` | Immutable reference | Yes | No | Many |
| `&mut T` | Mutable reference | Yes | Yes | One (and no `&T` at same time) |

---

## Next

→ [Concept 6: Slices — `&str` and `&[T]`](06-slices.md)
