# Concept 9: Enums and Variants

If structs are "a type with multiple fields," enums are "a type that can be **one of several variants**." A struct is always ALL its fields. An enum is exactly ONE of its variants at a time.

---

## Why Enums?

Imagine modeling a traffic light:

```rust
// Without enums — using strings (fragile, error-prone)
let light: &str = "green";
let light: &str = "grean";   // typo — no compiler help

// With enums — the compiler knows all valid values
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

let light = TrafficLight::Green;
// TrafficLight::Grean  → COMPILER ERROR — no such variant
```

Enums give you **exhaustive, compiler-checked** options. No typos, no invalid states.

---

## Defining an Enum

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let d = Direction::Up;
```

- `enum` — keyword
- `Direction` — name (PascalCase)
- `Up`, `Down`, etc. — **variants** (also PascalCase)
- Access with `::` — `Direction::Up`

---

## Enums with Data

This is where Rust enums get powerful. Each variant can hold **different data**:

```rust
enum Message {
    Quit,                        // no data
    Echo(String),                // holds a String
    Move { x: i32, y: i32 },    // holds named fields (like a struct)
    Color(u8, u8, u8),           // holds multiple values (like a tuple struct)
}

let m1 = Message::Quit;
let m2 = Message::Echo(String::from("hello"));
let m3 = Message::Move { x: 10, y: 20 };
let m4 = Message::Color(255, 0, 0);
```

Each variant is a different "shape" of data, but they're all the same type: `Message`. You can put them in the same Vec, pass them to the same function, etc.

---

## Why This Matters: Comparing to Other Languages

In most languages, you'd use a class hierarchy or union types:

```python
# Python — no compiler help, anything can go wrong
def handle(msg):
    if msg["type"] == "quit":
        ...
    elif msg["type"] == "echo":
        print(msg["text"])  # hope "text" exists
```

In Rust, the compiler **guarantees** you handle every variant and every variant has exactly the data it's supposed to.

---

## Enums Have Methods Too

Just like structs, you can add methods with `impl`:

```rust
impl TrafficLight {
    fn is_stop(&self) -> bool {
        match self {
            TrafficLight::Red => true,
            TrafficLight::Yellow => true,
            TrafficLight::Green => false,
        }
    }
}

let light = TrafficLight::Red;
println!("{}", light.is_stop());  // true
```

(We cover `match` properly in Concept 10.)

---

## The Two Most Important Enums in Rust

You already used both of these in earlier concepts. Now you see the mechanism underneath:

### `Option<T>` — "maybe a value, maybe not"

```rust
// Defined in the standard library:
enum Option<T> {
    Some(T),    // there is a value
    None,       // there is no value
}

let x: Option<i32> = Some(42);
let y: Option<i32> = None;
```

`Option` is just an enum with two variants. `Some(42)` is type `Option<i32>` — not `Some<i32>`. `Some` is a **variant**, not a type.

### `Result<T, E>` — "success or failure"

```rust
// Defined in the standard library:
enum Result<T, E> {
    Ok(T),     // success with value T
    Err(E),    // failure with error E
}

let ok: Result<i32, String> = Ok(42);
let err: Result<i32, String> = Err(String::from("something broke"));
```

`Result` has exactly **two variants**: `Ok` and `Err`. Every time you used `.ok()` to convert to `Option`, you were working with this enum.

---

## Enum Variants Follow Ownership Rules

Same ownership rules as structs and any other type:

```rust
enum Animal {
    Dog(String),
    Cat(String),
}

let a = Animal::Dog(String::from("Rex"));
let b = a;               // a is MOVED to b
println!("{:?}", a);     // COMPILER ERROR — a is dead
```

`Animal::Dog(String)` contains a `String` (heap type), so it moves. Same rules from Concept 2 — enums are no different.

---

## Common Patterns in This Repo

**Solana instruction enum (Phase 6, 12):**
```rust
enum Instruction {
    Transfer { from: [u8; 32], to: [u8; 32], amount: u64 },
    CreateAccount { pubkey: [u8; 32], lamports: u64 },
    CloseAccount { pubkey: [u8; 32] },
}
```

**VM instruction set (Phase 13):**
```rust
enum EbpfInstruction {
    Add(u8, u8),     // add register A to register B
    Load(u8, u64),   // load value into register
    Jump(i32),       // jump by offset
    Exit,            // halt
}
```

**Error types (everywhere):**
```rust
enum ParseError {
    InvalidFormat,
    OutOfRange(u64),
    UnexpectedByte(u8),
}
```

---

## Summary

| Concept | Syntax | What it means |
|---|---|---|
| Simple enum | `enum Foo { A, B, C }` | A type with named variants, no data |
| Enum with data | `enum Foo { A(i32), B(String) }` | Each variant holds different data |
| Struct-like variant | `enum Foo { A { x: i32, y: i32 } }` | Variant with named fields |
| Create a variant | `Foo::A(42)` | Use `::` to pick a variant |
| Methods on enums | `impl Foo { fn bar(&self) ... }` | Same as struct methods |
| `Option<T>` | `Some(T)` or `None` | Standard library enum — "maybe a value" |
| `Result<T, E>` | `Ok(T)` or `Err(E)` | Standard library enum — "success or failure" |

---

## Mini Quiz

**Q1:** How many variants does this enum have?
```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}
```
(a) 2  (b) 3  (c) 5

**Q2:** What type is `x`?
```rust
let x = Some(42);
```
(a) `i32`  (b) `Option<i32>`  (c) `Some<i32>`

**Q3:** Can two variants of the same enum hold different types of data?
(a) Yes  (b) No

**Q4:** What's wrong here?
```rust
enum Animal {
    Dog(String),
    Cat(String),
}

let a = Animal::Dog(String::from("Rex"));
let b = a;
println!("{:?}", a);
```
(a) Works fine  (b) Compiler error — why?

**Q5:** `Result<T, E>` is an enum. How many variants does it have, and what are they?

---

## Answers

**Q1: (b) 3.**
`Circle`, `Rectangle`, and `Triangle` are 3 variants. Each holds different data but they're all the same type: `Shape`.

**Q2: (b) `Option<i32>`.**
`Some` is a variant of `Option`, not a type itself. `Some(42)` is type `Option<i32>`. Think of `Some` and `None` as doors into the `Option` building — the building's name is `Option<i32>`.

**Q3: (a) Yes.**
That's the whole point. `Circle(f64)`, `Rectangle(f64, f64)`, `Triangle { base: f64, height: f64 }` all hold different shapes of data, but they're all `Shape`.

**Q4: (b) Compiler error.**
`a` is moved to `b` on `let b = a;`. After the move, `a` is dead. `println!` tries to use `a` — compiler error:
```
error[E0382]: borrow of moved value: `a`
```
`Animal::Dog(String)` contains a `String` (heap type), so it moves. Same ownership rules from Concept 2.

**Q5: Two variants — `Ok(T)` and `Err(E)`.**
```rust
enum Result<T, E> {
    Ok(T),    // success
    Err(E),   // failure
}
```

---

## Next

→ [Concept 10: `match` and Pattern Matching](10-match-and-pattern-matching.md)
