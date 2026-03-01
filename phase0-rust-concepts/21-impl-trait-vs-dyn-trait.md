# Concept 21: `impl Trait` vs `dyn Trait`

## Plain English

Both let you write functions that accept any type implementing a trait — but they work differently under the hood.

- `impl Trait` — resolved at **compile time** (static dispatch). Faster, no overhead.
- `dyn Trait` — resolved at **runtime** (dynamic dispatch). Flexible, small overhead.

---

## Code Example

```rust
trait Speak { fn speak(&self) -> &str; }

struct Dog;
struct Cat;
impl Speak for Dog { fn speak(&self) -> &str { "Woof" } }
impl Speak for Cat { fn speak(&self) -> &str { "Meow" } }

// impl Trait — compiler generates a concrete version at compile time
fn static_noise(animal: &impl Speak) {
    println!("{}", animal.speak());
}

// dyn Trait — type is resolved at runtime via a vtable pointer
fn dynamic_noise(animal: &dyn Speak) {
    println!("{}", animal.speak());
}

// dyn Trait shines here — mixed types in one collection
fn main() {
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &animals {
        dynamic_noise(a.as_ref());
    }
}
```

You can't do this with `impl Trait` — all items in a `Vec` must be the same concrete type.

---

## Comparison Table

| | `impl Trait` | `dyn Trait` |
|---|---|---|
| Dispatch | Static (compile time) | Dynamic (runtime) |
| Performance | Faster | Slight overhead (vtable) |
| Mixed types in collection | No | Yes |
| Use when | Single type per call site | Need runtime flexibility |

---

## Mini Quiz

**Q1.** What is the key difference between `impl Trait` and `dyn Trait`?
- A) `impl Trait` is resolved at compile time, `dyn Trait` at runtime ✓
- B) `impl Trait` only works with structs, `dyn Trait` works with enums

**Q2.** When would you use `dyn Trait` over `impl Trait`?
- A) When you need a collection of mixed types implementing the same trait ✓
- B) When you want better performance

---

## Quiz Answers

**Q1 → A.** `impl Trait` uses monomorphization — the compiler generates a separate version per type at compile time. `dyn Trait` uses a vtable pointer resolved at runtime.

**Q2 → A.** `dyn Trait` is for runtime flexibility — e.g. a `Vec<Box<dyn Trait>>` holding mixed types. If you want performance, use `impl Trait`.

---

## Next

→ [22-function-pointers.md](22-function-pointers.md)
