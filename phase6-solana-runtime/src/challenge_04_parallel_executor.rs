//! # Challenge 7.4: Parallel Transaction Executor (Sealevel)
//!
//! ## Problem
//! Implement Solana's key innovation: parallel transaction execution.
//! Transactions that touch non-overlapping accounts execute concurrently.
//!
//! ## Why This Matters
//! Sealevel is what makes Solana fast. Every Solana infra interview will
//! test whether you understand how parallel execution works, how conflicts
//! are detected, and what the throughput implications are.
//!
//! ## Requirements
//! - Analyze transactions to identify account read/write sets
//! - Group non-conflicting transactions into parallel batches
//! - Execute batches concurrently, conflicting transactions sequentially
//! - Return correct final account states

use std::collections::{HashMap, HashSet};

pub type Pubkey = [u8; 32];

#[derive(Debug, Clone)]
pub struct AccountAccess {
    pub pubkey: Pubkey,
    pub is_writable: bool,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub accounts: Vec<AccountAccess>,
    /// Simplified: just transfers `amount` from accounts[0] to accounts[1]
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct AccountState {
    pub balance: u64,
}

/// Determine which transactions can execute in parallel.
/// Returns groups where all transactions within a group are non-conflicting.
/// Two transactions conflict if they both access the same account AND
/// at least one access is writable.
pub fn schedule_parallel(
    _transactions: &[Transaction],
) -> Vec<Vec<usize>> {
    todo!("Group transaction indices into non-conflicting batches")
}

/// Execute all transactions, using parallelism where possible.
/// Returns the final account states after all transactions.
pub fn execute_parallel(
    _transactions: &[Transaction],
    _initial_state: &HashMap<Pubkey, AccountState>,
) -> HashMap<Pubkey, AccountState> {
    todo!("Schedule into batches, execute each batch (can be parallel), apply state changes")
}

/// Execute transactions sequentially for comparison.
pub fn execute_sequential(
    _transactions: &[Transaction],
    _initial_state: &HashMap<Pubkey, AccountState>,
) -> HashMap<Pubkey, AccountState> {
    todo!("Execute one by one, updating state after each")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key(id: u8) -> Pubkey {
        let mut key = [0u8; 32];
        key[0] = id;
        key
    }

    fn make_tx(id: u64, from: u8, to: u8, amount: u64) -> Transaction {
        Transaction {
            id,
            accounts: vec![
                AccountAccess { pubkey: make_key(from), is_writable: true },
                AccountAccess { pubkey: make_key(to), is_writable: true },
            ],
            amount,
        }
    }

    #[test]
    fn test_non_conflicting_are_parallel() {
        // A->B and C->D don't conflict — should be in same batch
        let txs = vec![
            make_tx(0, 1, 2, 100), // A -> B
            make_tx(1, 3, 4, 200), // C -> D
        ];
        let batches = schedule_parallel(&txs);
        // Both should be in the first batch
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].len(), 2);
    }

    #[test]
    fn test_conflicting_are_sequential() {
        // A->B and B->C conflict on B — must be in different batches
        let txs = vec![
            make_tx(0, 1, 2, 100), // A -> B
            make_tx(1, 2, 3, 200), // B -> C (conflicts on B)
        ];
        let batches = schedule_parallel(&txs);
        assert!(batches.len() >= 2);
    }

    #[test]
    fn test_execution_correctness() {
        let mut initial = HashMap::new();
        initial.insert(make_key(1), AccountState { balance: 1000 });
        initial.insert(make_key(2), AccountState { balance: 1000 });
        initial.insert(make_key(3), AccountState { balance: 1000 });

        // Non-conflicting: 1->2 (100) and 3->2... wait, 2 is shared
        // Let's do: 1->2 (100) independently
        let txs = vec![make_tx(0, 1, 2, 100)];

        let result = execute_parallel(&txs, &initial);
        assert_eq!(result.get(&make_key(1)).unwrap().balance, 900);
        assert_eq!(result.get(&make_key(2)).unwrap().balance, 1100);
    }

    #[test]
    fn test_parallel_matches_sequential() {
        let mut initial = HashMap::new();
        for i in 1..=6u8 {
            initial.insert(make_key(i), AccountState { balance: 10000 });
        }

        let txs = vec![
            make_tx(0, 1, 2, 100),
            make_tx(1, 3, 4, 200),
            make_tx(2, 5, 6, 300),
            make_tx(3, 1, 3, 50), // Conflicts with tx0 and tx1
        ];

        let parallel_result = execute_parallel(&txs, &initial);
        let sequential_result = execute_sequential(&txs, &initial);

        for key in initial.keys() {
            assert_eq!(
                parallel_result.get(key).unwrap().balance,
                sequential_result.get(key).unwrap().balance,
                "parallel and sequential must produce same state"
            );
        }
    }
}
