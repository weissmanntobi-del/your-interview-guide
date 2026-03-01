//! # Challenge 10.2: Declarative Macro Generator
//!
//! ## Problem
//! Write `macro_rules!` macros that generate Solana-style boilerplate: account structs
//! with serialization, program error enums, and instruction enums.
//!
//! ## Why This Matters
//! Solana's ecosystem relies heavily on macros: `declare_id!`, `msg!`, Anchor's
//! `#[account]`, and `#[program]` all generate boilerplate. Understanding declarative
//! macros means understanding how half the Solana code you read actually works.
//! Interviewers test macro skills to assess your ability to reduce boilerplate and
//! work with Solana tooling at a deep level.
//!
//! ## Requirements
//! - `define_account!` macro: generates a struct with named fields, a `new()` constructor,
//!   a `SPACE` const (sum of field sizes), and a `try_deserialize(&[u8]) -> Result<Self>`
//! - `define_error!` macro: generates a numbered error enum with Display impl
//! - `define_instruction!` macro: generates an enum of instruction variants with data
//!
//! ## Constraints
//! - No proc macros — `macro_rules!` only
//! - Generated code must compile and pass the tests
//! - Support at least u8, u16, u32, u64, bool, and [u8; N] field types
//!
//! ## Hints
//! - Use `macro_rules!` with repetition patterns `$($field:ident : $ty:ty),*`
//! - For SPACE: map each type to its known size using a helper macro or match
//! - For try_deserialize: read fields sequentially from the byte slice
//! - `stringify!` is useful for generating Display impls

/// Helper: returns the serialized size of a primitive type.
/// Students should implement this as part of the macro system.
pub fn size_of_type<T>() -> usize {
    std::mem::size_of::<T>()
}

// TODO: Implement define_account! macro
//
// Usage:
//   define_account!(TokenAccount {
//       mint: [u8; 32],
//       owner: [u8; 32],
//       amount: u64,
//       delegate_set: bool,
//   });
//
// Should generate:
//   - pub struct TokenAccount { pub mint: [u8; 32], pub owner: [u8; 32], ... }
//   - impl TokenAccount {
//       pub const SPACE: usize = 32 + 32 + 8 + 1;  // sum of field sizes
//       pub fn new(mint: [u8; 32], owner: [u8; 32], amount: u64, delegate_set: bool) -> Self
//       pub fn try_deserialize(data: &[u8]) -> Result<Self, &'static str>
//     }

macro_rules! define_account {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        todo!("Generate struct, SPACE const, new() constructor, and try_deserialize")
    };
}

// TODO: Implement define_error! macro
//
// Usage:
//   define_error!(TokenError {
//       InsufficientFunds,
//       InvalidOwner,
//       AccountFrozen,
//   });
//
// Should generate:
//   - pub enum TokenError { InsufficientFunds = 0, InvalidOwner = 1, AccountFrozen = 2 }
//   - impl Display for TokenError

macro_rules! define_error {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        todo!("Generate error enum with Display impl")
    };
}

// TODO: Implement define_instruction! macro
//
// Usage:
//   define_instruction!(TokenInstruction {
//       Transfer { amount: u64 },
//       Approve { amount: u64 },
//       Revoke,
//   });
//
// Should generate:
//   - pub enum TokenInstruction { Transfer { amount: u64 }, Approve { amount: u64 }, Revoke }

macro_rules! define_instruction {
    ($name:ident { $($variant:ident $({ $($vfield:ident : $vty:ty),* })?),* $(,)? }) => {
        todo!("Generate instruction enum")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Uncomment these tests once the macros are implemented:

    // define_account!(TestAccount {
    //     owner: [u8; 32],
    //     balance: u64,
    //     is_frozen: bool,
    // });

    // define_error!(TestError {
    //     NotFound,
    //     Unauthorized,
    //     Overflow,
    // });

    // define_instruction!(TestInstruction {
    //     Transfer { amount: u64 },
    //     Freeze,
    // });

    #[test]
    fn test_placeholder() {
        // Replace this with real tests once macros are implemented
        // Example tests that should pass:
        //
        // #[test]
        // fn test_account_space() {
        //     assert_eq!(TestAccount::SPACE, 32 + 8 + 1);
        // }
        //
        // #[test]
        // fn test_account_new() {
        //     let acc = TestAccount::new([1u8; 32], 1000, false);
        //     assert_eq!(acc.balance, 1000);
        //     assert_eq!(acc.is_frozen, false);
        // }
        //
        // #[test]
        // fn test_account_deserialize() {
        //     let acc = TestAccount::new([1u8; 32], 500, true);
        //     // Serialize manually
        //     let mut data = Vec::new();
        //     data.extend_from_slice(&[1u8; 32]);
        //     data.extend_from_slice(&500u64.to_le_bytes());
        //     data.push(1); // true
        //     let deserialized = TestAccount::try_deserialize(&data).unwrap();
        //     assert_eq!(deserialized.balance, acc.balance);
        //     assert_eq!(deserialized.is_frozen, acc.is_frozen);
        // }
        //
        // #[test]
        // fn test_account_deserialize_too_short() {
        //     let data = vec![0u8; 10]; // too short for TestAccount
        //     assert!(TestAccount::try_deserialize(&data).is_err());
        // }
        //
        // #[test]
        // fn test_error_display() {
        //     assert_eq!(format!("{}", TestError::NotFound), "NotFound");
        //     assert_eq!(format!("{}", TestError::Unauthorized), "Unauthorized");
        // }
        //
        // #[test]
        // fn test_instruction_variants() {
        //     let ix = TestInstruction::Transfer { amount: 100 };
        //     match ix {
        //         TestInstruction::Transfer { amount } => assert_eq!(amount, 100),
        //         _ => panic!("wrong variant"),
        //     }
        //     let _ = TestInstruction::Freeze; // unit variant
        // }
    }
}
