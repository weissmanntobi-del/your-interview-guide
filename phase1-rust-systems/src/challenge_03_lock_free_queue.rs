//! # Challenge 1.3: Lock-Free Queue
//!
//! ## Problem
//! Implement a lock-free single-producer, single-consumer (SPSC) queue
//! using atomic operations. No mutexes allowed.
//!
//! ## Why This Matters
//! Lock-free data structures are used in blockchain clients for high-throughput
//! pipelines: transaction ingestion, block processing, event notification.
//! Agave's banking stage uses channel-like structures for passing transactions
//! between pipeline stages without lock contention.
//!
//! ## Requirements
//! - `Queue::new(capacity)` — bounded queue with fixed capacity
//! - `push(value)` — returns Ok(()) or Err(value) if full
//! - `pop()` — returns Some(T) or None if empty
//! - Must be safe for one producer thread and one consumer thread
//!
//! ## Constraints
//! - NO Mutex, RwLock, or other blocking primitives
//! - Use only AtomicUsize for synchronization
//! - Use correct memory ordering (Acquire/Release)

use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

pub struct SpscQueue<T> {
    // TODO: implement fields
    // You'll need:
    // - A buffer: Box<[UnsafeCell<Option<T>>]> or similar
    // - head: AtomicUsize (consumer reads from here)
    // - tail: AtomicUsize (producer writes here)
    // - capacity: usize
    _placeholder: std::marker::PhantomData<T>,
}

// SAFETY: Safe to send between threads because we use atomics for synchronization
unsafe impl<T: Send> Send for SpscQueue<T> {}
unsafe impl<T: Send> Sync for SpscQueue<T> {}

impl<T> SpscQueue<T> {
    /// Create a new bounded SPSC queue.
    pub fn new(_capacity: usize) -> Self {
        todo!("Allocate buffer and initialize atomic indices")
    }

    /// Push a value into the queue. Returns Err(value) if full.
    pub fn push(&self, _value: T) -> Result<(), T> {
        todo!("Write to tail position, advance tail with Release ordering")
    }

    /// Pop a value from the queue. Returns None if empty.
    pub fn pop(&self) -> Option<T> {
        todo!("Read from head position, advance head with Release ordering")
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        todo!("Compare head and tail with Acquire ordering")
    }

    /// Returns the number of items in the queue.
    pub fn len(&self) -> usize {
        todo!("Calculate distance between head and tail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_push_pop_single_thread() {
        let q = SpscQueue::new(4);
        assert!(q.is_empty());
        q.push(1).unwrap();
        q.push(2).unwrap();
        q.push(3).unwrap();
        assert_eq!(q.len(), 3);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn test_full_queue() {
        let q = SpscQueue::new(2);
        q.push(1).unwrap();
        q.push(2).unwrap();
        assert_eq!(q.push(3), Err(3)); // Queue is full
    }

    #[test]
    fn test_wrap_around() {
        let q = SpscQueue::new(2);
        q.push(1).unwrap();
        q.pop().unwrap();
        q.push(2).unwrap();
        q.push(3).unwrap();
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
    }

    #[test]
    fn test_producer_consumer_threads() {
        let q = std::sync::Arc::new(SpscQueue::new(64));
        let q_producer = q.clone();
        let q_consumer = q.clone();

        let producer = thread::spawn(move || {
            for i in 0u64..1000 {
                // Spin until push succeeds
                while q_producer.push(i).is_err() {
                    std::hint::spin_loop();
                }
            }
        });

        let consumer = thread::spawn(move || {
            let mut received = Vec::new();
            while received.len() < 1000 {
                if let Some(val) = q_consumer.pop() {
                    received.push(val);
                } else {
                    std::hint::spin_loop();
                }
            }
            received
        });

        producer.join().unwrap();
        let received = consumer.join().unwrap();

        // Verify all items received in order
        assert_eq!(received.len(), 1000);
        for (i, val) in received.iter().enumerate() {
            assert_eq!(*val, i as u64);
        }
    }

    #[test]
    fn test_empty_pop() {
        let q: SpscQueue<i32> = SpscQueue::new(4);
        assert_eq!(q.pop(), None);
        assert!(q.is_empty());
    }
}
