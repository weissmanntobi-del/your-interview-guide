//! # Challenge 06: Write Cache
//!
//! ## Problem
//! Implement a write-through cache that buffers account writes and flushes them
//! in batches. The cache absorbs high-frequency writes, deduplicates by pubkey
//! (keeping only the latest write), and produces a sorted batch when flushed.
//!
//! ## Why This Matters
//! Solana processes thousands of account updates per slot. Writing each update
//! directly to AppendVec storage would be prohibitively slow. Instead, a write
//! cache accumulates updates in memory and flushes them to storage in sorted
//! batches at slot boundaries. This batching strategy reduces I/O operations,
//! enables deduplication (only the final state of each account per slot matters),
//! and ensures deterministic write ordering.
//!
//! ## Requirements
//! - `WriteCache::new(capacity)` creates a cache that triggers a flush when it
//!   holds `capacity` entries.
//! - `write(&mut self, account)` inserts or overwrites the entry for a pubkey.
//!   If an entry with the same pubkey already exists, it is replaced (latest wins).
//! - `read(&self, pubkey) -> Option<&CachedAccount>` reads back from the cache.
//! - `flush(&mut self) -> Vec<CachedAccount>` drains the cache and returns entries
//!   sorted by (slot ascending, then pubkey ascending). The cache is empty after.
//! - `should_flush(&self) -> bool` returns true when len >= capacity.
//! - `len(&self)` and `is_empty()` report current cache size.
//!
//! ## Constraints
//! - Use `HashMap` internally for O(1) write and read by pubkey.
//! - Flush sort order: primary key is `slot` (ascending), secondary key is
//!   `pubkey` (ascending, lexicographic byte comparison).
//! - Writing the same pubkey again always replaces the previous entry entirely
//!   (including slot, lamports, and data).
//!
//! ## Hints
//! - A `HashMap<[u8; 32], CachedAccount>` gives O(1) upsert and lookup.
//! - For flush, drain the map into a Vec and sort with a custom comparator.
//! - `[u8; 32]` implements `Ord`, so you can compare pubkeys directly with `cmp`.

use std::collections::HashMap;

/// A cached account write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedAccount {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub data: Vec<u8>,
    pub slot: u64,
}

/// Write cache that buffers account updates and flushes in sorted batches.
pub struct WriteCache {
    // TODO: choose appropriate fields
    _placeholder: (),
}

impl WriteCache {
    /// Create a new cache that should be flushed when it reaches `capacity` entries.
    pub fn new(capacity: usize) -> Self {
        todo!("Initialize the cache with the given capacity threshold")
    }

    /// Write (insert or overwrite) a cached account.
    ///
    /// If an entry with the same pubkey already exists, it is fully replaced
    /// by the new entry.
    pub fn write(&mut self, account: CachedAccount) {
        todo!("Insert or replace the entry keyed by account.pubkey")
    }

    /// Read a cached account by pubkey.
    pub fn read(&self, pubkey: &[u8; 32]) -> Option<&CachedAccount> {
        todo!("Look up the pubkey in the internal map")
    }

    /// Drain the cache and return all entries sorted by (slot asc, pubkey asc).
    ///
    /// The cache is empty after this call.
    pub fn flush(&mut self) -> Vec<CachedAccount> {
        todo!(
            "Drain all entries from the map into a Vec, \
             sort by slot ascending then pubkey ascending, return."
        )
    }

    /// Returns true if the cache has reached its capacity threshold.
    pub fn should_flush(&self) -> bool {
        todo!("Check if current len >= capacity")
    }

    /// Number of entries currently in the cache.
    pub fn len(&self) -> usize {
        todo!("Return the number of entries in the map")
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        todo!("Return true if len is 0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cached(pk: u8, lamports: u64, slot: u64) -> CachedAccount {
        CachedAccount {
            pubkey: [pk; 32],
            lamports,
            data: vec![pk; 16],
            slot,
        }
    }

    #[test]
    fn test_write_and_read() {
        let mut cache = WriteCache::new(100);
        cache.write(cached(1, 500, 10));
        let acct = cache.read(&[1u8; 32]).expect("should exist");
        assert_eq!(acct.lamports, 500);
        assert_eq!(acct.slot, 10);
    }

    #[test]
    fn test_read_missing_returns_none() {
        let cache = WriteCache::new(100);
        assert!(cache.read(&[99u8; 32]).is_none());
    }

    #[test]
    fn test_overwrite_keeps_latest() {
        let mut cache = WriteCache::new(100);
        cache.write(cached(1, 100, 10));
        cache.write(cached(1, 999, 20));
        assert_eq!(cache.len(), 1);
        let acct = cache.read(&[1u8; 32]).unwrap();
        assert_eq!(acct.lamports, 999);
        assert_eq!(acct.slot, 20);
    }

    #[test]
    fn test_flush_returns_sorted() {
        let mut cache = WriteCache::new(100);
        cache.write(cached(3, 100, 20));
        cache.write(cached(1, 200, 10));
        cache.write(cached(2, 300, 10));
        cache.write(cached(4, 400, 20));

        let flushed = cache.flush();
        assert_eq!(flushed.len(), 4);
        // Slot 10 first, sorted by pubkey
        assert_eq!(flushed[0].pubkey, [1u8; 32]);
        assert_eq!(flushed[0].slot, 10);
        assert_eq!(flushed[1].pubkey, [2u8; 32]);
        assert_eq!(flushed[1].slot, 10);
        // Slot 20 next, sorted by pubkey
        assert_eq!(flushed[2].pubkey, [3u8; 32]);
        assert_eq!(flushed[2].slot, 20);
        assert_eq!(flushed[3].pubkey, [4u8; 32]);
        assert_eq!(flushed[3].slot, 20);
    }

    #[test]
    fn test_flush_empties_cache() {
        let mut cache = WriteCache::new(100);
        cache.write(cached(1, 100, 10));
        cache.write(cached(2, 200, 20));
        let _ = cache.flush();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_should_flush_capacity_trigger() {
        let mut cache = WriteCache::new(3);
        assert!(!cache.should_flush());
        cache.write(cached(1, 100, 10));
        cache.write(cached(2, 200, 10));
        assert!(!cache.should_flush());
        cache.write(cached(3, 300, 10));
        assert!(cache.should_flush());
    }

    #[test]
    fn test_overwrite_does_not_inflate_count() {
        let mut cache = WriteCache::new(3);
        cache.write(cached(1, 100, 10));
        cache.write(cached(1, 200, 20));
        cache.write(cached(1, 300, 30));
        // Only 1 unique pubkey, should NOT trigger flush at capacity 3
        assert_eq!(cache.len(), 1);
        assert!(!cache.should_flush());
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut cache = WriteCache::new(10);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        cache.write(cached(1, 100, 10));
        assert!(!cache.is_empty());
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_flush_empty_cache() {
        let mut cache = WriteCache::new(10);
        let flushed = cache.flush();
        assert!(flushed.is_empty());
    }

    #[test]
    fn test_write_after_flush() {
        let mut cache = WriteCache::new(10);
        cache.write(cached(1, 100, 10));
        let _ = cache.flush();
        cache.write(cached(2, 200, 20));
        assert_eq!(cache.len(), 1);
        let acct = cache.read(&[2u8; 32]).unwrap();
        assert_eq!(acct.lamports, 200);
    }
}
