//! # Challenge 10.1: Typestate Pattern
//!
//! ## Problem
//! Model a transaction lifecycle using the typestate pattern where invalid state
//! transitions are caught at compile time. A transaction moves through:
//! Unsigned → Signed → Submitted → Confirmed.
//!
//! ## Why This Matters
//! Agave uses typestates for transaction lifecycle stages. The typestate pattern
//! encodes state machine rules in the type system — if it compiles, the state
//! machine is correct. This eliminates entire classes of runtime bugs and is a
//! common pattern in production Rust codebases.
//!
//! ## Requirements
//! - Zero-sized marker types for each state: `Unsigned`, `Signed`, `Submitted`, `Confirmed`
//! - `TransactionBuilder<State>` generic over the state marker
//! - `new(message)` → `TransactionBuilder<Unsigned>`
//! - `sign(key)` on Unsigned → `TransactionBuilder<Signed>` (consumes self)
//! - `submit()` on Signed → `TransactionBuilder<Submitted>` (consumes self)
//! - `confirm(slot)` on Submitted → `TransactionBuilder<Confirmed>` (consumes self)
//! - `message()` available on all states
//! - `signature()` available only on Signed and later
//! - `slot()` available only on Confirmed
//!
//! ## Constraints
//! - Invalid transitions must NOT compile (you can't submit an unsigned tx)
//! - Each transition consumes self, preventing reuse of old states
//!
//! ## Hints
//! - Use `PhantomData<State>` if the state marker isn't stored in a field
//! - Each state-specific method is an `impl TransactionBuilder<SpecificState>` block
//! - Store accumulated data (signature, slot) as Options that get filled during transitions

use std::marker::PhantomData;

// State marker types — zero-sized, used only at the type level
pub struct Unsigned;
pub struct Signed;
pub struct Submitted;
pub struct Confirmed;

/// A transaction builder that tracks its lifecycle state at the type level.
pub struct TransactionBuilder<State> {
    // TODO: implement fields
    // - message: String
    // - signature: Option<String>
    // - slot: Option<u64>
    // - _state: PhantomData<State>
    _placeholder: PhantomData<State>,
}

// Methods available in ALL states
impl<State> TransactionBuilder<State> {
    /// Get the transaction message (available in any state).
    pub fn message(&self) -> &str {
        todo!("Return the message")
    }
}

// Methods only on Unsigned
impl TransactionBuilder<Unsigned> {
    /// Create a new unsigned transaction with the given message.
    pub fn new(_message: String) -> Self {
        todo!("Create TransactionBuilder<Unsigned> with message")
    }

    /// Sign the transaction, consuming the Unsigned builder and producing a Signed one.
    pub fn sign(self, _key: &str) -> TransactionBuilder<Signed> {
        todo!("Consume self, create a Signed builder with signature = hash(key + message)")
    }
}

// Methods only on Signed
impl TransactionBuilder<Signed> {
    /// Get the signature (available once signed).
    pub fn signature(&self) -> &str {
        todo!("Return the signature")
    }

    /// Submit the transaction, consuming the Signed builder.
    pub fn submit(self) -> TransactionBuilder<Submitted> {
        todo!("Consume self, create a Submitted builder")
    }
}

// Methods only on Submitted
impl TransactionBuilder<Submitted> {
    /// Get the signature (still available after submission).
    pub fn signature(&self) -> &str {
        todo!("Return the signature")
    }

    /// Confirm the transaction at the given slot.
    pub fn confirm(self, _slot: u64) -> TransactionBuilder<Confirmed> {
        todo!("Consume self, create a Confirmed builder with slot")
    }
}

// Methods only on Confirmed
impl TransactionBuilder<Confirmed> {
    /// Get the signature.
    pub fn signature(&self) -> &str {
        todo!("Return the signature")
    }

    /// Get the confirmation slot (only available on confirmed transactions).
    pub fn slot(&self) -> u64 {
        todo!("Return the slot")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unsigned() {
        let tx = TransactionBuilder::<Unsigned>::new("transfer 100 SOL".to_string());
        assert_eq!(tx.message(), "transfer 100 SOL");
    }

    #[test]
    fn test_sign_transition() {
        let tx = TransactionBuilder::new("transfer".to_string());
        let signed = tx.sign("my_secret_key");
        assert_eq!(signed.message(), "transfer");
        assert!(!signed.signature().is_empty());
    }

    #[test]
    fn test_submit_transition() {
        let tx = TransactionBuilder::new("transfer".to_string());
        let signed = tx.sign("key");
        let sig = signed.signature().to_string();
        let submitted = signed.submit();
        assert_eq!(submitted.message(), "transfer");
        assert_eq!(submitted.signature(), sig);
    }

    #[test]
    fn test_confirm_transition() {
        let tx = TransactionBuilder::new("transfer".to_string());
        let confirmed = tx.sign("key").submit().confirm(42);
        assert_eq!(confirmed.message(), "transfer");
        assert_eq!(confirmed.slot(), 42);
        assert!(!confirmed.signature().is_empty());
    }

    #[test]
    fn test_full_lifecycle() {
        let confirmed = TransactionBuilder::new("send 1 SOL to Alice".to_string())
            .sign("bobs_key")
            .submit()
            .confirm(12345);

        assert_eq!(confirmed.message(), "send 1 SOL to Alice");
        assert_eq!(confirmed.slot(), 12345);
        assert!(!confirmed.signature().is_empty());
    }

    #[test]
    fn test_different_keys_different_signatures() {
        let tx1 = TransactionBuilder::new("msg".to_string()).sign("key_a");
        let tx2 = TransactionBuilder::new("msg".to_string()).sign("key_b");
        assert_ne!(tx1.signature(), tx2.signature());
    }

    // NOTE: The following should NOT compile if uncommented — that's the point
    // of typestates. You can verify by uncommenting and checking for compile errors.
    //
    // #[test]
    // fn test_cannot_submit_unsigned() {
    //     let tx = TransactionBuilder::new("msg".to_string());
    //     tx.submit(); // ERROR: no method `submit` on TransactionBuilder<Unsigned>
    // }
    //
    // #[test]
    // fn test_cannot_confirm_unsigned() {
    //     let tx = TransactionBuilder::new("msg".to_string());
    //     tx.confirm(1); // ERROR: no method `confirm` on TransactionBuilder<Unsigned>
    // }
    //
    // #[test]
    // fn test_cannot_sign_twice() {
    //     let signed = TransactionBuilder::new("msg".to_string()).sign("key");
    //     signed.sign("key2"); // ERROR: no method `sign` on TransactionBuilder<Signed>
    // }
}
