//! # Challenge 02: Write Lock Scheduler
//!
//! ## Problem
//! Implement a transaction scheduler that groups transactions into batches based on
//! account lock conflicts. Two transactions conflict if they both access the same
//! account and at least one of them holds a write lock. Read-read on the same account
//! is allowed within the same batch.
//!
//! ## Why This Matters
//! Solana processes transactions in parallel within a batch. Conflicting transactions
//! must be placed in separate sequential batches. Efficient scheduling maximizes
//! throughput by fitting as many non-conflicting transactions into each batch as
//! possible. This is central to the banking stage and the scheduler component.
//!
//! ## Requirements
//! - `conflicts(a, b)` returns true when two transactions have overlapping account
//!   locks where at least one side is a write lock.
//! - `schedule(txs)` produces a `Vec<ScheduleBatch>` using a greedy algorithm:
//!   iterate transactions in order; place each in the first batch where it has no
//!   conflict with any already-placed transaction; create a new batch if necessary.
//! - Read-read locks on the same account do NOT conflict.
//!
//! ## Constraints
//! - Account comparison is by pubkey bytes (`[u8; 32]`).
//! - Each transaction appears in exactly one batch.
//! - Batch indices are sequential starting from 0.
//!
//! ## Hints
//! - For `conflicts`, iterate all lock pairs and check pubkey equality.
//! - For greedy scheduling, maintain a list of batches. For each transaction, try
//!   each batch in order by checking conflict against all transactions already in it.
//! - Use a helper to look up a transaction by id from the original list.

pub type Pubkey = [u8; 32];

/// Describes a lock that a transaction holds on an account.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountLock {
    pub pubkey: Pubkey,
    pub is_write: bool,
}

/// A transaction with its declared account locks.
#[derive(Debug, Clone)]
pub struct SchedulableTransaction {
    pub id: u64,
    pub account_locks: Vec<AccountLock>,
}

/// A batch of transaction ids that can execute in parallel.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduleBatch {
    pub batch_index: usize,
    pub transaction_ids: Vec<u64>,
}

/// Determine whether two transactions conflict.
///
/// Two transactions conflict if they both lock the same account and at least
/// one of those locks is a write lock. Read-read on the same account is fine.
pub fn conflicts(a: &SchedulableTransaction, b: &SchedulableTransaction) -> bool {
    todo!(
        "For each pair of locks (one from a, one from b), check if pubkeys match \
         and at least one is_write is true"
    )
}

/// Schedule transactions into sequential batches using a greedy algorithm.
///
/// For each transaction (in the order given):
/// 1. Try to fit it into the earliest existing batch with no conflicts.
/// 2. If no existing batch works, create a new batch.
///
/// Returns a list of `ScheduleBatch` with batch_index starting at 0.
pub fn schedule(txs: &[SchedulableTransaction]) -> Vec<ScheduleBatch> {
    todo!(
        "Maintain a Vec of (ScheduleBatch, Vec of &SchedulableTransaction). \
         For each tx, scan batches and use `conflicts` to check compatibility."
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

    fn write_lock(n: u8) -> AccountLock {
        AccountLock { pubkey: pubkey(n), is_write: true }
    }

    fn read_lock(n: u8) -> AccountLock {
        AccountLock { pubkey: pubkey(n), is_write: false }
    }

    #[test]
    fn test_no_conflict_different_accounts() {
        let a = SchedulableTransaction { id: 1, account_locks: vec![write_lock(1)] };
        let b = SchedulableTransaction { id: 2, account_locks: vec![write_lock(2)] };
        assert!(!conflicts(&a, &b));
    }

    #[test]
    fn test_write_write_conflict() {
        let a = SchedulableTransaction { id: 1, account_locks: vec![write_lock(1)] };
        let b = SchedulableTransaction { id: 2, account_locks: vec![write_lock(1)] };
        assert!(conflicts(&a, &b));
    }

    #[test]
    fn test_read_read_no_conflict() {
        let a = SchedulableTransaction { id: 1, account_locks: vec![read_lock(1)] };
        let b = SchedulableTransaction { id: 2, account_locks: vec![read_lock(1)] };
        assert!(!conflicts(&a, &b));
    }

    #[test]
    fn test_read_write_conflict() {
        let a = SchedulableTransaction { id: 1, account_locks: vec![read_lock(1)] };
        let b = SchedulableTransaction { id: 2, account_locks: vec![write_lock(1)] };
        assert!(conflicts(&a, &b));
    }

    #[test]
    fn test_write_read_conflict() {
        let a = SchedulableTransaction { id: 1, account_locks: vec![write_lock(1)] };
        let b = SchedulableTransaction { id: 2, account_locks: vec![read_lock(1)] };
        assert!(conflicts(&a, &b));
    }

    #[test]
    fn test_schedule_non_conflicting_same_batch() {
        let txs = vec![
            SchedulableTransaction { id: 1, account_locks: vec![write_lock(1)] },
            SchedulableTransaction { id: 2, account_locks: vec![write_lock(2)] },
            SchedulableTransaction { id: 3, account_locks: vec![write_lock(3)] },
        ];
        let batches = schedule(&txs);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].transaction_ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_schedule_write_write_separates() {
        let txs = vec![
            SchedulableTransaction { id: 1, account_locks: vec![write_lock(1)] },
            SchedulableTransaction { id: 2, account_locks: vec![write_lock(1)] },
        ];
        let batches = schedule(&txs);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].transaction_ids, vec![1]);
        assert_eq!(batches[1].transaction_ids, vec![2]);
    }

    #[test]
    fn test_schedule_read_read_same_batch() {
        let txs = vec![
            SchedulableTransaction { id: 1, account_locks: vec![read_lock(1)] },
            SchedulableTransaction { id: 2, account_locks: vec![read_lock(1)] },
        ];
        let batches = schedule(&txs);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].transaction_ids, vec![1, 2]);
    }

    #[test]
    fn test_schedule_mixed_creates_multiple_batches() {
        // tx1 writes A, tx2 writes A (conflict with tx1), tx3 reads B (no conflict with either)
        let txs = vec![
            SchedulableTransaction { id: 1, account_locks: vec![write_lock(1)] },
            SchedulableTransaction { id: 2, account_locks: vec![write_lock(1)] },
            SchedulableTransaction { id: 3, account_locks: vec![read_lock(2)] },
        ];
        let batches = schedule(&txs);
        // tx1 in batch 0, tx2 in batch 1 (conflict with tx1), tx3 in batch 0 (no conflict)
        assert_eq!(batches.len(), 2);
        assert!(batches[0].transaction_ids.contains(&1));
        assert!(batches[0].transaction_ids.contains(&3));
        assert!(batches[1].transaction_ids.contains(&2));
    }

    #[test]
    fn test_schedule_empty_input() {
        let batches = schedule(&[]);
        assert!(batches.is_empty());
    }
}
