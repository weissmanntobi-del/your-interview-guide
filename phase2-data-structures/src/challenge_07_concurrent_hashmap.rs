//! # Challenge 2.7: Concurrent HashMap — Sharded locking. Time: 75 min | Hard
use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::hash::Hash;
pub struct ConcurrentMap<K, V> { _p: std::marker::PhantomData<(K, V)> }
impl<K: Eq + Hash + Clone, V: Clone> ConcurrentMap<K, V> {
    pub fn new(_num_shards: usize) -> Self { todo!() }
    pub fn insert(&self, _key: K, _value: V) { todo!() }
    pub fn get(&self, _key: &K) -> Option<V> { todo!() }
    pub fn remove(&self, _key: &K) -> Option<V> { todo!() }
}
#[cfg(test)] mod tests { use super::*; use std::thread;
    #[test] fn test_concurrent_access() {
        let map = Arc::new(ConcurrentMap::new(16));
        let handles: Vec<_> = (0..100).map(|i| { let m = map.clone(); thread::spawn(move || { m.insert(i, i * 10); }) }).collect();
        for h in handles { h.join().unwrap(); }
        for i in 0..100 { assert_eq!(map.get(&i), Some(i * 10)); }
    }
}
