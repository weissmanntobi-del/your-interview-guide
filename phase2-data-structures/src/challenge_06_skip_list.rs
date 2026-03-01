//! # Challenge 2.6: Skip List — O(log n) ordered map. Time: 60 min | Hard
pub struct SkipList<K, V> { _p: std::marker::PhantomData<(K, V)> }
impl<K: Ord, V> SkipList<K, V> {
    pub fn new() -> Self { todo!() }
    pub fn insert(&mut self, _key: K, _value: V) { todo!() }
    pub fn get(&self, _key: &K) -> Option<&V> { todo!() }
    pub fn remove(&mut self, _key: &K) -> Option<V> { todo!() }
    pub fn len(&self) -> usize { todo!() }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_insert_get() { let mut sl = SkipList::new(); sl.insert(3, "c"); sl.insert(1, "a"); assert_eq!(sl.get(&1), Some(&"a")); assert_eq!(sl.get(&3), Some(&"c")); assert_eq!(sl.get(&2), None); }
}
