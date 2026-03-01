//! # Challenge 04: Incremental Account Hash Accumulator
//!
//! ## Problem
//! Implement an incremental hash accumulator that maintains a running hash over
//! all accounts. The critical property is **order independence**: the same set of
//! (pubkey, hash) pairs must produce the same accumulated hash regardless of the
//! order in which updates arrive.
//!
//! ## Why This Matters
//! Solana validators must agree on the "bank hash" -- a single hash representing
//! the entire account state at each slot. Since accounts are updated in parallel
//! across multiple threads, the accumulation strategy must be order-independent.
//! Solana achieves this by sorting account hashes or using XOR-based accumulation.
//! This challenge explores the design of such an accumulator.
//!
//! ## Requirements
//! - `account_hash(entry) -> [u8; 32]` computes a SHA-256 hash of a single
//!   account's fields (pubkey, lamports LE, owner, data).
//! - `HashAccumulator::new()` creates an empty accumulator.
//! - `update(&mut self, pubkey, hash)` sets the hash for a pubkey, replacing any
//!   previous hash for that pubkey.
//! - `remove(&mut self, pubkey)` removes the pubkey's hash from accumulation.
//! - `accumulate(&self) -> [u8; 32]` produces the final order-independent hash.
//!
//! ## Constraints
//! - The accumulate function must be order-independent. Suggested approach: sort
//!   all (pubkey, hash) pairs by pubkey, then hash the concatenation.
//! - Use `sha2` for all hashing (`use sha2::{Sha256, Digest};`).
//! - `account_hash` feeds: pubkey (32) + lamports (8, LE) + owner (32) + data (N).
//!
//! ## Hints
//! - Store a `HashMap<[u8;32], [u8;32]>` mapping pubkey -> account_hash.
//! - For `accumulate`, collect entries, sort by pubkey, then feed each
//!   (pubkey, hash) pair into a final SHA-256 hasher.
//! - An alternative order-independent approach is XOR of all hashes, but sorted
//!   Merkle-style is more collision-resistant.

use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// A lightweight account representation for hashing.
#[derive(Debug, Clone)]
pub struct AccountEntry {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub owner: [u8; 32],
    pub data: Vec<u8>,
}

/// Compute the SHA-256 hash of a single account's fields.
///
/// Hash input (in order): pubkey (32 bytes) || lamports (8 bytes LE) ||
/// owner (32 bytes) || data (variable).
pub fn account_hash(entry: &AccountEntry) -> [u8; 32] {
    todo!(
        "Create a Sha256 hasher, feed pubkey + lamports (LE) + owner + data, finalize."
    )
}

/// Incremental, order-independent hash accumulator over all accounts.
pub struct HashAccumulator {
    // TODO: choose an appropriate backing data structure
    _placeholder: (),
}

impl HashAccumulator {
    /// Create an empty accumulator.
    pub fn new() -> Self {
        todo!("Initialize the internal hash map")
    }

    /// Set (or replace) the hash associated with a pubkey.
    pub fn update(&mut self, pubkey: [u8; 32], hash: [u8; 32]) {
        todo!("Insert or overwrite the hash for this pubkey")
    }

    /// Remove a pubkey from the accumulator.
    pub fn remove(&mut self, pubkey: &[u8; 32]) {
        todo!("Remove the pubkey entry from the map")
    }

    /// Produce an order-independent accumulated hash.
    ///
    /// Strategy: sort all (pubkey, hash) pairs by pubkey, then hash the
    /// concatenation of (pubkey || hash) for each entry using SHA-256.
    /// If the accumulator is empty, return `[0u8; 32]`.
    pub fn accumulate(&self) -> [u8; 32] {
        todo!(
            "Collect all entries, sort by pubkey, feed each (pubkey, hash) \
             into a SHA-256 hasher, and return the final digest."
        )
    }

    /// Number of pubkeys tracked.
    pub fn len(&self) -> usize {
        todo!("Return the number of entries in the map")
    }

    /// Whether the accumulator is empty.
    pub fn is_empty(&self) -> bool {
        todo!("Return true if no entries are tracked")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(pk: u8, lamports: u64) -> AccountEntry {
        AccountEntry {
            pubkey: [pk; 32],
            lamports,
            owner: [0xFF; 32],
            data: vec![pk; 8],
        }
    }

    #[test]
    fn test_account_hash_deterministic() {
        let entry = make_entry(1, 100);
        let h1 = account_hash(&entry);
        let h2 = account_hash(&entry);
        assert_eq!(h1, h2);
        assert_ne!(h1, [0u8; 32]);
    }

    #[test]
    fn test_different_accounts_different_hashes() {
        let h1 = account_hash(&make_entry(1, 100));
        let h2 = account_hash(&make_entry(2, 100));
        let h3 = account_hash(&make_entry(1, 200));
        assert_ne!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_order_independence() {
        let e1 = make_entry(1, 100);
        let e2 = make_entry(2, 200);
        let e3 = make_entry(3, 300);

        let mut acc_a = HashAccumulator::new();
        acc_a.update(e1.pubkey, account_hash(&e1));
        acc_a.update(e2.pubkey, account_hash(&e2));
        acc_a.update(e3.pubkey, account_hash(&e3));

        let mut acc_b = HashAccumulator::new();
        acc_b.update(e3.pubkey, account_hash(&e3));
        acc_b.update(e1.pubkey, account_hash(&e1));
        acc_b.update(e2.pubkey, account_hash(&e2));

        assert_eq!(acc_a.accumulate(), acc_b.accumulate());
    }

    #[test]
    fn test_update_changes_result() {
        let e1 = make_entry(1, 100);
        let mut acc = HashAccumulator::new();
        acc.update(e1.pubkey, account_hash(&e1));
        let hash_before = acc.accumulate();

        let e1_updated = make_entry(1, 999);
        acc.update(e1_updated.pubkey, account_hash(&e1_updated));
        let hash_after = acc.accumulate();

        assert_ne!(hash_before, hash_after);
    }

    #[test]
    fn test_remove_and_readd() {
        let e1 = make_entry(1, 100);
        let h = account_hash(&e1);

        let mut acc = HashAccumulator::new();
        acc.update(e1.pubkey, h);
        let hash_with = acc.accumulate();

        acc.remove(&e1.pubkey);
        assert_eq!(acc.accumulate(), [0u8; 32]); // empty

        acc.update(e1.pubkey, h);
        assert_eq!(acc.accumulate(), hash_with);
    }

    #[test]
    fn test_empty_accumulator() {
        let acc = HashAccumulator::new();
        assert_eq!(acc.accumulate(), [0u8; 32]);
        assert!(acc.is_empty());
        assert_eq!(acc.len(), 0);
    }

    #[test]
    fn test_len_tracks_entries() {
        let mut acc = HashAccumulator::new();
        acc.update([1u8; 32], [0xAA; 32]);
        acc.update([2u8; 32], [0xBB; 32]);
        assert_eq!(acc.len(), 2);
        acc.remove(&[1u8; 32]);
        assert_eq!(acc.len(), 1);
    }

    #[test]
    fn test_update_same_key_does_not_increase_len() {
        let mut acc = HashAccumulator::new();
        acc.update([1u8; 32], [0xAA; 32]);
        acc.update([1u8; 32], [0xBB; 32]);
        assert_eq!(acc.len(), 1);
    }
}
