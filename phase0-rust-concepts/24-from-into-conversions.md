# Concept 24: `From`/`Into` Conversions

## Plain English

`From` and `Into` are traits for converting between types.

- `From<T>` — defines how to create your type **from** another type
- `Into<T>` — the reverse, automatically provided when you implement `From`

If you implement `From<A> for B`, Rust automatically gives you `Into<B> for A` for free.

The `?` operator also uses `From` under the hood — it converts error types automatically when propagating errors up the call stack.

---

## Code Example

```rust
#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let boiling = Celsius(100.0);
    let f = Fahrenheit::from(boiling);    // explicit
    println!("{:?}", f); // Fahrenheit(212.0)

    let freezing = Celsius(0.0);
    let f2: Fahrenheit = freezing.into(); // implicit — Into provided for free
    println!("{:?}", f2); // Fahrenheit(32.0)
}
```

---

## Comparison Table

| | `From` | `Into` |
|---|---|---|
| You implement | `From<T> for U` | Never — it's auto-provided |
| Call syntax | `U::from(t)` | `t.into()` |
| Relationship | Define once | Get the other for free |

---

## Mini Quiz

**Q1.** If you implement `From<A> for B`, what do you get for free?
- A) `Into<A> for B`
- B) `Into<B> for A` ✓

**Q2.** How does `?` use `From`?
- A) It uses `From` to convert error types automatically when propagating errors ✓
- B) It uses `From` to convert `Ok` values

---

## Quiz Answers

**Q1 → B.** Implementing `From<A> for B` automatically gives you `Into<B> for A`. You only ever implement `From` — `Into` is derived automatically.

**Q2 → A.** When `?` propagates an error, it calls `From::from()` on the error to convert it to the return type's error variant. This lets you use `?` across functions with different error types as long as the conversion is defined.

---

## Next

→ [solana-pinocchio/01-borsh.md](solana-pinocchio/01-borsh.md)
