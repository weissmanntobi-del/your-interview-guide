# Solana/Pinocchio Concept 02: Zero-Copy Parsing

## Plain English

Zero-copy parsing means reading data from a byte buffer **without allocating new memory**. Instead of copying bytes into a new struct, you create references that point directly into the original buffer.

In Solana, account data arrives as a raw `&[u8]` buffer. Zero-copy lets you interpret that buffer as a struct directly — no copying, no allocating.

---

## Code Example

```rust
use std::mem;

#[repr(C)]  // ensures predictable memory layout
struct Header {
    version: u32,
    count: u32,
}

fn parse(data: &[u8]) -> &Header {
    // Reinterpret the bytes as a Header — no copy
    unsafe {
        &*(data.as_ptr() as *const Header)
    }
}
```

- `#[repr(C)]` — tells Rust to lay out fields in order, no padding surprises
- The cast reinterprets raw bytes as the struct — zero allocation

---

## Comparison Table

| | Normal parsing | Zero-copy parsing |
|---|---|---|
| Allocates memory | Yes | No |
| Speed | Slower | Faster |
| Safety | Safe | Requires `unsafe` or a crate |
| Use when | Small data, convenience | High-performance, on-chain data |

---

## Mini Quiz

**Q1.** What is the main benefit of zero-copy parsing?
- A) It makes code easier to read
- B) It avoids memory allocation by reading directly from the original buffer ✓

**Q2.** Why is `#[repr(C)]` important for zero-copy structs?
- A) It makes the struct serializable with Borsh
- B) It guarantees a predictable memory layout so bytes map correctly to fields ✓

---

## Quiz Answers

**Q1 → B.** Zero-copy avoids allocation entirely — you read directly from the original buffer without copying anything.

**Q2 → B.** `#[repr(C)]` ensures fields are laid out in order with no compiler-added padding, so the raw bytes map correctly to struct fields.

---

## Next

→ [03-bytemuck.md](03-bytemuck.md)
