# Concept 18: `Box<T>` — Heap Allocation

## Plain English

`Box<T>` puts a value on the **heap** instead of the stack. You use it when:
- The size of a type isn't known at compile time
- You have a large value you don't want to copy
- You need a pointer to a trait object

Think of it as a **shipping box** — you put something inside, seal it, and it lives on the heap. The box itself (a pointer) lives on the stack.

---

## Code Example

```rust
fn main() {
    let x = Box::new(5);  // 5 is on the heap, x is a pointer on the stack
    println!("{}", x);    // works — Box auto-derefs
}
```

A common real use — **recursive types** (a type that contains itself):

```rust
// This would be infinite size — compiler rejects it
// enum List { Cons(i32, List) }

// Box fixes it — the pointer has a known size
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

---

## Comparison Table

| | Stack | `Box<T>` (Heap) |
|---|---|---|
| Size must be known at compile time | Yes | No |
| Fast access | Yes | Slightly slower |
| Ownership | Normal | Single owner |
| Use when | Small, known-size values | Large or recursive types |

---

## Mini Quiz

**Q1.** What does `Box<T>` do?
- A) It allows multiple owners of a value
- B) It allocates a value on the heap and gives you a pointer to it ✓

**Q2.** Why do recursive types like `enum List { Cons(i32, List) }` need `Box`?
- A) Because `Box` makes the type cheaper to copy
- B) Because without `Box`, the type would have infinite size — `Box` gives it a known pointer size ✓

---

## Quiz Answers

**Q1 → B.** `Box<T>` heap-allocates the value and gives you a single-owner smart pointer to it.

**Q2 → B.** A recursive type without `Box` would be infinitely large — the compiler can't determine its size. `Box` breaks the cycle by storing a fixed-size pointer instead.

---

## Next

→ [19-rc.md](19-rc.md)
