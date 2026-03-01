# Concept 10: `match` and Pattern Matching

In Concept 9 you learned enums can be one of several variants. But how do you check which variant it is and get the data out? That's what `match` does.

---

## What is `match`?

`match` checks a value against a list of **patterns** and runs the first one that matches:

```rust
let number = 3;

match number {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("something else"),  // _ is wildcard — catches anything
}
// prints "three"
```

Each line is an **arm**: `pattern => code`. Arms are checked top to bottom. `_` is the catch-all — like `else`.

---

## The Key Rule: `match` Must Be Exhaustive

Rust forces you to handle **every possible case**. Miss one and it won't compile:

```rust
enum Direction { Up, Down, Left, Right }

let d = Direction::Up;

match d {
    Direction::Up => println!("up"),
    Direction::Down => println!("down"),
    // COMPILER ERROR — Left and Right not handled
}
```

Fix: either add the missing arms or use `_`:

```rust
match d {
    Direction::Up => println!("up"),
    Direction::Down => println!("down"),
    _ => println!("left or right"),
}
```

This is the whole point — the compiler **guarantees** you never forget a case. No runtime surprises.

---

## Extracting Data from Enum Variants

When a variant holds data, `match` lets you pull it out:

```rust
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Quit => println!("quit"),
    Message::Echo(text) => println!("echo: {}", text),  // text = the String
    Message::Move { x, y } => println!("move to {}, {}", x, y),  // x=10, y=20
}
// prints "move to 10, 20"
```

This is **destructuring** — pulling data out of a variant as part of the match.

---

## `match` is an Expression

Just like `if/else`, `match` returns a value:

```rust
let label = match d {
    Direction::Up => "north",
    Direction::Down => "south",
    Direction::Left => "west",
    Direction::Right => "east",
};

println!("{}", label);  // "north"
```

No semicolons on the arm values — same implicit return rule as functions.

---

## Matching `Option<T>`

This is the most common use of `match` early on:

```rust
fn describe(value: Option<i32>) {
    match value {
        Some(n) => println!("got: {}", n),  // n = the i32 inside Some
        None => println!("no value"),
    }
}

describe(Some(42));  // "got: 42"
describe(None);      // "no value"
```

The compiler forces you to handle `None` — you can't accidentally ignore it.

---

## Multiple Patterns with `|`

Use `|` to match several patterns in one arm:

```rust
match number {
    1 | 2 => println!("one or two"),
    3 | 4 => println!("three or four"),
    _ => println!("other"),
}
```

---

## `if let` — When You Only Care About One Variant

When you only care about one variant and want to ignore the rest, `if let` is cleaner than a full `match`:

```rust
let value = Some(42);

// Full match — verbose when you only care about one arm
match value {
    Some(n) => println!("got {}", n),
    None => {}
}

// if let — cleaner
if let Some(n) = value {
    println!("got {}", n);
}
```

| | `match` | `if let` |
|---|---|---|
| Handles | ALL variants (exhaustive) | ONE pattern, ignores the rest |
| Use when | Multiple variants need handling | You only care about one case |

---

## Summary

| Concept | Syntax | What it means |
|---|---|---|
| Basic match | `match val { pat => code }` | Check val against patterns |
| Wildcard | `_ => code` | Catch-all |
| Extract data | `Some(n) => ...` | Pull data out of a variant |
| Match expression | `let x = match { ... }` | match returns a value |
| Multiple patterns | `1 \| 2 => code` | Match either |
| `if let` | `if let Some(n) = val { }` | Match one pattern, ignore the rest |
| Exhaustive | All variants must be handled | Compiler error if you miss one |

---

## Mini Quiz

**Q1:** Does this compile?
```rust
enum Color { Red, Green, Blue }

let c = Color::Red;
match c {
    Color::Red => println!("red"),
    Color::Green => println!("green"),
}
```
(a) Yes  (b) No — why?

**Q2:** What does this print?
```rust
let x = Some(5);
if let Some(n) = x {
    println!("{}", n * 2);
}
```
(a) Nothing  (b) `5`  (c) `10`

**Q3:** What's the difference between `match` and `if let`?

---

## Answers

**Q1: (b) No — compiler error.**
`Color::Blue` is not handled. `match` must be exhaustive — every variant needs an arm or a `_` wildcard.

**Q2: (c) `10`.**
`x` is `Some(5)` so the pattern matches. `n` is bound to `5`. `5 * 2 = 10`. If `x` were `None`, the block would be skipped entirely.

**Q3:**
- `match` is exhaustive — the compiler forces you to handle every variant
- `if let` handles one pattern and silently ignores everything else
- Use `match` when multiple variants need different handling. Use `if let` when you only care about one case.

---

## Next

→ [Concept 11: Generics `<T>`](11-generics.md)
