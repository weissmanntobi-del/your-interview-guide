# Solana/Pinocchio Concept 01: Borsh

## Plain English

Borsh stands for **Binary Object Representation Serializer for Hashing**. It's a binary serialization format used heavily in Solana.

Serialization = converting a Rust struct into bytes so you can store or send it. Deserialization = converting bytes back into a struct.

Borsh is fast, deterministic (same input always produces same bytes), and has no extra overhead.

---

## Code Example

```rust
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let player = Player { name: "Alice".to_string(), score: 42 };

    // Serialize to bytes
    let bytes = player.try_to_vec().unwrap();
    println!("{:?}", bytes);

    // Deserialize back
    let restored = Player::try_from_slice(&bytes).unwrap();
    println!("{:?}", restored);
}
```

- `#[derive(BorshSerialize, BorshDeserialize)]` — auto-generates the encode/decode logic
- `.try_to_vec()` — converts struct to `Vec<u8>`
- `::try_from_slice()` — converts `&[u8]` back to struct

---

## Comparison Table

| | Borsh | JSON |
|---|---|---|
| Format | Binary | Text |
| Speed | Fast | Slower |
| Deterministic | Yes | No |
| Human readable | No | Yes |
| Used in | Solana programs | Web APIs |

---

## Mini Quiz

**Q1.** What does serialization mean in the context of Borsh?
- A) Converting a struct into bytes ✓
- B) Converting bytes into a struct

**Q2.** Why does Solana use Borsh instead of JSON?
- A) JSON is not supported in Rust
- B) Borsh is binary, fast, and deterministic — better for on-chain data ✓

---

## Quiz Answers

**Q1 → A.** Serialization = struct to bytes. Deserialization = bytes back to struct.

**Q2 → B.** Borsh is binary, fast, and deterministic. JSON is human-readable but slower and non-deterministic — not suitable for on-chain data.

---

## Next

→ [02-zero-copy-parsing.md](02-zero-copy-parsing.md)
