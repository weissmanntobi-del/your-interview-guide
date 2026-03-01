# Solana/Pinocchio Concept 04: Wincode

## Plain English

Wincode is a serialization format used in Pinocchio. Like Borsh, it converts structs to/from bytes — but it's designed to be even more minimal and efficient for on-chain Solana programs where compute units matter.

You implement `WincodeSerialize` and `WincodeDeserialize` for your types, either manually or via derive macros.

---

## Code Example

```rust
use wincode::{WincodeSerialize, WincodeDeserialize};

#[derive(WincodeSerialize, WincodeDeserialize, Debug)]
struct Transfer {
    amount: u64,
    recipient: [u8; 32],
}

fn main() {
    let tx = Transfer { amount: 1000, recipient: [0u8; 32] };

    let mut buf = vec![0u8; 40];
    tx.serialize(&mut buf).unwrap();

    let restored = Transfer::deserialize(&buf).unwrap();
    println!("{:?}", restored);
}
```

---

## Comparison Table

| | Borsh | Wincode |
|---|---|---|
| Used in | General Solana | Pinocchio specifically |
| Overhead | Low | Minimal |
| Derive macros | Yes | Yes |
| Focus | General purpose | On-chain compute efficiency |

---

## Mini Quiz

**Q1.** What is Wincode used for?
- A) Serializing and deserializing data in Pinocchio programs ✓
- B) Formatting output for display

**Q2.** How does Wincode differ from Borsh?
- A) Wincode uses text format, Borsh uses binary
- B) Wincode is more minimal, optimized for on-chain compute efficiency in Pinocchio ✓

---

## Quiz Answers

**Q1 → A.** Wincode is Pinocchio's serialization format — used to encode/decode instruction and account data in on-chain programs.

**Q2 → B.** Both are binary formats, but Wincode is more stripped down. On-chain programs have tight compute unit budgets — every byte and operation matters.

---

## Back to main path

→ [../README.md](../README.md)
