//! # Challenge 02: Account Index
//!
//! ## Problem
//! Implement an in-memory index that maps account public keys to their storage
//! locations across multiple slots. Each account may exist in several slots (one
//! entry per slot), and the index must support efficient lookup of the latest
//! version as well as historical queries.
//!
//! ## Why This Matters
//! Solana's AccountsDB maintains an index from every account pubkey to the set of
//! (slot, offset) pairs where that account's data resides. This index is critical
//! for fast account lookups during transaction execution and for determining which
//! old entries can be cleaned. A validator that cannot index millions of accounts
//! efficiently will fall behind the cluster.
//!
//! ## Requirements
//! - `AccountIndex::new()` creates an empty index.
//! - `insert(&mut self, pubkey, slot, offset)` records a new location for the
//!   given pubkey at the given slot.
//! - `get_latest(&self, pubkey) -> Option<AccountLocation>` returns the location
//!   with the highest slot number for that pubkey.
//! - `get_all_slots(&self, pubkey) -> Vec<AccountLocation>` returns all locations
//!   for a pubkey, sorted by slot ascending.
//! - `get_all_keys_at_slot(&self, slot) -> Vec<[u8; 32]>` returns every pubkey
//!   that has an entry at the given slot.
//! - `remove_slot(&mut self, slot)` removes all entries for the given slot across
//!   all pubkeys. Pubkeys with no remaining entries are also removed.
//!
//! ## Constraints
//! - Use `std::collections::HashMap` (no external crate required).
//! - A pubkey can have at most one entry per slot (inserting the same pubkey+slot
//!   again overwrites the offset).
//! - `get_all_keys_at_slot` may return keys in any order.
//!
//! ## Hints
//! - Consider `HashMap<[u8;32], Vec<AccountLocation>>` or
//!   `HashMap<[u8;32], BTreeMap<u64, usize>>` as the backing structure.
//! - A secondary index `HashMap<u64, HashSet<[u8;32]>>` makes `get_all_keys_at_slot`
//!   and `remove_slot` efficient.

use std::collections::HashMap;

/// Where an account is physically stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccountLocation {
    pub slot: u64,
    pub offset: usize,
}

/// In-memory index mapping pubkey -> storage locations across slots.
pub struct AccountIndex {
    // TODO: choose an appropriate data structure
    _placeholder: (),
}

impl AccountIndex {
    /// Create a new empty index.
    pub fn new() -> Self {
        todo!("Initialize internal maps")
    }

    /// Record that `pubkey` is stored at `offset` in `slot`.
    /// If the same pubkey+slot already exists, overwrite the offset.
    pub fn insert(&mut self, pubkey: [u8; 32], slot: u64, offset: usize) {
        todo!(
            "Insert into primary index (pubkey -> locations). \
             Also update any secondary index for slot-based lookups."
        )
    }

    /// Return the location with the highest slot for this pubkey.
    pub fn get_latest(&self, pubkey: &[u8; 32]) -> Option<AccountLocation> {
        todo!("Find the entry with the maximum slot value for this pubkey")
    }

    /// Return all locations for this pubkey, sorted by slot ascending.
    pub fn get_all_slots(&self, pubkey: &[u8; 32]) -> Vec<AccountLocation> {
        todo!("Collect all entries for this pubkey and sort by slot")
    }

    /// Return all pubkeys that have an entry at the given slot.
    pub fn get_all_keys_at_slot(&self, slot: u64) -> Vec<[u8; 32]> {
        todo!("Use secondary index or scan primary index for entries at this slot")
    }

    /// Remove all entries across all pubkeys for the given slot.
    /// Also remove pubkeys that have no remaining entries.
    pub fn remove_slot(&mut self, slot: u64) {
        todo!(
            "Remove every entry whose slot matches. \
             Clean up pubkeys with empty location lists."
        )
    }

    /// Total number of distinct pubkeys in the index.
    pub fn num_pubkeys(&self) -> usize {
        todo!("Return number of unique pubkeys currently tracked")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pk(b: u8) -> [u8; 32] {
        [b; 32]
    }

    #[test]
    fn test_insert_and_get_latest() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(1), 10, 0);
        idx.insert(pk(1), 20, 100);
        let latest = idx.get_latest(&pk(1)).unwrap();
        assert_eq!(latest.slot, 20);
        assert_eq!(latest.offset, 100);
    }

    #[test]
    fn test_get_latest_returns_none_for_unknown() {
        let idx = AccountIndex::new();
        assert!(idx.get_latest(&pk(99)).is_none());
    }

    #[test]
    fn test_multiple_slots_same_key() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(1), 5, 0);
        idx.insert(pk(1), 10, 80);
        idx.insert(pk(1), 3, 160);
        let all = idx.get_all_slots(&pk(1));
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].slot, 3);
        assert_eq!(all[1].slot, 5);
        assert_eq!(all[2].slot, 10);
    }

    #[test]
    fn test_overwrite_same_slot() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(1), 10, 0);
        idx.insert(pk(1), 10, 999);
        let all = idx.get_all_slots(&pk(1));
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].offset, 999);
    }

    #[test]
    fn test_get_all_keys_at_slot() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(1), 10, 0);
        idx.insert(pk(2), 10, 80);
        idx.insert(pk(3), 20, 160);
        let mut keys = idx.get_all_keys_at_slot(10);
        keys.sort();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&pk(1)));
        assert!(keys.contains(&pk(2)));
    }

    #[test]
    fn test_remove_slot() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(1), 10, 0);
        idx.insert(pk(1), 20, 80);
        idx.insert(pk(2), 10, 160);
        idx.remove_slot(10);
        // pk(1) still has slot 20
        assert_eq!(idx.get_all_slots(&pk(1)).len(), 1);
        assert_eq!(idx.get_latest(&pk(1)).unwrap().slot, 20);
        // pk(2) had only slot 10 -> should be gone
        assert!(idx.get_latest(&pk(2)).is_none());
        assert_eq!(idx.num_pubkeys(), 1);
    }

    #[test]
    fn test_remove_slot_cleans_empty_pubkeys() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(5), 100, 0);
        idx.remove_slot(100);
        assert_eq!(idx.num_pubkeys(), 0);
        assert!(idx.get_all_keys_at_slot(100).is_empty());
    }

    #[test]
    fn test_multiple_pubkeys_independent() {
        let mut idx = AccountIndex::new();
        idx.insert(pk(1), 10, 0);
        idx.insert(pk(2), 20, 80);
        assert_eq!(idx.get_latest(&pk(1)).unwrap().slot, 10);
        assert_eq!(idx.get_latest(&pk(2)).unwrap().slot, 20);
        assert_eq!(idx.num_pubkeys(), 2);
    }
}
