# Concept 12: Traits — Defining and Implementing

## Plain English

A trait is a **contract**. It says: "any type that implements this trait must have these methods."

Think of it like a job description. The trait says "you must be able to do X." Any type that agrees to that description can be used wherever that trait is required.

In Rust, different types can share behavior through traits. Instead of inheritance (like in OOP), Rust uses traits to define shared behavior.

---

## Code Example

```rust
// Define the trait — the contract
trait Speak {
    fn speak(&self) -> &str;
}

struct Dog;
struct Cat;

// Implement the trait for each type
impl Speak for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
}

impl Speak for Cat {
    fn speak(&self) -> &str {
        "Meow!"
    }
}

// A function that accepts any type implementing Speak
fn make_noise(animal: &impl Speak) {
    println!("{}", animal.speak());
}

fn main() {
    let d = Dog;
    let c = Cat;
    make_noise(&d); // Woof!
    make_noise(&c); // Meow!
}
```

**Key things to notice:**
- `trait Speak { ... }` defines the contract
- `impl Speak for Dog { ... }` fulfills the contract
- `&impl Speak` in the function means "any type that implements Speak"
- Each type provides its **own** implementation

---

## Default Method Bodies

Traits can provide a **default implementation** that types can use as-is or override:

```rust
trait Greet {
    fn hello(&self) -> String {
        String::from("Hello!") // default
    }
}

struct Person;
impl Greet for Person {} // uses the default — no override needed
```

---

## Comparison Table

| | Trait | Struct/Enum |
|---|---|---|
| Defines | Behavior (methods) | Data (fields) |
| Implemented by | Any type | Itself only |
| Used for | Shared behavior across types | Holding state |

---

## Mini Quiz

**Q1.** What is a trait in Rust?
- A) A way to inherit fields from another struct
- B) A contract that defines methods a type must implement ✓
- C) A type that holds data

**Q2.** Can a trait have a default method body?
- A) No, all methods must be implemented by each type
- B) Yes, types can use the default or override it ✓

**Q3.** What does `fn foo(x: &impl Speak)` mean?
- A) `x` must be the exact type `Speak`
- B) `x` can be any type that implements the `Speak` trait ✓

---

## Quiz Answers

**Q1 → B.** Traits define behavior (methods), not data. Structs hold data.

**Q2 → B.** Traits can have default method bodies. A type implementing the trait can choose to use the default or override it.

**Q3 → B.** `&impl Speak` is shorthand for "a reference to any type that implements `Speak`." It's not the trait itself used as a type.

---

## Next

→ [13-trait-bounds.md](13-trait-bounds.md)
