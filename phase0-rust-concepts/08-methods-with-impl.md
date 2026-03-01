# Concept 8: Methods with `impl`

In Concept 7 you learned how to create structs with data. But structs are just data containers — they don't *do* anything. Methods let you attach **behavior** to your structs.

---

## What is `impl`?

`impl` stands for "implementation." It's where you define functions that belong to a struct:

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

Now `Rectangle` has an `area()` method. You call it with dot notation:

```rust
let rect = Rectangle { width: 10.0, height: 5.0 };
println!("Area: {}", rect.area());   // Area: 50.0
```

---

## `&self` — The Key Concept

The first parameter of a method is always some form of `self` — it's how the method accesses the struct's data:

```rust
impl Rectangle {
    fn area(&self) -> f64 {
    //      ^^^^^ borrows the struct (read-only)
        self.width * self.height
    }
}
```

`&self` is shorthand for `self: &Rectangle`. It means "I am borrowing this Rectangle, not taking ownership."

### Three forms of `self`

| Parameter | Meaning | When to use |
|---|---|---|
| `&self` | Immutable borrow — read only | Most methods (just reading fields) |
| `&mut self` | Mutable borrow — can modify | Methods that change the struct's data |
| `self` | Takes ownership — consumes the struct | Methods that transform or destroy the struct |

---

## `&self` — Read-Only Methods

```rust
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

let rect = Rectangle { width: 10.0, height: 5.0 };
println!("{}", rect.area());       // 50.0
println!("{}", rect.is_square());  // false
println!("{}", rect.area());       // still works — &self only borrows
```

After calling `rect.area()`, `rect` is still alive. `&self` borrows — doesn't consume.

---

## `&mut self` — Methods That Modify

```rust
impl Rectangle {
    fn double_width(&mut self) {
        self.width *= 2.0;
    }
}

let mut rect = Rectangle { width: 10.0, height: 5.0 };
//  ^^^ must be mut to call &mut self methods
rect.double_width();
println!("{}", rect.width);   // 20.0
```

**Two things must be `mut`:**
1. The variable: `let mut rect`
2. The method parameter: `&mut self`

Same rules as Concept 4 — mutable reference requires a mutable variable.

---

## `self` — Methods That Consume

```rust
impl Rectangle {
    fn into_square(self) -> Rectangle {
    //             ^^^^ takes ownership — rect is consumed
        Rectangle {
            width: self.width,
            height: self.width,
        }
    }
}

let rect = Rectangle { width: 10.0, height: 5.0 };
let square = rect.into_square();
// println!("{}", rect.width);  // COMPILER ERROR — rect was moved
println!("{}", square.width);   // 10.0
println!("{}", square.height);  // 10.0
```

After calling `rect.into_square()`, `rect` is dead — the method consumed it. Useful for "transform and return a new version" patterns.

---

## Associated Functions (no `self`)

Functions inside `impl` that don't take `self` are called "associated functions." They belong to the type, not to an instance. You call them with `::` instead of `.`:

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }  // field init shorthand
    }

    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Called with :: not .
let rect = Rectangle::new(10.0, 5.0);
let sq = Rectangle::square(7.0);
```

**`new` is a convention, not a keyword.** Rust doesn't have constructors. `new()` is just a common name for "create a new instance."

**Field init shorthand:** `Rectangle { width, height }` — when the variable name matches the field name, you can skip the `: value` part.

---

## Multiple `impl` Blocks

You can split methods across multiple `impl` blocks:

```rust
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
```

Both methods work on `Rectangle`. Useful when organizing code or when trait implementations go in separate blocks (Concept 12).

---

## Methods Follow Ownership Rules

Methods follow the same ownership rules as functions:

```rust
let rect = Rectangle::new(10.0, 5.0);

// &self methods — can call multiple times, no move
rect.area();
rect.is_square();
rect.area();           // still valid

// self method — consumes rect
let sq = rect.into_square();
// rect is dead now
```

This connects directly to Concepts 2-4:
- `&self` = immutable borrow (Concept 3)
- `&mut self` = mutable borrow (Concept 4)
- `self` = move/ownership transfer (Concept 2)

---

## A Complete Example

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }      // associated function — constructor
    }

    fn value(&self) -> u32 {
        self.count                 // &self — read only
    }

    fn increment(&mut self) {
        self.count += 1;           // &mut self — modifies
    }

    fn into_inner(self) -> u32 {
        self.count                 // self — consumes, returns value
    }
}

let mut c = Counter::new();        // Counter::new() — associated function
c.increment();                      // &mut self — needs let mut
c.increment();
println!("{}", c.value());         // &self — prints 2
let final_count = c.into_inner();  // self — c is consumed
// c is dead here
println!("{}", final_count);       // 2
```

---

## Summary

| Concept | Syntax | Meaning |
|---|---|---|
| Impl block | `impl Foo { ... }` | Attach methods to a struct |
| Read method | `fn foo(&self) -> T` | Borrows struct, read-only |
| Mutate method | `fn foo(&mut self)` | Borrows struct, can modify fields |
| Consume method | `fn foo(self) -> T` | Takes ownership, struct gone after |
| Associated function | `fn foo() -> T` (no self) | Called with `Foo::foo()`, not `instance.foo()` |
| Constructor convention | `fn new(...) -> Foo` | Not a keyword, just a naming pattern |
| Field init shorthand | `Foo { width, height }` | When variable name matches field name |

---

## Mini Quiz

**Q1:** What does `&self` mean in a method?
(a) The method takes ownership of the struct
(b) The method borrows the struct immutably (read-only)
(c) The method can modify the struct's fields

**Q2:** Does this compile?
```rust
struct Dog {
    name: String,
}

impl Dog {
    fn speak(&self) {
        println!("{} says woof!", self.name);
    }
}

let d = Dog { name: String::from("Rex") };
d.speak();
d.speak();
```
(a) Yes  (b) No — why?

**Q3:** What's wrong here?
```rust
let c = Counter::new();
c.increment();
```

---

## Answers

**Q1: (b) The method borrows the struct immutably.**
`&self` is shorthand for `self: &Rectangle` — an immutable borrow. Read-only access. The struct is still alive after the method returns.

**Q2: (a) Yes.**
`speak()` takes `&self` — an immutable borrow. You can call it as many times as you want. `d` is never moved or consumed.

**Q3:** `c` is not declared as `mut`. `increment()` takes `&mut self`, which requires the variable to be `let mut`:
```rust
let mut c = Counter::new();  // fix: add mut
c.increment();                // now works
```
`&mut self` methods need `let mut` on the variable — same rule as Concept 4.

---

## Next

→ [Concept 9: Enums and Variants](09-enums-and-variants.md)
