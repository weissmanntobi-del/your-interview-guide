# Concept 6: Slices — `&[T]` and `&str`

Slices are **views into a contiguous sequence of data** — you don't own the data, you just look at a portion of it. You already know `&str` (a string slice) from Concept 3. Now we generalize it to any type.

---

## What is a Slice?

A slice is a **reference to a range of elements** in an array, vector, or string. It has two pieces of info:
- A pointer to the first element
- A length (how many elements)

```rust
let numbers = [10, 20, 30, 40, 50];  // array of 5 elements
let slice = &numbers[1..4];           // slice: [20, 30, 40]
//          ^ borrow  ^ range: index 1, 2, 3 (not 4)
```

`slice` is a **variable** — it holds a `&[i32]` value. It doesn't own any data. It's a window into `numbers`.

---

## Slice Syntax

```rust
let a = [1, 2, 3, 4, 5];

&a[1..4]   // [2, 3, 4]        — from index 1 up to (not including) 4
&a[..3]    // [1, 2, 3]        — from start up to 3
&a[2..]    // [3, 4, 5]        — from index 2 to the end
&a[..]     // [1, 2, 3, 4, 5]  — the whole thing
&a[1..=3]  // [2, 3, 4]        — inclusive range (includes index 3)
```

### Range cheat sheet

| Range | Means | Example on `[10,20,30,40,50]` |
|---|---|---|
| `1..4` | index 1, 2, 3 (stop before 4) | `[20, 30, 40]` |
| `1..=4` | index 1, 2, 3, 4 (includes 4) | `[20, 30, 40, 50]` |
| `..3` | index 0, 1, 2 | `[10, 20, 30]` |
| `2..` | index 2 to end | `[30, 40, 50]` |
| `..` | everything | `[10, 20, 30, 40, 50]` |

**Key:** `..` is always **exclusive** on the right. Use `..=` to include the last index.

---

## Array Slices: `&[T]`

```rust
fn sum(numbers: &[i32]) -> i32 {
//              ^^^^^^ slice of i32 values — any length
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

let array = [1, 2, 3, 4, 5];
let vec = vec![10, 20, 30];

sum(&array);        // works — borrows the whole array as a slice
sum(&array[1..3]);  // works — borrows elements [2, 3]
sum(&vec);          // works — Vec auto-converts to slice
```

---

## Why `&[T]` instead of `&Vec<T>` or `&[T; N]`

This is the same logic as using `&str` instead of `&String`:

| Parameter | Accepts |
|---|---|
| `&Vec<i32>` | Only a borrowed Vec |
| `&[i32; 3]` | Only a reference to exactly a 3-element array |
| `&[i32]` | `&Vec`, arrays of **any** size, subranges, other slices |

### `&[T; N]` explained

`[T; N]` is a fixed-size array where the size `N` is part of the type. `[i32; 3]` and `[i32; 5]` are **different types**:

```rust
fn sum_strict(numbers: &[i32; 3]) -> i32 {
    numbers[0] + numbers[1] + numbers[2]
}

let a = [10, 20, 30];
let b = [1, 2, 3, 4, 5];

sum_strict(&a);   // works — exactly 3 elements
sum_strict(&b);   // COMPILER ERROR — b has 5 elements, not 3
```

Fix: use `&[i32]` and it works with any size.

### The rule

| For strings | For collections |
|---|---|
| Use `&str` instead of `&String` | Use `&[T]` instead of `&Vec<T>` |
| More flexible, same functionality | More flexible, same functionality |

**Rule of thumb:** If a function only reads a collection, take `&[T]`. It accepts data from any source — Vec, array, subrange.

---

## Slices are Borrowed — Ownership Rules Apply

Slices are references, so all borrow checker rules from Concepts 3-5 apply:

```rust
let mut v = vec![1, 2, 3];
let slice = &v[..];   // immutable borrow of v — slice is alive
v.push(4);             // COMPILER ERROR — can't mutate while borrowed
println!("{:?}", slice);
```

Why? `slice` is an immutable borrow (`&`). While it's alive, you can't take a mutable borrow via `.push()`. This is Rule 6 from Concept 5: many `&` OR one `&mut`, never both.

Mutable slices also exist:

```rust
fn double_all(numbers: &mut [i32]) {
    for n in numbers.iter_mut() {
        *n *= 2;
    }
}

let mut data = vec![1, 2, 3];
double_all(&mut data);
// data is now [2, 4, 6]
```

---

## Why Slices Matter

1. **Zero-cost** — no copying, no allocation. Just a pointer + length.
2. **Flexibility** — one function handles arrays, vectors, and subranges.
3. **Safety** — Rust checks bounds at runtime. `&a[10..20]` on a 5-element array panics instead of reading garbage memory like C would.
4. **Everywhere in Solana** — account data is `&[u8]`. Transaction data is `&[u8]`. You'll slice bytes constantly.

---

## Common Slice Methods

```rust
let s = &[10, 20, 30, 40, 50];

s.len()           // 5
s.is_empty()      // false
s[0]              // 10 (index)
s.first()         // Some(&10)
s.last()          // Some(&50)
s.contains(&30)   // true
s.iter()          // iterator over &i32
s.windows(2)      // overlapping pairs: [10,20], [20,30], [30,40], [40,50]
s.chunks(2)       // non-overlapping: [10,20], [30,40], [50]
s.split_at(2)     // (&[10,20], &[30,40,50])
```

---

## The Full Picture: Owned vs Borrowed

| Owned | Borrowed (immutable) | Borrowed (mutable) |
|---|---|---|
| `String` | `&str` | `&mut str` (rare) |
| `Vec<T>` | `&[T]` | `&mut [T]` |
| `[T; N]` (array) | `&[T]` | `&mut [T]` |

The pattern is always the same:
- Owned type stores data
- Slice type borrows a view into that data
- `&str` is just `&[u8]` with a UTF-8 guarantee

---

## Mini Quiz

**Q1:** What does this print?
```rust
let a = [10, 20, 30, 40, 50];
let s = &a[1..4];
println!("{:?}", s);
```
(a) `[10, 20, 30, 40]`  (b) `[20, 30, 40]`  (c) `[20, 30, 40, 50]`

**Q2:** Why should a function that reads a list take `&[i32]` instead of `&Vec<i32>`?

**Q3:** Does this compile?
```rust
let mut v = vec![1, 2, 3];
let slice = &v[..];
v.push(4);
println!("{:?}", slice);
```
(a) Yes  (b) No — and why?

---

## Answers

**Q1: (b) `[20, 30, 40]`**
`1..4` means index 1, 2, 3 — stop **before** index 4. Not including 4.

```
Index:   0    1    2    3    4
Value:  10   20   30   40   50
              ^         ^
            start    stop (not included)
```

**Q2:** `&[i32]` is more flexible — it accepts `&Vec`, arrays of any size, and subranges. `&Vec<i32>` only accepts a Vec. Same logic as `&str` vs `&String`.

**Q3: (b) No.**
`slice` is an immutable borrow of `v`. While `slice` is alive (used in `println!`), you can't mutate `v` with `.push()`. Rule 6 from Concept 5: you can't have `&` and `&mut` alive at the same time.

---

## When Should You Use `&Vec<T>`?

Honestly, almost never. There is no situation where `&Vec<T>` is better than `&[T]` for a function parameter.

```rust
// &Vec<T> gives you nothing extra over &[T]
fn read_data(data: &Vec<i32>) { ... }

// &[T] does everything &Vec<T> does, plus accepts more inputs
fn read_data(data: &[i32]) { ... }
```

When you pass `&Vec<i32>` to a function that takes `&[i32]`, Rust automatically converts it. So `&[i32]` already handles Vecs — you get no extra capability from writing `&Vec<i32>`.

### The only time `&Vec<T>` appears in real code

When you need to inspect Vec-specific metadata that a slice doesn't have:

| Info | `&[T]` | `&Vec<T>` |
|---|---|---|
| Length | Yes — `.len()` | Yes |
| Elements | Yes | Yes |
| Iteration | Yes | Yes |
| **Capacity** | No | Yes — `.capacity()` |

```rust
fn check_capacity(data: &Vec<i32>) {
    println!("len: {}, cap: {}", data.len(), data.capacity());
}
```

Capacity is how much heap space the Vec has reserved before it needs to reallocate. But in practice, you almost never need to know this in a function that's just reading data.

### Rule

| Situation | Use |
|---|---|
| Reading data | `&[T]` |
| Need `.capacity()` | `&Vec<T>` (rare, usually only performance-sensitive code) |
| Need to push/pop | `&mut Vec<T>` |

In 99% of cases, `&[T]` is the right choice.

---

## Next

→ [Concept 7: Structs and Fields](07-structs-and-fields.md)
