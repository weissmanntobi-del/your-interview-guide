//! # Challenge 1.1: Lifetime Arena Allocator
//!
//! ## Problem
//! Implement a simple arena (bump) allocator that allocates objects from a
//! pre-allocated buffer. All allocations share the arena's lifetime and are
//! freed together when the arena is dropped.
//!
//! ## Why This Matters
//! Arena allocators are used in blockchain clients (reth, Agave) to batch-allocate
//! short-lived objects during block/transaction processing. They avoid per-object
//! heap allocation overhead and improve cache locality.
//!
//! ## Requirements
//! - `Arena::new(capacity)` — create arena with given byte capacity
//! - `arena.alloc(value)` — allocate a single value, return &T with arena lifetime
//! - `arena.alloc_slice(slice)` — allocate a copy of a slice, return &[T]
//! - `arena.reset()` — reset allocation pointer (reuse memory without dealloc)
//! - `arena.bytes_used()` — return bytes currently allocated
//!
//! ## Constraints
//! - You MUST use unsafe for the core allocation logic
//! - Allocated references must be valid for the arena's lifetime
//! - Handle alignment correctly (align allocations to align_of::<T>())
//! - Return None if arena is out of space
//!
//! ## Hints
//! - Use a Vec<u8> as the backing buffer
//! - Track current offset with a Cell<usize> (interior mutability without &mut)
//! - Use std::alloc::Layout for alignment calculations
//! - The key insight: &self methods that return &T tied to the arena lifetime

use std::cell::Cell;

pub struct Arena {
    // TODO: implement fields
    // You'll need:
    // - A backing buffer (Vec<u8> or Box<[u8]>)
    // - A current offset tracker (Cell<usize> for interior mutability)
    _placeholder: (),
}

impl Arena {
    /// Create a new arena with the given byte capacity.
    pub fn new(_capacity: usize) -> Self {
        todo!("Create arena with pre-allocated buffer")
    }

    /// Allocate a single value in the arena.
    /// Returns None if there isn't enough space.
    pub fn alloc<T: Copy>(&self, _value: T) -> Option<&T> {
        todo!("Allocate value with proper alignment, return reference tied to arena lifetime")
    }

    /// Allocate a copy of a slice in the arena.
    /// Returns None if there isn't enough space.
    pub fn alloc_slice<T: Copy>(&self, _slice: &[T]) -> Option<&[T]> {
        todo!("Allocate slice copy with proper alignment")
    }

    /// Reset the arena, allowing all memory to be reused.
    /// WARNING: This invalidates all previously returned references.
    /// In a real implementation, you'd use generational indices or
    /// lifetime tricks to prevent use-after-reset.
    pub fn reset(&self) {
        todo!("Reset offset to 0")
    }

    /// Returns the number of bytes currently allocated.
    pub fn bytes_used(&self) -> usize {
        todo!("Return current offset")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc_single_u64() {
        let arena = Arena::new(1024);
        let val = arena.alloc(42u64).expect("should allocate");
        assert_eq!(*val, 42u64);
    }

    #[test]
    fn test_alloc_multiple_values() {
        let arena = Arena::new(1024);
        let a = arena.alloc(1u32).unwrap();
        let b = arena.alloc(2u32).unwrap();
        let c = arena.alloc(3u32).unwrap();
        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
        assert_eq!(*c, 3);
    }

    #[test]
    fn test_alloc_different_types() {
        let arena = Arena::new(1024);
        let a = arena.alloc(42u8).unwrap();
        let b = arena.alloc(1000u64).unwrap();
        let c = arena.alloc(true).unwrap();
        assert_eq!(*a, 42u8);
        assert_eq!(*b, 1000u64);
        assert_eq!(*c, true);
    }

    #[test]
    fn test_alloc_slice() {
        let arena = Arena::new(1024);
        let data = [1u32, 2, 3, 4, 5];
        let slice = arena.alloc_slice(&data).unwrap();
        assert_eq!(slice, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_out_of_space() {
        let arena = Arena::new(8); // Only 8 bytes
        let _ = arena.alloc(1u64).unwrap(); // Uses all 8 bytes
        assert!(arena.alloc(2u64).is_none()); // Should fail
    }

    #[test]
    fn test_bytes_used() {
        let arena = Arena::new(1024);
        assert_eq!(arena.bytes_used(), 0);
        arena.alloc(42u64).unwrap();
        assert!(arena.bytes_used() >= 8); // At least 8 bytes for u64
    }

    #[test]
    fn test_reset() {
        let arena = Arena::new(64);
        arena.alloc(1u64).unwrap();
        arena.alloc(2u64).unwrap();
        assert!(arena.bytes_used() > 0);
        arena.reset();
        assert_eq!(arena.bytes_used(), 0);
        // Should be able to allocate again after reset
        let val = arena.alloc(99u64).unwrap();
        assert_eq!(*val, 99);
    }

    #[test]
    fn test_alignment() {
        let arena = Arena::new(1024);
        // Allocate a u8 first, then a u64
        // The u64 should be properly aligned despite the u8 before it
        let _ = arena.alloc(1u8).unwrap();
        let b = arena.alloc(42u64).unwrap();
        let ptr = b as *const u64 as usize;
        assert_eq!(ptr % std::mem::align_of::<u64>(), 0, "u64 must be 8-byte aligned");
    }

    #[test]
    fn test_many_small_allocations() {
        let arena = Arena::new(4096);
        for i in 0u32..100 {
            let val = arena.alloc(i).expect("should have space");
            assert_eq!(*val, i);
        }
    }

    #[test]
    fn test_alloc_empty_slice() {
        let arena = Arena::new(1024);
        let empty: &[u64] = &[];
        let slice = arena.alloc_slice(empty).unwrap();
        assert!(slice.is_empty());
    }
}
