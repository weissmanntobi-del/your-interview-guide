# Concept 7: Structs and Fields

A struct is a **custom data type** that groups related values together under one name. Think of it as creating your own type — like how `i32` holds a number, a struct holds multiple named pieces of data.

---

## Why Structs?

Without structs, you'd pass around loose values:

```rust
fn print_user(name: &str, age: u32, active: bool) {
    println!("{}, age {}, active: {}", name, age, active);
}
```

With 3 values this is fine. With 10 values? A mess. Structs group them:

```rust
fn print_user(user: &User) {
    println!("{}, age {}, active: {}", user.name, user.age, user.active);
}
```

One parameter instead of many. The data travels together.

---

## Defining a Struct

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}
```

- `struct` — keyword to define a new type
- `User` — the name of your type (convention: PascalCase)
- `name: String` — a **field** with a name and a type
- Each field has an explicit type (just like function parameters)

---

## Creating an Instance

```rust
let alice = User {
    name: String::from("Alice"),
    age: 30,
    active: true,
};
```

You must provide **every field**. Rust won't let you skip one — no default values unless you explicitly implement them.

---

## Accessing Fields

Use dot notation:

```rust
println!("{}", alice.name);    // "Alice"
println!("{}", alice.age);     // 30
println!("{}", alice.active);  // true
```

---

## Mutating Fields

The **entire variable** must be `let mut` — you can't make just one field mutable:

```rust
let mut bob = User {
    name: String::from("Bob"),
    age: 25,
    active: true,
};

bob.age = 26;          // works — bob is mut
bob.active = false;    // works
```

Without `mut`:

```rust
let charlie = User { name: String::from("Charlie"), age: 28, active: true };
charlie.age = 30;   // COMPILER ERROR — charlie is not mutable
```

Rust doesn't have per-field mutability. The whole variable is either mutable or it isn't — all or nothing.

---

## Ownership in Structs

Structs **own** their fields. When the struct is dropped, all its fields are dropped too:

```rust
let user = User {
    name: String::from("Alice"),  // user owns this String
    age: 30,
    active: true,
};
// when user goes out of scope, the String "Alice" is freed too
```

This means you can't put a `&str` in a struct without lifetimes (Concept 14). Use `String` for string fields for now:

```rust
// Won't compile without lifetimes
struct User {
    name: &str,    // ERROR — whose string is this borrowing? for how long?
}

// Works — struct owns its data
struct User {
    name: String,  // owned string, no borrow questions
}
```

---

## Struct Update Syntax

Create a new struct from an existing one, changing only some fields:

```rust
let alice = User {
    name: String::from("Alice"),
    age: 30,
    active: true,
};

let bob = User {
    name: String::from("Bob"),
    ..alice   // take remaining fields (age, active) from alice
};
// bob.age = 30, bob.active = true
```

**Warning:** `..alice` **moves** any non-Copy fields. If you hadn't overridden `name`, Alice's `name` String would be moved into Bob and `alice` would be partially dead. Fields with Copy types (`u32`, `bool`) are fine — they get copied.

---

## Tuple Structs

Sometimes you want a named type but don't need named fields:

```rust
struct Point(f64, f64);       // x, y
struct Color(u8, u8, u8);    // r, g, b

let origin = Point(3.0, 7.0);
let red = Color(255, 0, 0);

println!("x: {}", origin.0);  // 3.0 — access by index (zero-based)
println!("y: {}", origin.1);  // 7.0
println!("r: {}", red.0);     // 255
```

Why use tuple structs? **Type safety** without field names:

```rust
let p = Point(1.0, 2.0);
let c = Color(1, 2, 3);
// p and c are different types even though both are "just numbers"
// you can't accidentally pass a Color where a Point is expected
```

---

## Unit Structs

A struct with no fields at all:

```rust
struct Marker;

let m = Marker;
```

Rare for now, but useful in Concept 12 (Traits) when implementing a trait on a type that holds no data.

---

## Common Patterns in This Repo

**Solana account data:**
```rust
struct Account {
    pubkey: [u8; 32],
    lamports: u64,
    data: Vec<u8>,
    owner: [u8; 32],
    executable: bool,
}
```

**A cache entry:**
```rust
struct CacheEntry<V> {
    value: V,
    created_at: u64,
    access_count: u32,
}
```

**A parse result:**
```rust
struct ParseResult {
    value: u64,
    bytes_consumed: usize,
    valid: bool,
}
```

Structs are everywhere — almost every challenge in phases 1-17 defines at least one.

---

## Summary

| Concept | Syntax | What it means |
|---|---|---|
| Define a struct | `struct Foo { field: Type }` | Create a custom type with named fields |
| Create an instance | `Foo { field: value }` | Must provide all fields |
| Access a field | `foo.field` | Dot notation |
| Mutate a field | `foo.field = new` | Whole variable must be `let mut` |
| Update syntax | `Foo { field: new, ..old }` | Copy/move remaining fields from another instance |
| Tuple struct | `struct Foo(Type, Type)` | Named type, unnamed fields, access by index |
| Unit struct | `struct Foo;` | No fields, used for trait implementations |

---

## Mini Quiz

**Q1:** Does this compile?
```rust
struct Point {
    x: f64,
    y: f64,
}

let p = Point { x: 1.0, y: 2.0 };
p.x = 3.0;
```
(a) Yes  (b) No — why?

**Q2:** Why does this fail?
```rust
struct User {
    name: &str,
    age: u32,
}
```

**Q3:** What is `origin.1` here?
```rust
struct Point(f64, f64);
let origin = Point(3.0, 7.0);
```
(a) 3.0  (b) 7.0  (c) compiler error

---

## Answers

**Q1: (b) No.**
`p` is not declared as `mut`. You cannot mutate any field without `let mut` on the whole variable:
```rust
let mut p = Point { x: 1.0, y: 2.0 };  // fix: add mut
p.x = 3.0;   // now works
```
Rust has no per-field mutability — the entire variable is mutable or it isn't.

**Q2:** `&str` is a borrowed reference. The struct doesn't own it. Rust asks: "how long does this borrow live?" Without a lifetime annotation, the compiler can't guarantee the referenced string will outlive the struct. Fix: use `String` (owned) for string fields until you learn lifetimes (Concept 14).

**Q3: (b) 7.0.**
Tuple struct fields are accessed by zero-based index:
- `origin.0` = `3.0` (first field)
- `origin.1` = `7.0` (second field)

---

## Next

→ [Concept 8: Methods with `impl`](08-methods-with-impl.md)
