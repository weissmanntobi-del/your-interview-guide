//! # Challenge 2.1: LRU Cache
//!
//! ## Problem
//! Implement a Least Recently Used (LRU) cache with O(1) get and put operations.
//!
//! ## Why This Matters
//! LRU caches are used everywhere in blockchain clients: caching account state,
//! recent block headers, contract bytecode, and trie nodes. reth uses LRU caches
//! for its state caching layer. The combination of HashMap + doubly-linked list
//! is a classic data structure interview question.
//!
//! ## Requirements
//! - `LruCache::new(capacity)` — create cache with max capacity
//! - `get(&key)` — return Some(&value) and mark as recently used, or None
//! - `put(key, value)` — insert/update. If at capacity, evict least recently used.
//! - Both operations must be O(1)

use std::collections::HashMap;

pub struct LruCache<K, V> {
    // TODO: implement
    // Hint: HashMap<K, NodeIndex> + a doubly-linked list
    // The list maintains access order: most recent at front, least at back
    _placeholder: std::marker::PhantomData<(K, V)>,
    _capacity: usize,
}

impl<K: Eq + std::hash::Hash + Clone, V> LruCache<K, V> {
    pub fn new(_capacity: usize) -> Self {
        todo!("Initialize empty cache with given capacity")
    }

    pub fn get(&mut self, _key: &K) -> Option<&V> {
        todo!("Look up key, move to front of list, return value ref")
    }

    pub fn put(&mut self, _key: K, _value: V) -> Option<V> {
        todo!("Insert at front. If exists, update and move to front. If full, evict back. Return evicted value if any.")
    }

    pub fn len(&self) -> usize {
        todo!("Return number of entries")
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        todo!("Return max capacity")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_put_get() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), Some(&2));
    }

    #[test]
    fn test_eviction() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3); // Should evict "a"
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_access_updates_recency() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.get(&"a"); // "a" is now most recent
        cache.put("c", 3); // Should evict "b" (least recent), not "a"
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("a", 10); // Update
        assert_eq!(cache.get(&"a"), Some(&10));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_capacity_one() {
        let mut cache = LruCache::new(1);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_miss_returns_none() {
        let mut cache: LruCache<&str, i32> = LruCache::new(2);
        assert_eq!(cache.get(&"nonexistent"), None);
    }

    #[test]
    fn test_len_and_empty() {
        let mut cache = LruCache::new(3);
        assert!(cache.is_empty());
        cache.put("a", 1);
        assert_eq!(cache.len(), 1);
        cache.put("b", 2);
        assert_eq!(cache.len(), 2);
    }
}
