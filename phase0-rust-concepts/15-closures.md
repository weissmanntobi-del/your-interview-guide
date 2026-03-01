# Concept 15: Closures

## Plain English

A closure is an **anonymous function that can capture variables from its surrounding scope**.

A regular function can't access variables outside of itself — you have to pass everything in as arguments. A closure can grab variables from the surrounding code automatically — it "closes over" its environment and remembers those variables.

---

## The Backpack Analogy

Imagine you're going on a trip. You pack a backpack before you leave home. Once you're on the trip, you open the backpack and use what's inside — even though those things came from home, not from the trip itself.

That's a closure. It packs variables from its surroundings and carries them wherever it goes.

---

## Code Example

```rust
let bonus = 100;  // set once, up here

let total = |salary| salary + bonus;
//           ^^^^^^   ^^^^^^^^^^^^^^
//           input    uses salary (input) + bonus (captured from above)

println!("{}", total(500)); // 600 — only pass salary, bonus is remembered
println!("{}", total(600)); // 700
```

- `salary` = what you pass in each time (it changes)
- `bonus` = what the closure captured from outside (stays the same, already remembered)

---

## Closure vs Function

```rust
// Regular function — must pass everything in
fn total(salary: i32, bonus: i32) -> i32 { salary + bonus }
total(500, 100); // have to pass bonus every time

// Closure — bakes in bonus, only asks for what changes
let bonus = 100;
let total = |salary| salary + bonus;
total(500); // bonus is already remembered
```

---

## Comparison Table

| | Function | Closure |
|---|---|---|
| Syntax | `fn name(params) -> T` | `\|params\| body` |
| Captures environment | No | Yes |
| Has a name | Yes | No (anonymous) |
| Used inline | No | Yes |

---

## Mini Quiz

**Q1.** What makes a closure different from a regular function?
- A) Closures can only take one argument
- B) Closures can capture variables from the surrounding scope ✓

**Q2.** What is the syntax for a closure that takes `x` and returns `x * 2`?
- A) `fn(x) { x * 2 }`
- B) `|x| x * 2` ✓

---

## Quiz Answers

**Q1 → B.** Closures capture variables from the surrounding scope — that's the key difference from regular functions.

**Q2 → B.** `|x| x * 2` is the closure syntax. The pipes `|` replace parentheses to visually distinguish closures from regular functions.

---

## Next

→ [16-fn-fnmut-fnonce.md](16-fn-fnmut-fnonce.md)
