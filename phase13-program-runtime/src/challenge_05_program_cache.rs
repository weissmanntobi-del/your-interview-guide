/// # Challenge 05: LRU Program Cache for Compiled Programs
///
/// The Solana runtime caches compiled programs in memory to avoid recompiling them
/// on every transaction. This challenge implements a bounded LRU cache with:
///
/// - **Entry limit**: Maximum number of programs stored.
/// - **Byte limit**: Maximum total `compiled_size` across all entries.
/// - **LRU eviction**: When the cache is full, the least recently used entry is
///   evicted. "Recently used" is determined by `last_used_slot`; ties are broken by
///   `use_count` (lower count is evicted first).
/// - **Slot-based invalidation**: `invalidate_slot(slot)` removes all programs
///   deployed at a given slot (useful during slot rollbacks / forks).
/// - **Statistics tracking**: Hits, misses, evictions, and total cached bytes.

/// A compiled program stored in the cache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledProgram {
    /// The program's public key (account address).
    pub program_id: [u8; 32],
    /// The compiled bytecode / native code.
    pub code: Vec<u8>,
    /// Size in bytes of the compiled representation (used for byte budget).
    pub compiled_size: usize,
    /// The slot at which this program was deployed.
    pub slot_deployed: u64,
    /// The slot at which this program was last accessed via `get`.
    pub last_used_slot: u64,
    /// How many times this program has been retrieved via `get`.
    pub use_count: u64,
}

/// Cumulative statistics about cache behaviour.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CacheStats {
    /// Number of `get` calls that found the requested program.
    pub hits: u64,
    /// Number of `get` calls that did not find the requested program.
    pub misses: u64,
    /// Number of entries evicted to make room for new insertions.
    pub evictions: u64,
    /// Current total of `compiled_size` across all cached programs.
    pub total_bytes: usize,
}

/// A bounded LRU cache for compiled Solana programs.
pub struct ProgramCache {
    max_entries: usize,
    max_bytes: usize,
    current_slot: u64,
    entries: Vec<CompiledProgram>,
    stats: CacheStats,
}

impl ProgramCache {
    /// Create a new program cache with the given capacity limits.
    pub fn new(max_entries: usize, max_bytes: usize) -> Self {
        todo!()
    }

    /// Set the current slot. Subsequent `get` calls will update `last_used_slot`
    /// to this value.
    pub fn set_current_slot(&mut self, slot: u64) {
        todo!()
    }

    /// Retrieve a program by its ID. On hit, updates `last_used_slot` and
    /// increments `use_count`. Returns `None` on miss.
    pub fn get(&mut self, program_id: &[u8; 32]) -> Option<&CompiledProgram> {
        todo!()
    }

    /// Insert a compiled program into the cache. If the cache is at capacity
    /// (entries or bytes), evict the LRU entry first (repeating as needed for
    /// byte limit).
    ///
    /// If a program with the same `program_id` already exists, it is replaced
    /// (the old entry's bytes are freed before the new entry is inserted).
    pub fn insert(&mut self, program: CompiledProgram) {
        todo!()
    }

    /// Remove a specific program from the cache. Returns `true` if the program
    /// was present.
    pub fn invalidate(&mut self, program_id: &[u8; 32]) -> bool {
        todo!()
    }

    /// Remove all programs that were deployed at the given slot. Returns the
    /// number of programs removed.
    pub fn invalidate_slot(&mut self, slot: u64) -> usize {
        todo!()
    }

    /// Return a snapshot of the current cache statistics.
    pub fn stats(&self) -> &CacheStats {
        todo!()
    }

    /// Return the number of programs currently cached.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Return `true` if the cache contains no programs.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Find the index of the LRU entry. LRU is determined by `last_used_slot`
    /// (lowest first); ties broken by `use_count` (lowest first).
    fn find_lru_index(&self) -> Option<usize> {
        todo!()
    }

    /// Evict the LRU entry, updating stats accordingly.
    fn evict_one(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(id: u8) -> [u8; 32] {
        [id; 32]
    }

    fn make_program(id: u8, size: usize, slot: u64) -> CompiledProgram {
        CompiledProgram {
            program_id: pid(id),
            code: vec![id; size],
            compiled_size: size,
            slot_deployed: slot,
            last_used_slot: slot,
            use_count: 0,
        }
    }

    #[test]
    fn test_insert_and_retrieve() {
        let mut cache = ProgramCache::new(10, 10_000);
        cache.set_current_slot(5);
        cache.insert(make_program(1, 100, 1));

        let prog = cache.get(&pid(1)).unwrap();
        assert_eq!(prog.program_id, pid(1));
        assert_eq!(prog.last_used_slot, 5);
        assert_eq!(prog.use_count, 1);
        assert_eq!(cache.stats().hits, 1);
    }

    #[test]
    fn test_cache_miss() {
        let mut cache = ProgramCache::new(10, 10_000);
        assert!(cache.get(&pid(99)).is_none());
        assert_eq!(cache.stats().misses, 1);
    }

    #[test]
    fn test_lru_eviction_by_slot() {
        // Cache holds 2 entries. Insert 3; the one with the lowest last_used_slot
        // should be evicted.
        let mut cache = ProgramCache::new(2, 100_000);

        cache.set_current_slot(1);
        cache.insert(make_program(1, 100, 1));

        cache.set_current_slot(2);
        cache.insert(make_program(2, 100, 2));

        // Access program 1 at slot 3 so its last_used_slot becomes 3
        cache.set_current_slot(3);
        cache.get(&pid(1));

        // Insert program 3; program 2 (last_used_slot=2) should be evicted
        cache.insert(make_program(3, 100, 3));

        assert!(cache.get(&pid(1)).is_some()); // still cached
        assert_eq!(cache.stats().evictions, 1);
        // Program 2 was evicted, so this is a miss
        let miss_before = cache.stats().misses;
        assert!(cache.get(&pid(2)).is_none());
        assert_eq!(cache.stats().misses, miss_before + 1);
    }

    #[test]
    fn test_invalidation() {
        let mut cache = ProgramCache::new(10, 10_000);
        cache.insert(make_program(1, 100, 1));
        assert!(cache.invalidate(&pid(1)));
        assert!(cache.is_empty());
        assert!(!cache.invalidate(&pid(1))); // already removed
    }

    #[test]
    fn test_slot_rollback_invalidation() {
        let mut cache = ProgramCache::new(10, 10_000);
        cache.insert(make_program(1, 100, 5));
        cache.insert(make_program(2, 200, 5));
        cache.insert(make_program(3, 300, 6));

        let removed = cache.invalidate_slot(5);
        assert_eq!(removed, 2);
        assert_eq!(cache.len(), 1);
        assert!(cache.get(&pid(3)).is_some());
    }

    #[test]
    fn test_byte_limit_eviction() {
        // max_bytes = 250, insert two 100-byte programs, then a 100-byte program
        // that pushes total over 250.
        let mut cache = ProgramCache::new(10, 250);
        cache.set_current_slot(1);
        cache.insert(make_program(1, 100, 1));
        cache.set_current_slot(2);
        cache.insert(make_program(2, 100, 2));
        cache.set_current_slot(3);
        // This would make total_bytes = 300 > 250, so LRU must be evicted
        cache.insert(make_program(3, 100, 3));

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.stats().evictions, 1);
        assert!(cache.stats().total_bytes <= 250);
    }

    #[test]
    fn test_stats_tracking() {
        let mut cache = ProgramCache::new(10, 10_000);
        cache.insert(make_program(1, 100, 1));

        cache.get(&pid(1));
        cache.get(&pid(1));
        cache.get(&pid(99));

        let s = cache.stats();
        assert_eq!(s.hits, 2);
        assert_eq!(s.misses, 1);
        assert_eq!(s.total_bytes, 100);
    }

    #[test]
    fn test_frequently_used_survives_eviction() {
        // Cache with 2 slots. Program 1 is used many times, program 2 is used once.
        // When program 3 is inserted, program 2 should be evicted (lower use_count
        // as tiebreaker when last_used_slot is equal).
        let mut cache = ProgramCache::new(2, 100_000);
        cache.set_current_slot(1);
        cache.insert(make_program(1, 100, 1));
        cache.insert(make_program(2, 100, 1));

        // Access both at the same slot so last_used_slot is equal
        cache.set_current_slot(5);
        cache.get(&pid(1)); // use_count = 1
        cache.get(&pid(1)); // use_count = 2
        cache.get(&pid(1)); // use_count = 3
        cache.get(&pid(2)); // use_count = 1

        cache.insert(make_program(3, 100, 5));

        // Program 2 should have been evicted (lower use_count)
        assert!(cache.get(&pid(1)).is_some());
        assert!(cache.get(&pid(3)).is_some());
    }

    #[test]
    fn test_empty_cache() {
        let cache = ProgramCache::new(10, 10_000);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.stats().total_bytes, 0);
    }
}
