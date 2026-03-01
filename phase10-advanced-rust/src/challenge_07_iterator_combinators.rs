//! # Challenge 10.7: Custom Iterator Combinators
//!
//! ## Problem
//! Build custom iterator combinators from scratch: `.batch(n)`, `.dedup_by_key()`,
//! `.take_while_inclusive()`, and `.inspect_nth()` as an extension trait on Iterator.
//!
//! ## Why This Matters
//! Custom iterators show up in Solana's transaction processing pipelines and log
//! parsing. Understanding how to build iterator adapters from scratch demonstrates
//! deep knowledge of Rust's iterator protocol and zero-cost abstractions.
//!
//! ## Requirements
//! - `BatchIter`: collects items into Vec<T> of size n (last batch may be smaller)
//! - `DedupByKeyIter`: skips consecutive items with same key (like Unix `uniq`)
//! - `TakeWhileInclusiveIter`: like take_while but includes the first failing item
//! - `InspectNthIter`: calls a closure on every nth item
//! - All exposed via an `IteratorExt` trait that adds methods to any Iterator
//!
//! ## Constraints
//! - Implement Iterator trait for each combinator struct
//! - Must work with any Iterator, not just specific types
//! - Combinators must be chainable (return impl Iterator)
//!
//! ## Hints
//! - Each combinator is a struct that holds the inner iterator + state
//! - BatchIter needs a `size: usize` and collects via `self.iter.by_ref().take(n)`
//! - DedupByKeyIter holds `last_key: Option<K>` to compare
//! - TakeWhileInclusive needs a `done: bool` flag

use std::cell::RefCell;

/// Extension trait that adds custom combinators to any Iterator.
pub trait IteratorExt: Iterator + Sized {
    /// Collect items into batches of `n`. The last batch may be smaller.
    fn batch(self, n: usize) -> BatchIter<Self> {
        todo!("Create BatchIter wrapping self")
    }

    /// Skip consecutive items that produce the same key.
    fn dedup_by_key<K, F>(self, key_fn: F) -> DedupByKeyIter<Self, K, F>
    where
        K: PartialEq,
        F: FnMut(&Self::Item) -> K,
    {
        todo!("Create DedupByKeyIter wrapping self")
    }

    /// Like take_while, but includes the first item that fails the predicate.
    fn take_while_inclusive<F>(self, predicate: F) -> TakeWhileInclusiveIter<Self, F>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        todo!("Create TakeWhileInclusiveIter wrapping self")
    }

    /// Call a closure on every nth item (1-indexed: nth=1 means every item).
    fn inspect_nth<F>(self, n: usize, f: F) -> InspectNthIter<Self, F>
    where
        F: FnMut(&Self::Item),
    {
        todo!("Create InspectNthIter wrapping self")
    }
}

// Blanket implementation for all iterators
impl<I: Iterator> IteratorExt for I {}

/// Batches items into Vec<T> of size n.
pub struct BatchIter<I: Iterator> {
    // TODO: iter: I, size: usize
    _placeholder: std::marker::PhantomData<I>,
}

impl<I: Iterator> Iterator for BatchIter<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Collect up to self.size items. Return None if no items remain.")
    }
}

/// Deduplicates consecutive items by key.
pub struct DedupByKeyIter<I: Iterator, K, F> {
    // TODO: iter: I, key_fn: F, last_key: Option<K>
    _placeholder: std::marker::PhantomData<(I, K, F)>,
}

impl<I, K, F> Iterator for DedupByKeyIter<I, K, F>
where
    I: Iterator,
    K: PartialEq,
    F: FnMut(&I::Item) -> K,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Skip items whose key matches the previous item's key")
    }
}

/// Like take_while but includes the boundary item.
pub struct TakeWhileInclusiveIter<I: Iterator, F> {
    // TODO: iter: I, predicate: F, done: bool
    _placeholder: std::marker::PhantomData<(I, F)>,
}

impl<I, F> Iterator for TakeWhileInclusiveIter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Return items while predicate is true. When predicate fails, return that item too, then stop.")
    }
}

/// Calls a closure on every nth item.
pub struct InspectNthIter<I: Iterator, F> {
    // TODO: iter: I, n: usize, count: usize, f: F
    _placeholder: std::marker::PhantomData<(I, F)>,
}

impl<I, F> Iterator for InspectNthIter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item),
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Get next item, increment count, call f if count % n == 0, return item")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_exact_multiple() {
        let batches: Vec<Vec<i32>> = (1..=6).batch(3).collect();
        assert_eq!(batches, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_batch_with_remainder() {
        let batches: Vec<Vec<i32>> = (1..=7).batch(3).collect();
        assert_eq!(batches, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn test_batch_empty() {
        let batches: Vec<Vec<i32>> = std::iter::empty::<i32>().batch(3).collect();
        assert!(batches.is_empty());
    }

    #[test]
    fn test_dedup_by_key() {
        let data = vec![1, 1, 2, 2, 2, 3, 1, 1];
        let result: Vec<i32> = data.into_iter().dedup_by_key(|x| *x).collect();
        assert_eq!(result, vec![1, 2, 3, 1]); // like Unix uniq
    }

    #[test]
    fn test_dedup_by_key_no_dupes() {
        let data = vec![1, 2, 3, 4];
        let result: Vec<i32> = data.into_iter().dedup_by_key(|x| *x).collect();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_take_while_inclusive() {
        let data = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = data.into_iter().take_while_inclusive(|x| *x < 3).collect();
        assert_eq!(result, vec![1, 2, 3]); // includes 3 (the boundary)
    }

    #[test]
    fn test_take_while_inclusive_all_pass() {
        let data = vec![1, 2, 3];
        let result: Vec<i32> = data.into_iter().take_while_inclusive(|x| *x < 10).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_inspect_nth() {
        let inspected = RefCell::new(Vec::new());
        let _: Vec<i32> = (1..=10)
            .inspect_nth(3, |x| inspected.borrow_mut().push(*x))
            .collect();
        assert_eq!(*inspected.borrow(), vec![3, 6, 9]);
    }

    #[test]
    fn test_chaining_combinators() {
        let data = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let result: Vec<Vec<i32>> = data
            .into_iter()
            .dedup_by_key(|x| *x) // [1, 2, 3, 4, 5]
            .take_while_inclusive(|x| *x < 4) // [1, 2, 3, 4]
            .batch(2) // [[1, 2], [3, 4]]
            .collect();
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }
}
