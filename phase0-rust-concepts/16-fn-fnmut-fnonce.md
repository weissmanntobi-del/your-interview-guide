# Concept 16: `Fn`, `FnMut`, `FnOnce`

## Plain English

Every closure in Rust automatically implements one (or more) of three traits depending on **how it uses the variables it captured**:

| Trait | What the closure does with captured variables |
|---|---|
| `Fn` | Only reads them — can be called many times |
| `FnMut` | Modifies them — can be called many times |
| `FnOnce` | Consumes them — can only be called **once** |

Think of it as: how rough is the closure with what it captured?

---

## Code Example

```rust
// Fn — just reads (shared borrow)
let name = String::from("Alice");
let greet = || println!("Hello, {}", name);
greet(); // works
greet(); // works again — name still exists

// FnMut — modifies (mutable borrow)
let mut count = 0;
let mut increment = || {
    count += 1;
    println!("{}", count);
};
increment(); // 1
increment(); // 2

// FnOnce — consumes (takes ownership)
let name = String::from("Alice");
let consume = move || {
    println!("Hello, {}", name);
    // name is moved into closure — gone after this call
};
consume(); // works
// consume(); // ERROR — name was consumed
```

---

## Comparison Table

| Trait | Captures by | Callable | Requires `mut`? |
|---|---|---|---|
| `Fn` | Shared borrow `&T` | Many times | No |
| `FnMut` | Mutable borrow `&mut T` | Many times | Yes |
| `FnOnce` | Ownership (move) | Once only | No |

---

## Mini Quiz

**Q1.** A closure that only reads a captured variable implements which trait?
- A) `FnOnce`
- B) `Fn` ✓
- C) `FnMut`

**Q2.** Why can an `FnOnce` closure only be called once?
- A) It's a Rust syntax limitation
- B) It takes ownership of the captured variable, so it's consumed after the first call ✓

---

## Quiz Answers

**Q1 → B.** A closure that only reads captured variables implements `Fn` — it borrows immutably and can be called any number of times.

**Q2 → B.** `FnOnce` moves the captured value into the closure. After the first call, that value is consumed and no longer exists — calling it again would be a use-after-move.

---

## Next

→ [17-iterator-chains.md](17-iterator-chains.md)
