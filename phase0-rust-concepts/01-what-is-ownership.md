# Concept 1: What is Ownership?

Ownership is THE core concept of Rust. It is how Rust manages memory without a garbage collector (like Java/Go) and without manual malloc/free (like C/C++). Every other Rust concept builds on this.

---

## The 3 Rules of Ownership

Memorize these. Everything else follows from them:

**Rule 1:** Every value in Rust has exactly **one owner** (a variable).

```rust
let s = String::from("hello");
// s is the owner of the string "hello"
// there is no other owner
```

**Rule 2:** There can only be **one owner at a time**.

```rust
let s1 = String::from("hello");
let s2 = s1;
// s2 is now the owner
// s1 is INVALID — you cannot use it anymore
```

**Rule 3:** When the owner goes out of scope, the value is **dropped** (memory is freed).

```rust
{
    let s = String::from("hello");
    // s is valid here, you can use it
}
// s is OUT OF SCOPE — Rust automatically frees the memory
// no garbage collector needed, no manual free() needed
```

---

## Why Does This Matter?

In other languages:

```python
# Python — both point to the same string, garbage collector cleans up later
s1 = "hello"
s2 = s1
print(s1)  # works fine
```

```c
// C — you manage memory yourself, easy to mess up
char *s1 = malloc(6);
char *s2 = s1;
free(s1);
printf("%s", s2);  // BUG! use-after-free — crash or security hole
```

Rust eliminates BOTH problems:
- No garbage collector (fast, predictable performance)
- No use-after-free bugs (compiler catches it)

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // COMPILER ERROR: "value used after move"
```

The compiler literally **refuses to compile** if you violate ownership rules. You can't ship the bug.

---

## What is a "Move"?

When you write `let s2 = s1;`, ownership **moves** from `s1` to `s2`. After the move:
- `s2` owns the data
- `s1` is dead — the compiler won't let you touch it

Think of it like handing someone a physical book:
- You had the book (`s1`)
- You gave it to them (`s2 = s1`)
- You no longer have the book — you can't read it anymore

---

## Exception: Copy Types

Simple types like numbers, booleans, and chars are **copied**, not moved:

```rust
let x = 5;
let y = x;
println!("{}", x);  // WORKS FINE — integers are copied, not moved
```

Why? Numbers are small and cheap to copy. There is no heap memory to worry about. Rust copies them automatically.

**Types that Copy:** `i32`, `u32`, `f64`, `bool`, `char`, tuples of Copy types

**Types that Move:** `String`, `Vec<T>`, any heap-allocated data

---

## Real-World Analogy

| Concept | Analogy |
|---------|---------|
| Ownership | You own a car. Only one person on the title. |
| Move | You sell the car. New owner on the title. You can't drive it. |
| Drop | Car gets scrapped when the owner is done with it. |
| Copy | You photocopy a document. Both people have their own copy. |

---

## Mini Quiz

**Q1:** What happens here?
```rust
let a = String::from("rust");
let b = a;
println!("{}", a);
```
Does it: (a) print "rust", (b) compiler error, (c) runtime crash?

**Q2:** What about this?
```rust
let x = 42;
let y = x;
println!("{}", x);
```
Does it: (a) print 42, (b) compiler error, (c) runtime crash?

**Q3:** When does Rust free the memory for a `String`?

---

## Answers

**Q1: (b) Compiler error.**
`let b = a;` moves ownership from `a` to `b`. After that, `a` is dead. The compiler gives:
```
error[E0382]: borrow of moved value: `a`
```

**Q2: (a) Print 42.**
`i32` is a Copy type. `let y = x;` makes a copy — both `x` and `y` have their own independent `42`.

**Q3: When the owner goes out of scope — not when it is moved.**
- **Move** = change of ownership (car still exists, new owner)
- **Drop** = owner goes out of scope, memory freed (car gets scrapped)

---

## Next

→ [Concept 2: Move Semantics](02-move-semantics.md)
