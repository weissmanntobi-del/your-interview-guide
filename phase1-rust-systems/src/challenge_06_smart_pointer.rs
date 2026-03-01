//! # Challenge 1.6: Custom Smart Pointer
//!
//! ## Problem
//! Implement a reference-counted smart pointer (simplified Arc) with:
//! - Shared ownership via Clone
//! - Automatic cleanup via Drop
//! - Deref to access inner value
//! - Thread-safe reference counting using AtomicUsize
//!
//! ## Why This Matters
//! Understanding how Arc works internally is essential for concurrent Rust.
//! Blockchain clients use Arc extensively for shared state between threads.

use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct SharedPtr<T> {
    // TODO: implement fields
    // You'll need a raw pointer to an allocation containing:
    // - The value T
    // - An atomic reference count
    _placeholder: std::marker::PhantomData<T>,
}

impl<T> SharedPtr<T> {
    pub fn new(_value: T) -> Self {
        todo!("Allocate inner struct with value and refcount=1")
    }

    pub fn ref_count(&self) -> usize {
        todo!("Return current reference count")
    }
}

impl<T> Clone for SharedPtr<T> {
    fn clone(&self) -> Self {
        todo!("Increment refcount, return new SharedPtr to same allocation")
    }
}

impl<T> Deref for SharedPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        todo!("Return reference to inner value")
    }
}

impl<T> Drop for SharedPtr<T> {
    fn drop(&mut self) {
        todo!("Decrement refcount. If it reaches 0, deallocate.")
    }
}

unsafe impl<T: Send + Sync> Send for SharedPtr<T> {}
unsafe impl<T: Send + Sync> Sync for SharedPtr<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_basic_usage() {
        let ptr = SharedPtr::new(42);
        assert_eq!(*ptr, 42);
        assert_eq!(ptr.ref_count(), 1);
    }

    #[test]
    fn test_clone_increments_refcount() {
        let ptr1 = SharedPtr::new(42);
        let ptr2 = ptr1.clone();
        assert_eq!(ptr1.ref_count(), 2);
        assert_eq!(*ptr1, *ptr2);
        drop(ptr2);
        assert_eq!(ptr1.ref_count(), 1);
    }

    #[test]
    fn test_drop_deallocates() {
        let ptr = SharedPtr::new(String::from("hello"));
        let ptr2 = ptr.clone();
        drop(ptr);
        assert_eq!(&*ptr2, "hello"); // Still accessible
        assert_eq!(ptr2.ref_count(), 1);
        // ptr2 drops here, deallocating the String
    }

    #[test]
    fn test_thread_safety() {
        let ptr = SharedPtr::new(42);
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let p = ptr.clone();
                thread::spawn(move || {
                    assert_eq!(*p, 42);
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(ptr.ref_count(), 1);
    }
}
