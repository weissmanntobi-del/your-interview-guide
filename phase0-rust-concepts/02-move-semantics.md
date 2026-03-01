# Concept 2: Move Semantics (Deeper Dive)

Concept 1 showed that `let b = a;` moves ownership. But moves happen in **more places than just assignment**. Understanding where moves happen is key to not fighting the borrow checker.

---

## Moves Happen in 3 Places

### 1. Assignment (you already know this)

```rust
let s1 = String::from("hello");
let s2 = s1;       // move
// s1 is dead
```

### 2. Passing to a Function

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}   // s goes out of scope here, memory freed

let name = String::from("alice");
takes_ownership(name);      // name is MOVED into the function
// println!("{}", name);    // COMPILER ERROR — name is dead
```

When you pass a `String` to a function, you are **giving it away**. The function now owns it. After the call, your variable is gone.

Think of it like mailing a physical package:
- You had the package (`name`)
- You mailed it to the function (`takes_ownership(name)`)
- You don't have the package anymore

### 3. Returning from a Function

```rust
fn create_greeting() -> String {
    let s = String::from("hello world");
    s   // ownership moves OUT to the caller
}

let greeting = create_greeting();
// greeting now owns the String
```

Returning a value **transfers ownership to the caller**. This is how you get data out of functions without copying.

---

## The Problem: What If You Don't WANT to Give Ownership Away?

This gets annoying fast:

```rust
fn print_length(s: String) -> String {
    println!("length: {}", s.len());
    s   // have to return it just to give ownership back!
}

let name = String::from("alice");
let name = print_length(name);  // move in, get it back
// name is usable again, but this is ugly
```

You had to **return the value just to keep using it**. This is the problem that **borrowing** (Concept 3) solves.

---

## Moves with Copy Types: No Move Happens

Simple types are **copied**, not moved:

```rust
fn print_number(x: i32) {
    println!("{}", x);
}

let n = 42;
print_number(n);     // n is COPIED into the function
println!("{}", n);   // WORKS — n is still valid
```

No move. `i32` is small and cheap — Rust just copies it.

---

## How to Think About It

Every time you use a non-Copy variable, ask: **"am I giving it away?"**

| Action | Move? | After the action |
|--------|-------|-----------------|
| `let b = a;` | Yes | `a` is dead |
| `some_function(a)` | Yes | `a` is dead |
| `let b = some_function();` | Return moves to `b` | `b` is alive |
| `let y = x;` (where x is `i32`) | No (copy) | Both alive |
| `some_function(x)` (where x is `i32`) | No (copy) | `x` still alive |

---

## A Complete Example Showing All 3 Moves

```rust
fn make_greeting(name: String) -> String {
    // name was moved IN from the caller
    let greeting = format!("Hello, {}!", name);
    // name is consumed by format!, greeting is a new String
    greeting  // moved OUT to the caller
}

fn main() {
    let my_name = String::from("Alice");     // my_name owns it
    let result = make_greeting(my_name);      // my_name moved into function
    // my_name is DEAD here
    println!("{}", result);                   // result owns the greeting
}   // result goes out of scope, memory freed
```

---

## Mini Quiz

**Q1:** What happens here?
```rust
fn eat_string(s: String) {
    println!("{}", s);
}

let food = String::from("pizza");
eat_string(food);
eat_string(food);
```
Does it: (a) print "pizza" twice, (b) compiler error on the second call, (c) runtime crash?

**Q2:** What about this?
```rust
fn double(x: i32) -> i32 {
    x * 2
}

let n = 10;
let a = double(n);
let b = double(n);
```
Does it: (a) compiler error, (b) works fine — `a` is 20 and `b` is 20, (c) runtime crash?

---

## Answers

**Q1: (b) Compiler error on the second call.**

```rust
eat_string(food);    // food is MOVED into the function — food is now dead
eat_string(food);    // COMPILER ERROR — food was already moved
```

The error:
```
error[E0382]: use of moved value: `food`
  --> src/main.rs:6:16
   |
5  | eat_string(food);
   |            ---- value moved here
6  | eat_string(food);
   |            ^^^^ value used here after move
```

**Q2: (b) Works fine.**
`i32` is a Copy type. `n` gets copied into `double()` each time. Both calls work, `a` is 20, `b` is 20, `n` is still 10.

---

## Key Takeaway

**Passing a String to a function is a move, not a copy.** After the first call, you don't have it anymore. If `food` were an `i32` instead, it would work fine.

---

## Next

→ [Concept 3: Borrowing with `&`](03-borrowing.md)
