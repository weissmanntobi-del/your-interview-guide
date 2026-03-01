//! # Challenge 04: Durable Nonce Account Lifecycle
//!
//! ## Problem
//! Implement Solana's durable nonce mechanism. A nonce account stores a nonce value
//! that replaces the recent blockhash in a transaction, allowing transactions to
//! remain valid indefinitely. The lifecycle is: initialize, advance (change the nonce
//! value), verify (check that a nonce matches), and use (verify + advance atomically).
//!
//! ## Why This Matters
//! Durable nonces solve the problem of transaction expiration on Solana. Normal
//! transactions expire after ~60 seconds when the blockhash becomes stale. Durable
//! nonces allow offline signing, multi-party signing, and scheduled execution. They
//! are essential for custody solutions, governance proposals, and any workflow where
//! a transaction cannot be signed and submitted immediately.
//!
//! ## Requirements
//! - `initialize_nonce` creates a nonce account with a given authority and initial nonce value.
//! - `advance_nonce` changes the nonce value; only the authority can do this. Returns the old value.
//! - `verify_nonce` checks that a given nonce value matches the stored one.
//! - `use_nonce` atomically verifies the current nonce and advances to a new value.
//! - Proper error handling for all failure modes.
//!
//! ## Constraints
//! - Nonce values are `[u8; 32]` (representing a blockhash-like value).
//! - Authority is a `[u8; 32]` pubkey.
//! - `initialize_nonce` must fail if the account is already initialized.
//! - `advance_nonce` must fail if the account is not initialized or the authority mismatches.
//! - Rent / lamports are tracked but not enforced beyond existence.
//!
//! ## Hints
//! - Use a `HashMap<[u8; 32], NonceAccount>` for `NonceState`.
//! - `use_nonce` is literally verify + advance; implement it in terms of those two.
//! - For new nonce values in tests, use simple deterministic byte arrays.

use std::collections::HashMap;

pub type Pubkey = [u8; 32];

/// Errors that can occur during nonce operations.
#[derive(Debug, Clone, PartialEq)]
pub enum NonceError {
    /// The nonce account has not been initialized.
    NotInitialized,
    /// The nonce account is already initialized.
    AlreadyInitialized,
    /// The provided authority does not match the nonce account's authority.
    AuthorityMismatch,
    /// The provided nonce value does not match the stored one.
    InvalidNonce,
    /// The nonce account does not have sufficient lamports.
    InsufficientFunds,
    /// The nonce account was not found in state.
    AccountNotFound,
}

/// A durable nonce account.
#[derive(Debug, Clone, PartialEq)]
pub struct NonceAccount {
    pub pubkey: Pubkey,
    pub authority: Pubkey,
    pub nonce_value: Pubkey,
    pub lamports: u64,
    pub initialized: bool,
}

/// Global nonce state: a mapping from account pubkey to NonceAccount.
pub type NonceState = HashMap<Pubkey, NonceAccount>;

/// Initialize a new nonce account.
///
/// - The account must not already be initialized in `state`.
/// - Sets the authority, initial nonce value, and lamports.
/// - Inserts the new `NonceAccount` into `state`.
pub fn initialize_nonce(
    state: &mut NonceState,
    pubkey: Pubkey,
    authority: Pubkey,
    initial_nonce: Pubkey,
    lamports: u64,
) -> Result<(), NonceError> {
    todo!(
        "Check if pubkey already exists and is initialized. If so, return AlreadyInitialized. \
         Otherwise, insert a new NonceAccount with initialized=true."
    )
}

/// Advance the nonce to a new value. Only the authority can do this.
///
/// Returns the old nonce value on success.
pub fn advance_nonce(
    state: &mut NonceState,
    pubkey: &Pubkey,
    authority: &Pubkey,
    new_nonce: Pubkey,
) -> Result<Pubkey, NonceError> {
    todo!(
        "Look up the account. Verify it is initialized and the authority matches. \
         Swap in the new nonce value and return the old one."
    )
}

/// Verify that the provided nonce value matches the stored one.
pub fn verify_nonce(
    state: &NonceState,
    pubkey: &Pubkey,
    expected_nonce: &Pubkey,
) -> Result<(), NonceError> {
    todo!(
        "Look up the account. Verify it is initialized. \
         Compare expected_nonce with the stored nonce_value."
    )
}

/// Atomically verify the current nonce and advance to a new value.
///
/// This is the operation a transaction uses when it references a durable nonce.
/// If verification fails, the nonce is NOT advanced.
pub fn use_nonce(
    state: &mut NonceState,
    pubkey: &Pubkey,
    authority: &Pubkey,
    expected_nonce: &Pubkey,
    new_nonce: Pubkey,
) -> Result<(), NonceError> {
    todo!(
        "First verify_nonce. If it succeeds, advance_nonce. \
         If verify fails, return the error without modifying state."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pk(n: u8) -> Pubkey {
        let mut k = [0u8; 32];
        k[0] = n;
        k
    }

    fn nonce_val(n: u8) -> Pubkey {
        let mut v = [0u8; 32];
        v[31] = n;
        v
    }

    #[test]
    fn test_initialize_and_verify() {
        let mut state = NonceState::new();
        assert!(initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).is_ok());
        assert!(verify_nonce(&state, &pk(1), &nonce_val(1)).is_ok());
    }

    #[test]
    fn test_initialize_already_initialized() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();
        let result = initialize_nonce(&mut state, pk(1), pk(10), nonce_val(2), 1_000_000);
        assert_eq!(result, Err(NonceError::AlreadyInitialized));
    }

    #[test]
    fn test_advance_changes_nonce() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();

        let old = advance_nonce(&mut state, &pk(1), &pk(10), nonce_val(2)).unwrap();
        assert_eq!(old, nonce_val(1));

        // Old nonce should no longer verify
        assert_eq!(verify_nonce(&state, &pk(1), &nonce_val(1)), Err(NonceError::InvalidNonce));
        // New nonce should verify
        assert!(verify_nonce(&state, &pk(1), &nonce_val(2)).is_ok());
    }

    #[test]
    fn test_advance_wrong_authority() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();

        let result = advance_nonce(&mut state, &pk(1), &pk(99), nonce_val(2));
        assert_eq!(result, Err(NonceError::AuthorityMismatch));

        // Nonce should remain unchanged
        assert!(verify_nonce(&state, &pk(1), &nonce_val(1)).is_ok());
    }

    #[test]
    fn test_advance_not_initialized() {
        let mut state = NonceState::new();
        let result = advance_nonce(&mut state, &pk(1), &pk(10), nonce_val(2));
        assert_eq!(result, Err(NonceError::AccountNotFound));
    }

    #[test]
    fn test_verify_invalid_nonce() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();
        assert_eq!(verify_nonce(&state, &pk(1), &nonce_val(99)), Err(NonceError::InvalidNonce));
    }

    #[test]
    fn test_use_nonce_atomic_success() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();

        assert!(use_nonce(&mut state, &pk(1), &pk(10), &nonce_val(1), nonce_val(2)).is_ok());

        // Nonce should now be nonce_val(2)
        assert!(verify_nonce(&state, &pk(1), &nonce_val(2)).is_ok());
        assert_eq!(verify_nonce(&state, &pk(1), &nonce_val(1)), Err(NonceError::InvalidNonce));
    }

    #[test]
    fn test_use_nonce_fails_on_wrong_nonce() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();

        let result = use_nonce(&mut state, &pk(1), &pk(10), &nonce_val(99), nonce_val(2));
        assert_eq!(result, Err(NonceError::InvalidNonce));

        // Nonce should remain unchanged
        assert!(verify_nonce(&state, &pk(1), &nonce_val(1)).is_ok());
    }

    #[test]
    fn test_double_use_fails() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();

        // First use succeeds
        use_nonce(&mut state, &pk(1), &pk(10), &nonce_val(1), nonce_val(2)).unwrap();

        // Second use with old nonce fails (replay protection)
        let result = use_nonce(&mut state, &pk(1), &pk(10), &nonce_val(1), nonce_val(3));
        assert_eq!(result, Err(NonceError::InvalidNonce));
    }

    #[test]
    fn test_use_nonce_wrong_authority() {
        let mut state = NonceState::new();
        initialize_nonce(&mut state, pk(1), pk(10), nonce_val(1), 1_000_000).unwrap();

        let result = use_nonce(&mut state, &pk(1), &pk(99), &nonce_val(1), nonce_val(2));
        assert_eq!(result, Err(NonceError::AuthorityMismatch));
    }
}
