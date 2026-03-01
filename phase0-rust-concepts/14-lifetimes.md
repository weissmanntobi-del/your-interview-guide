# Concept 14: Lifetimes `'a`

## Plain English

A lifetime is the **scope for which a reference is valid**. Rust tracks this automatically most of the time, but sometimes you need to tell it explicitly how long a reference lives.

Every reference has a lifetime — a range of code where it's valid. Rust's borrow checker uses lifetimes to make sure you never hold a reference to something that no longer exists (a dangling reference).

Most of the time Rust figures out lifetimes on its own (this is called **lifetime elision**). You only need to write them explicitly when the compiler can't figure it out — usually when a function takes multiple references and returns one.

---

## The Problem Lifetimes Solve

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

This **doesn't compile**. The compiler sees two input references and one output reference, but doesn't know which input the output is tied to. It can't guarantee the returned reference is valid.

**Fix — add a lifetime annotation:**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` says: "the returned reference will be valid for as long as **both** `x` and `y` are valid" — i.e. the shorter of the two.

---

## Lifetime in Structs

If a struct holds a reference, you must annotate it:

```rust
struct Important<'a> {
    text: &'a str,  // text must live at least as long as the struct
}
```

This prevents the struct from outliving the data it references.

---

## Lifetime Elision (when you don't need to write them)

Rust has rules that let it infer lifetimes in simple cases:

```rust
fn first_word(s: &str) -> &str { ... }  // no annotation needed
```

You only write `'a` when the compiler says it can't figure it out.

---

## Key Vocabulary

| Term | Meaning |
|---|---|
| `'a` | A named lifetime parameter |
| Lifetime elision | Rust inferring lifetimes automatically |
| Dangling reference | A reference to data that no longer exists |
| `'static` | Valid for the entire program (e.g. string literals) |

---

## Mini Quiz

**Q1.** Why did `fn longest(x: &str, y: &str) -> &str` fail to compile?
- A) `&str` is not a valid return type
- B) The compiler can't tell which input reference the output is tied to ✓

**Q2.** What does `'a` in `fn foo<'a>(x: &'a str) -> &'a str` mean?
- A) The output reference lives exactly as long as `x` ✓
- B) The output reference lives forever

**Q3.** What is `'static`?
- A) A lifetime that lasts the entire duration of the program ✓
- B) A lifetime that only lasts inside a single function

---

## Quiz Answers

**Q1 → B.** `&str` is a perfectly valid return type. The problem is the compiler can't determine which input the output ties back to — it needs a lifetime annotation to verify safety.

**Q2 → A.** `'a` on both input and output means the returned reference is valid for as long as `x` is valid — they share the same lifetime.

**Q3 → A.** `'static` lasts the entire program. String literals like `"hello"` have `'static` lifetime because they're baked into the binary.

---

## Next

→ [15-closures.md](15-closures.md)
