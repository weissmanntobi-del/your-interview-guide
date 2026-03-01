//! # Challenge 03: Bundle Processor (Jito-Style)
//!
//! ## Problem
//! Implement an all-or-nothing bundle processor inspired by Jito's MEV bundle system.
//! A bundle is a sequence of transactions that must execute atomically: if any single
//! transaction fails, the entire bundle is rolled back and none of the state changes
//! persist. Bundles include a tip transaction that pays the validator.
//!
//! ## Why This Matters
//! Jito bundles are a core MEV primitive on Solana. Searchers submit bundles to
//! guarantee atomicity of multi-step strategies (e.g., sandwich, arbitrage). The
//! validator extracts the tip only if the entire bundle lands. Understanding this
//! pattern teaches rollback semantics, state snapshots, and tip validation.
//!
//! ## Requirements
//! - `validate_bundle` checks: non-empty, at least one tip transaction, tip <= max_tip.
//! - `process_bundle` executes each transaction in order against an `AccountState`.
//!   - For each transaction: debit `from`, credit `to`.
//!   - If any transaction fails (insufficient balance), rollback ALL changes and
//!     return `BundleResult { success: false, tip_extracted: 0, state_changes: vec![] }`.
//!   - On success, return collected state changes and the total tip extracted.
//!
//! ## Constraints
//! - Account balances are `u64`; a debit that would underflow is a failure.
//! - Tip transactions still transfer `amount` from `from` to `to`; the `is_tip` flag
//!   marks the amount as the validator tip (summed in `tip_extracted`).
//! - State changes are `(pubkey, delta)` where delta is positive for credits and
//!   negative for debits (i64).
//!
//! ## Hints
//! - Clone the account state before processing; restore the clone on failure.
//! - Collect state changes as you go so you can return them on success.
//! - `validate_bundle` is a pure check and does not modify state.

use std::collections::HashMap;

pub type Pubkey = [u8; 32];

/// Represents a single transaction within a bundle.
#[derive(Debug, Clone)]
pub struct BundleTransaction {
    pub id: u64,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub is_tip: bool,
}

/// A bundle of transactions to execute atomically.
#[derive(Debug, Clone)]
pub struct Bundle {
    pub id: u64,
    pub transactions: Vec<BundleTransaction>,
    pub max_tip: u64,
}

/// The result of processing a bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct BundleResult {
    pub bundle_id: u64,
    pub success: bool,
    pub tip_extracted: u64,
    /// Each entry is (pubkey, net_balance_change) where negative means debit.
    pub state_changes: Vec<(Pubkey, i64)>,
}

/// Errors that can occur during bundle validation.
#[derive(Debug, Clone, PartialEq)]
pub enum BundleError {
    EmptyBundle,
    NoTipTransaction,
    TipExceedsMax { total_tip: u64, max_tip: u64 },
}

/// Account balances keyed by pubkey.
pub type AccountState = HashMap<Pubkey, u64>;

/// Validate a bundle without modifying any state.
///
/// Checks:
/// 1. Bundle is non-empty.
/// 2. At least one transaction has `is_tip == true`.
/// 3. Sum of tip amounts does not exceed `bundle.max_tip`.
pub fn validate_bundle(bundle: &Bundle) -> Result<(), BundleError> {
    todo!(
        "Check for empty transactions, presence of a tip, and that the \
         total tip amount does not exceed max_tip"
    )
}

/// Process a bundle against account state. All-or-nothing semantics.
///
/// 1. Snapshot state before processing.
/// 2. Execute each transaction in order (debit `from`, credit `to`).
/// 3. If any debit causes an underflow, restore the snapshot and return failure.
/// 4. On success, return the bundle result with collected state changes and tip total.
pub fn process_bundle(bundle: &Bundle, state: &mut AccountState) -> BundleResult {
    todo!(
        "Clone state as snapshot. Iterate transactions, debit from, credit to. \
         On underflow, restore snapshot and return failure result. \
         Collect (pubkey, delta) state changes and sum tip amounts."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pubkey(n: u8) -> Pubkey {
        let mut k = [0u8; 32];
        k[0] = n;
        k
    }

    fn setup_state() -> AccountState {
        let mut state = AccountState::new();
        state.insert(pubkey(1), 10_000);
        state.insert(pubkey(2), 5_000);
        state.insert(pubkey(3), 0); // validator
        state
    }

    #[test]
    fn test_validate_empty_bundle() {
        let bundle = Bundle { id: 1, transactions: vec![], max_tip: 100 };
        assert_eq!(validate_bundle(&bundle), Err(BundleError::EmptyBundle));
    }

    #[test]
    fn test_validate_no_tip() {
        let bundle = Bundle {
            id: 1,
            transactions: vec![BundleTransaction {
                id: 1, from: pubkey(1), to: pubkey(2), amount: 100, is_tip: false,
            }],
            max_tip: 100,
        };
        assert_eq!(validate_bundle(&bundle), Err(BundleError::NoTipTransaction));
    }

    #[test]
    fn test_validate_tip_exceeds_max() {
        let bundle = Bundle {
            id: 1,
            transactions: vec![
                BundleTransaction { id: 1, from: pubkey(1), to: pubkey(2), amount: 100, is_tip: false },
                BundleTransaction { id: 2, from: pubkey(1), to: pubkey(3), amount: 200, is_tip: true },
            ],
            max_tip: 100,
        };
        assert_eq!(
            validate_bundle(&bundle),
            Err(BundleError::TipExceedsMax { total_tip: 200, max_tip: 100 })
        );
    }

    #[test]
    fn test_validate_valid_bundle() {
        let bundle = Bundle {
            id: 1,
            transactions: vec![
                BundleTransaction { id: 1, from: pubkey(1), to: pubkey(2), amount: 100, is_tip: false },
                BundleTransaction { id: 2, from: pubkey(1), to: pubkey(3), amount: 50, is_tip: true },
            ],
            max_tip: 100,
        };
        assert!(validate_bundle(&bundle).is_ok());
    }

    #[test]
    fn test_process_successful_bundle() {
        let mut state = setup_state();
        let bundle = Bundle {
            id: 1,
            transactions: vec![
                BundleTransaction { id: 1, from: pubkey(1), to: pubkey(2), amount: 1_000, is_tip: false },
                BundleTransaction { id: 2, from: pubkey(1), to: pubkey(3), amount: 500, is_tip: true },
            ],
            max_tip: 1_000,
        };
        let result = process_bundle(&bundle, &mut state);
        assert!(result.success);
        assert_eq!(result.tip_extracted, 500);
        assert_eq!(state[&pubkey(1)], 10_000 - 1_000 - 500);
        assert_eq!(state[&pubkey(2)], 5_000 + 1_000);
        assert_eq!(state[&pubkey(3)], 0 + 500);
    }

    #[test]
    fn test_process_rollback_on_insufficient_funds() {
        let mut state = setup_state();
        let bundle = Bundle {
            id: 2,
            transactions: vec![
                BundleTransaction { id: 1, from: pubkey(1), to: pubkey(2), amount: 9_000, is_tip: false },
                // This will fail: pubkey(1) only has 1_000 left after the first tx
                BundleTransaction { id: 2, from: pubkey(1), to: pubkey(3), amount: 2_000, is_tip: true },
            ],
            max_tip: 5_000,
        };
        let result = process_bundle(&bundle, &mut state);
        assert!(!result.success);
        assert_eq!(result.tip_extracted, 0);
        // State must be restored to original values
        assert_eq!(state[&pubkey(1)], 10_000);
        assert_eq!(state[&pubkey(2)], 5_000);
        assert_eq!(state[&pubkey(3)], 0);
    }

    #[test]
    fn test_process_new_account_credited() {
        let mut state = setup_state();
        let new_acct = pubkey(99);
        let bundle = Bundle {
            id: 3,
            transactions: vec![
                BundleTransaction { id: 1, from: pubkey(1), to: new_acct, amount: 100, is_tip: false },
                BundleTransaction { id: 2, from: pubkey(1), to: pubkey(3), amount: 10, is_tip: true },
            ],
            max_tip: 100,
        };
        let result = process_bundle(&bundle, &mut state);
        assert!(result.success);
        assert_eq!(*state.get(&new_acct).unwrap_or(&0), 100);
    }

    #[test]
    fn test_process_debit_from_zero_balance_fails() {
        let mut state = setup_state();
        let bundle = Bundle {
            id: 4,
            transactions: vec![
                // pubkey(3) has 0 balance
                BundleTransaction { id: 1, from: pubkey(3), to: pubkey(1), amount: 1, is_tip: false },
                BundleTransaction { id: 2, from: pubkey(1), to: pubkey(3), amount: 1, is_tip: true },
            ],
            max_tip: 100,
        };
        let result = process_bundle(&bundle, &mut state);
        assert!(!result.success);
    }

    #[test]
    fn test_result_contains_state_changes() {
        let mut state = setup_state();
        let bundle = Bundle {
            id: 5,
            transactions: vec![
                BundleTransaction { id: 1, from: pubkey(1), to: pubkey(2), amount: 500, is_tip: false },
                BundleTransaction { id: 2, from: pubkey(2), to: pubkey(3), amount: 100, is_tip: true },
            ],
            max_tip: 200,
        };
        let result = process_bundle(&bundle, &mut state);
        assert!(result.success);
        assert!(!result.state_changes.is_empty());
    }
}
