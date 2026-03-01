# Concept 3: Borrowing with `&` (Immutable References)

Borrowing is Rust's solution to the problem from Concept 2: **"I want to use a value without taking ownership."**

---

## The Problem (Recap)

```rust
fn print_length(s: String) {
    println!("length: {}", s.len());
}   // s is dropped here, memory freed!

let name = String::from("alice");
print_length(name);          // name moved into function
// println!("{}", name);     // DEAD — can't use name anymore
```

You just wanted to check the length. You didn't want to consume the string. But the function took ownership and destroyed it.

---

## The Solution: Borrow with `&`

Instead of giving ownership, you **lend** the value using `&`:

```rust
fn print_length(s: &String) {   // s is a REFERENCE — it borrows, doesn't own
    println!("length: {}", s.len());
}   // s (the reference) goes out of scope, but the original data is NOT freed

let name = String::from("alice");
print_length(&name);         // lend name to the function
println!("{}", name);        // WORKS — name is still yours
```

The `&` symbol means "I am borrowing this, not taking it."

---

## The Book Analogy

| Without borrowing | With borrowing |
|-------------------|----------------|
| You give your friend the book | You let your friend look at your book |
| Friend owns it now | You still own it |
| You can't read it anymore | Friend gives it back when done |
| Book gets destroyed when friend is done | Book is still yours after |

---

## How It Works Step by Step

```rust
let name = String::from("alice");   // name owns the String
print_length(&name);                 // create a reference to name, pass it in
//           ^ this & creates a borrow
```

Inside the function:

```rust
fn print_length(s: &String) {
//              ^  ^ this means "s is a borrowed reference to a String"
//              s does NOT own the String
    println!("length: {}", s.len());  // can READ through the reference
}
// s (the reference) is gone, but the actual String is untouched
```

After the function call, `name` is still perfectly valid — it was never moved. It was only **borrowed**.

---

## You Can Borrow Multiple Times

Since borrowing is just "looking," multiple borrows are fine:

```rust
fn print_it(s: &String) {
    println!("{}", s);
}

let name = String::from("alice");
print_it(&name);      // borrow 1
print_it(&name);      // borrow 2
print_it(&name);      // borrow 3
println!("{}", name); // still valid — never moved
```

---

## Borrowing is Read-Only

An immutable reference (`&`) means you can **read** but **not modify**:

```rust
fn try_to_change(s: &String) {
    s.push_str(" world");   // COMPILER ERROR — can't modify through &
}
```

Error:
```
error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
```

If you lend someone your book, they can read it but they should not write in it. Mutable borrowing `&mut` (Concept 4) handles that case.

---

## `&String` vs `&str`

You will often see functions written with `&str` instead of `&String`:

```rust
fn print_length(s: &str) {    // &str instead of &String
    println!("length: {}", s.len());
}
```

**`&str` is more flexible than `&String`:**

| Parameter type | Accepts |
|----------------|---------|
| `&String` | Only `&String` |
| `&str` | `&String`, `&str`, and string literals |

```rust
let owned = String::from("hello");
let literal = "world";

// With &str parameter — both work
print_length(&owned);    // Rust auto-converts &String to &str
print_length(literal);   // already &str
```

**Rule of thumb:** If a function only needs to read a string, use `&str` as the parameter type.

---

## `&str` in Depth — What it Actually Is

### `String` is the whole book. `&str` is a bookmark pointing at some pages.

```rust
let s = String::from("hello world");
```

`s` owns the full string "hello world" on the heap. Think of it as the whole book.

```rust
let hello: &str = &s[0..5];    // points at "hello"
let world: &str = &s[6..11];   // points at "world"
let all:   &str = &s[..];      // points at "hello world"
```

`hello` doesn't own anything. It's just saying: "look at bytes 0 through 4 of that string over there."

### How the range works

Think of the string as boxes, each with an index:

```
Index:   0   1   2   3   4   5   6   7   8   9   10
Letter:  h   e   l   l   o   ' ' w   o   r   l   d
```

`&s[0..5]` means: start at index 0, stop **before** index 5:

```
         h   e   l   l   o
         ^               ^
       start         stop (not included)
```

Result: `"hello"`

`&s[6..11]` means: start at index 6, stop before 11:

```
                         w   o   r   l   d
                         ^               ^
                       start           stop
```

Result: `"world"`

### Range shortcuts

```rust
let s = String::from("hello world");

&s[0..5]   // "hello"        — from 0 to 5
&s[..5]    // "hello"        — same (0 is default start)
&s[6..11]  // "world"        — from 6 to 11
&s[6..]    // "world"        — same (end is default stop)
&s[..]     // "hello world"  — the whole string
```

### Three places `&str` can come from

```rust
// 1. String literal — baked into your compiled program
let a: &str = "hello";

// 2. Borrowing a String — looking at its heap data
let owned = String::from("hello");
let b: &str = &owned;

// 3. Slicing part of a String — a substring
let c: &str = &owned[0..3];  // "hel"
```

All three are `&str`. All three are just "a pointer + a length" into some string data.

### The one gotcha: byte indices, not character indices

String slicing is **byte-based**, not character-based. For ASCII strings (English text) this is fine — every character is exactly 1 byte:

```rust
let s = String::from("hello");
let slice = &s[0..2];  // "he" — works fine
```

But multi-byte characters (emoji, accented letters) can cause a panic if you slice in the middle of one. For this repo and Solana work you mostly deal with ASCII and raw bytes (`&[u8]`), so this rarely bites you. Just good to know.

### One-line summary

`&str` = "I'm looking at some string characters that someone else owns. I know where they start and how many there are."

---

## Summary So Far

| Parameter | Ownership? | Can modify? | Use when |
|-----------|-----------|-------------|---------|
| `s: String` | Takes ownership | Yes (caller loses it) | Function needs to own the data |
| `s: &String` or `s: &str` | Borrows | No | Function just reads |

---

## Mini Quiz

**Q1:** What happens here?
```rust
fn shout(s: &String) {
    println!("{}!!!", s);
}

let msg = String::from("hello");
shout(&msg);
shout(&msg);
println!("{}", msg);
```
Does it: (a) compiler error, (b) prints "hello!!!" twice then "hello", (c) runtime crash?

**Q2:** Why does this fail?
```rust
fn add_exclamation(s: &String) {
    s.push_str("!");
}
```

**Q3:** If a function only needs to read a string, should the parameter be `String`, `&String`, or `&str`? Why?

---

## Answers

**Q1: (b) Prints "hello!!!" twice then "hello".**
`&msg` borrows without moving. You can borrow as many times as you want with `&`. All three print statements work fine.

**Q2:** `&String` is an **immutable reference** — read-only access. The `&` without `mut` means you cannot modify through it. `push_str` tries to modify the string, which is not allowed. You would need `&mut String` to modify.

**Q3: `&str`** because:
- You are only reading, so you do not need ownership (`String`)
- `&str` is more flexible than `&String` — it accepts `String`, `&String`, `&str`, and string literals
- This is standard Rust practice: if you are just reading a string, take `&str`

---

## Next

→ [Concept 4: Mutable References `&mut`](04-mutable-references.md)
