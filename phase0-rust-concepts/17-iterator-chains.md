# Concept 17: Iterator Chains (`map`, `filter`, `collect`)

## Plain English

An iterator is something you can loop over one item at a time. Rust's iterators are **lazy** — they don't do any work until you ask for the result.

You can chain operations on an iterator: transform items, filter them out, then collect the results into a new collection.

---

## Code Example

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = numbers
        .iter()                     // create an iterator
        .filter(|x| **x % 2 == 0)  // keep only even numbers
        .map(|x| x * 10)            // multiply each by 10
        .collect();                 // gather into a Vec

    println!("{:?}", result); // [20, 40]
}
```

- `.iter()` — starts the iterator
- `.filter(|x| ...)` — keeps items where the closure returns `true`
- `.map(|x| ...)` — transforms each item
- `.collect()` — consumes the iterator and builds a collection

---

## Key Point — Lazy Evaluation

Nothing runs until `.collect()` (or similar) is called. The chain is just a description of what to do — not doing it yet.

---

## Comparison Table

| Method | What it does |
|---|---|
| `.map(|x| ...)` | Transform each item |
| `.filter(|x| ...)` | Keep items where closure returns `true` |
| `.collect()` | Consume iterator, build a collection |
| `.sum()` | Add all items together |
| `.count()` | Count how many items |

---

## Mini Quiz

**Q1.** What does `.map(|x| x * 2)` do to an iterator?
- A) Removes items where `x * 2` is false
- B) Transforms each item by multiplying it by 2 ✓

**Q2.** Why do iterators need `.collect()` at the end?
- A) Because iterators are lazy — nothing runs until you consume them ✓
- B) Because `.map()` and `.filter()` don't work without it

---

## Quiz Answers

**Q1 → B.** `.map()` transforms each item. `.filter()` is for removing items.

**Q2 → A.** Iterators are lazy. The chain `.filter().map()` is just a blueprint — no work happens until a consuming method like `.collect()`, `.sum()`, or `.count()` is called.

---

## Next

→ [18-box.md](18-box.md)
