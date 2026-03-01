# Concept 11: Generics `<T>`

## Plain English

A generic lets you write one function or struct that works with **many different types** instead of writing a separate version for each type.

Think of it like a **template**. You write the shape once, and the compiler fills in the actual type when it's used.

Without generics, if you want a function that returns the largest item in a list, you'd have to write one for `i32`, one for `f64`, one for `u8`, etc. With generics, you write it **once** with a placeholder type `T`, and Rust figures out the real type at compile time. No runtime cost — it's all resolved before your program runs.

---

## Code Example

```rust
// A generic function — works for any type T
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];

    for item in list {
        if item > biggest {   // requires PartialOrd to compare
            biggest = item;
        }
    }

    biggest
}

fn main() {
    let numbers = vec![10, 40, 25, 7];
    let words = vec!["banana", "apple", "cherry"];

    println!("{}", largest(&numbers)); // 40
    println!("{}", largest(&words));   // cherry
}
```

**Key things to notice:**
- `<T>` declares a placeholder type
- `T: PartialOrd` is a **trait bound** — it says "T must support comparison" (covered in Concept 13)
- The compiler generates a **separate concrete version** for each type you use — this is called **monomorphization**

---

## Generics in Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point { x: 1, y: 2 };       // T = i32
    let float_point = Point { x: 1.5, y: 2.5 }; // T = f64
}
```

Both `x` and `y` must be the **same type** here because there's only one `T`. If you want them to be different types, you need two parameters: `Point<T, U>`.

---

## Comparison Table

| | Non-Generic | Generic |
|---|---|---|
| Works with | One specific type | Any type matching the bound |
| Code duplication | High (one fn per type) | None |
| Runtime cost | None | None (resolved at compile time) |
| Flexibility | Low | High |

---

## Key Vocabulary

| Term | Meaning |
|---|---|
| `<T>` | Type parameter — a placeholder |
| Monomorphization | Compiler generates concrete versions per type |
| Trait bound (`T: Trait`) | Constraint on what T must support |
| Concrete type | The actual type substituted in (e.g. `i32`) |

---

## Mini Quiz

**Q1.** What does `<T>` mean in `fn foo<T>(x: T)`?
- A) T is a value, not a type
- B) T is a placeholder for any type
- C) T must always be a number

**Q2.** If you write `struct Pair<T> { a: T, b: T }`, can `a` and `b` hold different types?
- A) Yes, T can vary per field
- B) No, both must be the same type — use `<T, U>` for different types

**Q3.** True or False: Generics add runtime overhead compared to writing separate functions for each type.
- A) True
- B) False — generics are resolved at compile time (monomorphization)

---

## Quiz Answers

**Q1 → B.** `T` is a placeholder. It stands for "some type, determined when the function is called." The name `T` is just convention — you could name it anything.

**Q2 → B.** With one `T`, both fields share the same type. `Pair<T, U>` lets them differ.

**Q3 → B (False).** Monomorphization means the compiler generates separate concrete functions per type at compile time. Zero runtime cost — identical to writing them by hand.

---

## Next

→ [12-traits.md](12-traits.md)
