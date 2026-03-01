# Concept 13: Trait Bounds

## Plain English

A trait bound is a **constraint you put on a generic type**. It says: "I'll accept any type `T`, but only if `T` implements this trait."

Without a trait bound, `T` is completely unknown — you can't call any methods on it because Rust doesn't know what it supports. A trait bound narrows it down: "T must be able to do at least this."

---

## Code Example

```rust
use std::fmt::Display;

// T must implement Display — so we can print it
fn print_twice<T: Display>(value: T) {
    println!("{}", value);
    println!("{}", value);
}

fn main() {
    print_twice(42);        // works — i32 implements Display
    print_twice("hello");   // works — &str implements Display
    // print_twice(vec![1]); // ERROR — Vec doesn't implement Display
}
```

---

## Multiple Bounds with `+`

You can require a type to implement **more than one** trait:

```rust
fn print_and_compare<T: Display + PartialOrd>(a: T, b: T) {
    if a > b {
        println!("{} is bigger", a);
    } else {
        println!("{} is bigger", b);
    }
}
```

`T: Display + PartialOrd` means T must support both printing and comparison.

---

## `where` Clause (cleaner syntax for complex bounds)

When bounds get long, use a `where` clause to keep the signature readable:

```rust
// Instead of this:
fn foo<T: Display + PartialOrd, U: Display>(a: T, b: U) { }

// Write this:
fn foo<T, U>(a: T, b: U)
where
    T: Display + PartialOrd,
    U: Display,
{ }
```

Same meaning, cleaner to read.

---

## Comparison Table

| Syntax | Meaning |
|---|---|
| `T: Trait` | T must implement Trait |
| `T: Trait + Trait2` | T must implement both |
| `where T: Trait` | Same as above, cleaner for long bounds |
| `&impl Trait` | Shorthand for a single trait bound in fn args |

---

## Mini Quiz

**Q1.** Why do you need `T: PartialOrd` to use `>` on a generic `T`?
- A) Because all types support `>` by default
- B) Because without the bound, Rust doesn't know if T supports comparison ✓

**Q2.** What does `T: Display + Clone` mean?
- A) T must implement either Display or Clone
- B) T must implement both Display and Clone ✓

---

## Quiz Answers

**Q1 → B.** Without the bound, Rust refuses to compile — it has no guarantee that `T` supports `>`. The bound is your promise to the compiler.

**Q2 → B.** `+` means AND, not OR. T must implement both traits.

---

## Next

→ [14-lifetimes.md](14-lifetimes.md)
