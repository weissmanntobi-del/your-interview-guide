# Rust Concepts — Learning Path

Concept-by-concept teaching notes for Pinocchio.

Each file covers one concept: plain-English explanation, code examples, mini quiz, and recorded answers.

## Learning Path — Pure Rust (24 Concepts)

### Stop 1 & 2: Ownership (Concepts 1–6)
| # | File | Concept | Status |
|---|------|---------|--------|
| 1 | [01-what-is-ownership.md](01-what-is-ownership.md) | What is ownership? The 3 rules | Done |
| 2 | [02-move-semantics.md](02-move-semantics.md) | Move semantics (functions, assignment, return) | Done |
| 3 | [03-borrowing.md](03-borrowing.md) | Borrowing with `&` (immutable references) | Done |
| 4 | [04-mutable-references.md](04-mutable-references.md) | Mutable references `&mut` | Done |
| 5 | [05-borrow-checker-rules.md](05-borrow-checker-rules.md) | The borrow checker — complete summary | Done |
| 6 | [06-slices.md](06-slices.md) | String slices `&str` and array slices `&[T]` | Done |

### Stop 3: Structs & Enums (Concepts 7–10)
| # | File | Concept | Status |
|---|------|---------|--------|
| 7 | [07-structs-and-fields.md](07-structs-and-fields.md) | Structs and fields | Done |
| 8 | [08-methods-with-impl.md](08-methods-with-impl.md) | Methods with `impl` | Done |
| 9 | [09-enums-and-variants.md](09-enums-and-variants.md) | Enums and variants | Done |
| 10 | [10-match-and-pattern-matching.md](10-match-and-pattern-matching.md) | `match` and pattern matching | Done |

### Stop 4: Generics & Traits (Concepts 11–14)
| # | File | Concept | Status |
|---|------|---------|--------|
| 11 | [11-generics.md](11-generics.md) | Generics `<T>` | Done |
| 12 | [12-traits.md](12-traits.md) | Traits — defining and implementing | Done |
| 13 | [13-trait-bounds.md](13-trait-bounds.md) | Trait bounds | Done |
| 14 | [14-lifetimes.md](14-lifetimes.md) | Lifetimes `'a` | Done |

### Stop 5: Closures & Iterators (Concepts 15–17)
| # | File | Concept | Status |
|---|------|---------|--------|
| 15 | [15-closures.md](15-closures.md) | Closures `\|x\| x + 1` | Done |
| 16 | [16-fn-fnmut-fnonce.md](16-fn-fnmut-fnonce.md) | `Fn`, `FnMut`, `FnOnce` traits | Done |
| 17 | [17-iterator-chains.md](17-iterator-chains.md) | Iterator chains (map, filter, collect) | Done |

### Stop 6: Smart Pointers (Concepts 18–20)
| # | File | Concept | Status |
|---|------|---------|--------|
| 18 | [18-box.md](18-box.md) | `Box<T>` — heap allocation | Done |
| 19 | [19-rc.md](19-rc.md) | `Rc<T>` — reference counting | Done |
| 20 | [20-refcell.md](20-refcell.md) | `RefCell<T>` — interior mutability | Done |

### Stop 7: Advanced Rust Patterns (Concepts 21–24)
| # | File | Concept | Status |
|---|------|---------|--------|
| 21 | [21-impl-trait-vs-dyn-trait.md](21-impl-trait-vs-dyn-trait.md) | `impl Trait` vs `dyn Trait` — static vs dynamic dispatch | Done |
| 22 | [22-function-pointers.md](22-function-pointers.md) | Raw `fn()` function pointer type vs closure traits | Done |
| 23 | [23-error-handling.md](23-error-handling.md) | Error handling — `Result`, `?`, custom errors, `map_err` | Done |
| 24 | [24-from-into-conversions.md](24-from-into-conversions.md) | `From`/`Into` conversions and how `?` uses `From` | Done |

---

## Solana / Pinocchio Concepts

These build on the Rust fundamentals above but are specific to Solana and Pinocchio.

| # | File | Concept | Status |
|---|------|---------|--------|
| S1 | [solana-pinocchio/01-borsh.md](solana-pinocchio/01-borsh.md) | Borsh encoding/decoding | Done |
| S2 | [solana-pinocchio/02-zero-copy-parsing.md](solana-pinocchio/02-zero-copy-parsing.md) | Zero-copy parsing | Done |
| S3 | [solana-pinocchio/03-bytemuck.md](solana-pinocchio/03-bytemuck.md) | Bytemuck | Done |
| S4 | [solana-pinocchio/04-wincode.md](solana-pinocchio/04-wincode.md) | Wincode serialization/deserialization | Done |

---

## How to Use

1. Read the concept file top to bottom.
2. Try the mini quiz before reading the answers.
3. Say "next" to move to the next concept.
4. Come back to any file as a reference anytime.
5. After finishing pure Rust (1–24), move to the Solana/Pinocchio subfolder.

## Pinocchio Coverage

| Topic | Concepts that cover it |
|---|---|
| Closures | 15, 16 |
| Generics | 11, 13 |
| Traits | 12, 13, 21 |
| Function Pointers | 16, 22 |
| Smart Pointers | 18, 19, 20 |
| Serialization (Borsh) | S1 |
| Serialization (Bytemuck) | S3 |
| Serialization (Wincode) | S4 |
| Zero-copy parsing | S2 |
| Error Handling | 23, 24 |
