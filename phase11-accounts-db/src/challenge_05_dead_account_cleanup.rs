//! # Challenge 05: Dead Account Cleanup
//!
//! ## Problem
//! Implement a garbage collection system that identifies "dead" accounts (accounts
//! whose latest entry has zero lamports), calculates reclaimable storage, and
//! determines whether storage files should be shrunk.
//!
//! ## Why This Matters
//! Solana's AccountsDB accumulates stale data as accounts are updated across slots.
//! Accounts that reach zero lamports are effectively deleted, but their old storage
//! entries linger. The cleanup process must correctly identify which accounts are
//! truly dead (only the LATEST entry matters), calculate how much space can be
//! reclaimed, and decide when shrinking a storage file is worthwhile. Without
//! proper cleanup, validator disk usage would grow without bound.
//!
//! ## Requirements
//! - `find_dead_accounts(entries) -> Vec<[u8; 32]>` returns pubkeys where the
//!   latest entry (highest `slot_updated`) has zero lamports.
//! - `find_reclaimable_storage(entries, dead_pubkeys) -> CleanupResult` calculates
//!   total reclaimable bytes and affected slots for the given dead pubkeys.
//! - `clean(entries, dead_pubkeys) -> Vec<StorageEntry>` returns entries with all
//!   dead pubkey entries removed.
//! - `should_shrink(total_bytes, alive_bytes, threshold) -> bool` returns true if
//!   the ratio `alive_bytes / total_bytes` is below the threshold.
//!
//! ## Constraints
//! - An account is dead ONLY if its latest entry (highest slot_updated) has zero
//!   lamports. Earlier entries for the same pubkey may have non-zero lamports --
//!   that does not make the account alive.
//! - Reclaimable bytes for each dead entry = `data_size + ENTRY_OVERHEAD` where
//!   `ENTRY_OVERHEAD = 80` (matching AppendVec header size).
//! - `should_shrink` uses floating-point division; threshold is a fraction (e.g., 0.5).
//!
//! ## Hints
//! - Group entries by pubkey, then find the max slot for each group.
//! - A `HashMap<[u8;32], &StorageEntry>` keyed by pubkey, keeping only the entry
//!   with the highest slot, is a clean approach for finding dead accounts.
//! - `clean` should remove ALL entries for dead pubkeys, not just the latest.

use std::collections::HashMap;

/// Overhead per entry in storage (matches AppendVec header).
const ENTRY_OVERHEAD: usize = 80;

/// An entry representing one version of an account in storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageEntry {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub slot_updated: u64,
    pub storage_offset: usize,
    pub data_size: usize,
}

/// Result of analyzing dead accounts and reclaimable storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupResult {
    pub dead_accounts: Vec<[u8; 32]>,
    pub reclaimable_bytes: usize,
    pub slots_affected: Vec<u64>,
}

/// Find pubkeys whose latest entry has zero lamports.
///
/// For each unique pubkey, find the entry with the highest `slot_updated`.
/// If that entry has `lamports == 0`, the account is dead.
pub fn find_dead_accounts(entries: &[StorageEntry]) -> Vec<[u8; 32]> {
    todo!(
        "Group entries by pubkey, find the latest (highest slot) for each, \
         return pubkeys where latest.lamports == 0."
    )
}

/// Calculate reclaimable storage for the given dead pubkeys.
///
/// Every entry (not just the latest) belonging to a dead pubkey contributes
/// `data_size + ENTRY_OVERHEAD` reclaimable bytes. Collect all unique affected slots.
pub fn find_reclaimable_storage(
    entries: &[StorageEntry],
    dead_pubkeys: &[[u8; 32]],
) -> CleanupResult {
    todo!(
        "Iterate entries, filter to those whose pubkey is in dead_pubkeys, \
         sum reclaimable bytes, collect affected slots. Return CleanupResult."
    )
}

/// Remove all entries belonging to dead pubkeys.
pub fn clean(entries: &[StorageEntry], dead_pubkeys: &[[u8; 32]]) -> Vec<StorageEntry> {
    todo!(
        "Filter out every entry whose pubkey appears in dead_pubkeys."
    )
}

/// Determine whether a storage file should be shrunk.
///
/// Returns true if `alive_bytes / total_bytes < threshold`.
/// If `total_bytes` is 0, return false (nothing to shrink).
pub fn should_shrink(total_bytes: usize, alive_bytes: usize, threshold: f64) -> bool {
    todo!(
        "Compute the alive ratio as f64 and compare against threshold."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(pk: u8, lamports: u64, slot: u64, offset: usize, data_size: usize) -> StorageEntry {
        StorageEntry {
            pubkey: [pk; 32],
            lamports,
            slot_updated: slot,
            storage_offset: offset,
            data_size,
        }
    }

    #[test]
    fn test_identify_dead_accounts() {
        let entries = vec![
            entry(1, 100, 10, 0, 64),   // pk1 at slot 10 alive
            entry(1, 0, 20, 100, 64),    // pk1 at slot 20 dead (latest)
            entry(2, 50, 15, 200, 32),   // pk2 alive
        ];
        let dead = find_dead_accounts(&entries);
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0], [1u8; 32]);
    }

    #[test]
    fn test_earlier_zero_does_not_make_dead() {
        // pk1 had zero lamports at slot 5, but was revived at slot 10
        let entries = vec![
            entry(1, 0, 5, 0, 64),
            entry(1, 500, 10, 100, 64),
        ];
        let dead = find_dead_accounts(&entries);
        assert!(dead.is_empty());
    }

    #[test]
    fn test_reclaimable_bytes() {
        let entries = vec![
            entry(1, 100, 10, 0, 64),
            entry(1, 0, 20, 144, 64),
            entry(2, 50, 15, 288, 32),
        ];
        let dead = find_dead_accounts(&entries);
        let result = find_reclaimable_storage(&entries, &dead);
        // Both entries for pk1 are reclaimable: (64+80) + (64+80) = 288
        assert_eq!(result.reclaimable_bytes, 2 * (64 + ENTRY_OVERHEAD));
        assert_eq!(result.dead_accounts.len(), 1);
    }

    #[test]
    fn test_slots_affected() {
        let entries = vec![
            entry(1, 0, 10, 0, 64),
            entry(1, 0, 20, 144, 64),
        ];
        let dead = find_dead_accounts(&entries);
        let result = find_reclaimable_storage(&entries, &dead);
        let mut slots = result.slots_affected.clone();
        slots.sort();
        assert_eq!(slots, vec![10, 20]);
    }

    #[test]
    fn test_clean_removes_all_dead_entries() {
        let entries = vec![
            entry(1, 100, 10, 0, 64),
            entry(1, 0, 20, 144, 64),
            entry(2, 50, 15, 288, 32),
        ];
        let dead = find_dead_accounts(&entries);
        let cleaned = clean(&entries, &dead);
        assert_eq!(cleaned.len(), 1);
        assert_eq!(cleaned[0].pubkey, [2u8; 32]);
    }

    #[test]
    fn test_full_cleanup_pipeline() {
        let entries = vec![
            entry(1, 0, 10, 0, 100),
            entry(2, 200, 10, 180, 50),
            entry(3, 0, 5, 310, 30),
            entry(3, 0, 15, 420, 30),  // latest for pk3 also zero
        ];
        let dead = find_dead_accounts(&entries);
        assert_eq!(dead.len(), 2); // pk1 and pk3
        let cleaned = clean(&entries, &dead);
        assert_eq!(cleaned.len(), 1);
        assert_eq!(cleaned[0].pubkey, [2u8; 32]);
    }

    #[test]
    fn test_should_shrink_below_threshold() {
        assert!(should_shrink(1000, 400, 0.5));  // 40% alive < 50% threshold
        assert!(!should_shrink(1000, 600, 0.5)); // 60% alive >= 50% threshold
    }

    #[test]
    fn test_should_shrink_zero_total() {
        assert!(!should_shrink(0, 0, 0.5));
    }

    #[test]
    fn test_no_dead_accounts() {
        let entries = vec![
            entry(1, 100, 10, 0, 64),
            entry(2, 200, 20, 144, 32),
        ];
        let dead = find_dead_accounts(&entries);
        assert!(dead.is_empty());
        let cleaned = clean(&entries, &dead);
        assert_eq!(cleaned.len(), 2);
    }
}
