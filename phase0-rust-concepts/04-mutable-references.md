# Concept 4: Mutable References (`&mut`)

In Concept 3, `&` lets you **read** a borrowed value. But what if you want to **modify** something without taking ownership? That is what `&mut` does.

---

## The Problem

```rust
fn add_greeting(s: &String) {
    s.push_str(", world!");    // COMPILER ERROR — & is read-only
}
```

`&` means "I am borrowing this to look at it." You cannot modify it. But sometimes a function needs to change the original value.

---

## The Solution: `&mut`

```rust
fn add_greeting(s: &mut String) {
//                 ^^^^ mutable reference — can read AND write
    s.push_str(", world!");    // WORKS
}

let mut name = String::from("hello");
//  ^^^ the variable itself must also be declared mut
add_greeting(&mut name);
//           ^^^^ pass a mutable reference
println!("{}", name);  // prints "hello, world!"
```

**Three things must line up:**
1. The variable is declared `let mut`
2. You pass `&mut` when calling
3. The function parameter is `&mut`

All three or it will not compile.

---

## Comparison: `&` vs `&mut`

```rust
// Immutable borrow — can read, cannot modify
fn read_it(s: &String) {
    println!("{}", s);        // OK — reading
    // s.push_str("!");       // ERROR — can't modify
}

// Mutable borrow — can read AND modify
fn change_it(s: &mut String) {
    println!("{}", s);        // OK — reading
    s.push_str("!");          // OK — modifying
}

let mut msg = String::from("hello");
read_it(&msg);         // immutable borrow
change_it(&mut msg);   // mutable borrow
println!("{}", msg);   // prints "hello!"
```

---

## THE BIG RULE: One `&mut` OR Many `&`, Never Both

At any given time, you can have EITHER:
- **One** mutable reference (`&mut`)
- **OR** any number of immutable references (`&`)
- **NEVER both at the same time**

```rust
let mut s = String::from("hello");

// FINE — multiple immutable borrows
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);  // OK

// FINE — one mutable borrow (after r1 and r2 are done)
let m1 = &mut s;
m1.push_str("!");           // OK

// FAILS — can't mix & and &mut
let r1 = &s;
let m1 = &mut s;
println!("{}", r1);          // COMPILER ERROR
```

---

## Why This Rule Exists

Imagine two pieces of code running:
- Reader A is looking at the string: "hello"
- Writer B changes the string to "goodbye"
- Reader A is now looking at unexpected data

This is called a **data race**. In C/C++ this causes crashes, security bugs, and corrupted data. Rust prevents it entirely at compile time:

- If someone is reading (`&`), nobody can write
- If someone is writing (`&mut`), nobody else can read or write

---

## The Whiteboard Analogy

| Scenario | Allowed? |
|----------|---------|
| 5 people reading a whiteboard | Yes — multiple `&` |
| 1 person writing, nobody else in the room | Yes — one `&mut` |
| 1 person writing while others read | **No** — someone might read half-changed data |
| 2 people writing at the same time | **No** — they would overwrite each other |

---

## Works with Any Type, Not Just String

```rust
fn double_it(n: &mut i32) {
    *n = *n * 2;    // * dereferences — "go to the value this reference points at"
}

let mut x = 5;
double_it(&mut x);
println!("{}", x);   // prints 10
```

The `*n` is **dereferencing** — reaching through the reference to the actual value. For `String` methods like `.push_str()`, Rust does this automatically. For raw assignment on primitives, you need `*`.

---

## All Three Ways to Pass Data to a Function

| Parameter | Ownership? | Can modify? | Use when |
|-----------|-----------|-------------|---------|
| `s: String` | Takes ownership | Yes (caller loses it) | Function needs to own the data |
| `s: &String` or `s: &str` | Borrows | No | Function just reads |
| `s: &mut String` | Borrows | Yes | Function needs to modify caller's data |

---

## Mini Quiz

**Q1:** What is wrong here?
```rust
let name = String::from("alice");
add_suffix(&mut name);
```

**Q2:** Does this compile?
```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);
```
(a) Yes, (b) No — can't have two references

**Q3:** Does this compile?
```rust
let mut s = String::from("hi");
let r1 = &s;
let m1 = &mut s;
println!("{}", r1);
```
(a) Yes, (b) No — and why?

---

## Answers

**Q1:** `name` is missing `mut`. You cannot take `&mut` of a non-mutable variable.
```rust
let mut name = String::from("alice");  // fix: add mut
add_suffix(&mut name);                 // now works
```

**Q2: (a) Yes.** Multiple immutable references (`&`) are fine. Many readers at the same time — no problem.

**Q3: (b) No.** You cannot have `&` and `&mut` alive at the same time. `r1` is an immutable borrow that is still alive when `m1` tries to take a mutable borrow. Rust prevents mixing readers and writers.

---

## Next

→ [Concept 5: The Borrow Checker Rules](05-borrow-checker-rules.md)
