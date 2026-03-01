/// Challenge 05 - Debug Deadlock Patterns
///
/// This challenge presents three BUGGY async functions that demonstrate common
/// deadlock patterns. Students must:
/// 1. Understand why each one deadlocks.
/// 2. Implement fix_N() functions that accomplish the same goal without deadlocking.
/// 3. Build a DeadlockDetector that detects potential lock-ordering issues.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

// ============================================================
// BUGGY FUNCTIONS (already implemented - DO NOT modify)
// ============================================================

/// Deadlock 1: Two mutexes acquired in different order.
/// Thread A: lock(m1) then lock(m2)
/// Thread B: lock(m2) then lock(m1) => deadlock!
pub async fn deadlock_1(
    m1: Arc<Mutex<Vec<u32>>>,
    m2: Arc<Mutex<Vec<u32>>>,
) {
    let handle_a = {
        let m1 = m1.clone();
        let m2 = m2.clone();
        tokio::spawn(async move {
            let mut guard1 = m1.lock().await;
            // Simulate some work
            tokio::task::yield_now().await;
            let mut guard2 = m2.lock().await;
            guard1.push(1);
            guard2.push(2);
        })
    };

    let handle_b = {
        let m1 = m1.clone();
        let m2 = m2.clone();
        tokio::spawn(async move {
            let mut guard2 = m2.lock().await; // REVERSED ORDER!
            tokio::task::yield_now().await;
            let mut guard1 = m1.lock().await;
            guard2.push(3);
            guard1.push(4);
        })
    };

    let _ = handle_a.await;
    let _ = handle_b.await;
}

/// Deadlock 2: Lock held across an await point.
/// The lock is held while awaiting a channel receive, but the sender
/// needs the same lock to send.
pub async fn deadlock_2(data: Arc<Mutex<Vec<String>>>) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(1);

    let data_clone = data.clone();
    let producer = tokio::spawn(async move {
        let guard = data_clone.lock().await;
        // BUG: Holding lock while sending (which may block)
        tx.send(format!("items: {}", guard.len())).await.ok();
        // Lock held until end of scope
        drop(guard);
    });

    let consumer = tokio::spawn(async move {
        if let Some(msg) = rx.recv().await {
            let mut guard = data.lock().await; // May deadlock if producer holds lock
            guard.push(msg);
        }
    });

    let _ = producer.await;
    let _ = consumer.await;
}

/// Deadlock 3: Channel send blocks because receiver is dropped.
pub async fn deadlock_3() -> Vec<u32> {
    let (tx, rx) = tokio::sync::mpsc::channel::<u32>(1);

    let sender = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.ok(); // If receiver dropped, send returns Err but we ignore
        }
    });

    // BUG: Drop receiver before sender finishes - using a bounded channel of size 1,
    // sender blocks on second send because buffer is full and receiver is gone
    drop(rx);
    let _ = sender.await;
    vec![]
}

// ============================================================
// FIX FUNCTIONS (student implements)
// ============================================================

/// Fix for deadlock_1: acquire both mutexes in the SAME order.
pub async fn fix_1(
    m1: Arc<Mutex<Vec<u32>>>,
    m2: Arc<Mutex<Vec<u32>>>,
) {
    // TODO: Implement the same logic as deadlock_1, but acquire locks in consistent order.
    // Both tasks should lock m1 first, then m2.
    todo!("Implement fix_1")
}

/// Fix for deadlock_2: don't hold the lock across the await.
pub async fn fix_2(data: Arc<Mutex<Vec<String>>>) {
    // TODO: Implement the same logic but drop the lock before awaiting the channel.
    // 1. Lock data, read what you need, drop the lock
    // 2. Send the message
    // 3. Consumer receives and re-locks to push
    todo!("Implement fix_2")
}

/// Fix for deadlock_3: ensure receiver is consumed before dropping.
pub async fn fix_3() -> Vec<u32> {
    // TODO: Actually receive from the channel instead of dropping it.
    // Spawn sender, then collect all values from receiver.
    todo!("Implement fix_3")
}

// ============================================================
// DEADLOCK DETECTOR
// ============================================================

/// A simple deadlock detector based on lock ordering.
/// Tracks which thread holds which locks and detects cycles.
pub struct DeadlockDetector {
    /// thread_id -> set of currently held lock_ids
    held: HashMap<u64, Vec<u64>>,
    /// Observed lock ordering: if lock A was acquired before lock B, record (A, B)
    ordering: HashSet<(u64, u64)>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        DeadlockDetector {
            held: HashMap::new(),
            ordering: HashSet::new(),
        }
    }

    /// Record that `thread_id` acquired `lock_id`.
    pub fn acquire(&mut self, lock_id: u64, thread_id: u64) {
        // TODO: Implement acquire
        // 1. For each lock currently held by this thread, record ordering (held -> lock_id)
        // 2. Add lock_id to the thread's held set
        todo!("Implement acquire")
    }

    /// Record that `thread_id` released `lock_id`.
    pub fn release(&mut self, lock_id: u64, thread_id: u64) {
        // TODO: Remove lock_id from thread's held set
        todo!("Implement release")
    }

    /// Detect if there's a potential deadlock (cycle in lock ordering).
    /// Returns true if we've seen both (A, B) and (B, A) for any A, B.
    pub fn detect_potential_deadlock(&self) -> bool {
        // TODO: Check for contradictory orderings
        // If (a, b) is in ordering AND (b, a) is also in ordering => potential deadlock
        todo!("Implement detect_potential_deadlock")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_detect_ordering_deadlock() {
        let mut detector = DeadlockDetector::new();
        // Thread 1: acquires lock 1, then lock 2
        detector.acquire(1, 1);
        detector.acquire(2, 1);
        detector.release(2, 1);
        detector.release(1, 1);

        // Thread 2: acquires lock 2, then lock 1 (reversed!)
        detector.acquire(2, 2);
        detector.acquire(1, 2);
        detector.release(1, 2);
        detector.release(2, 2);

        assert!(
            detector.detect_potential_deadlock(),
            "Should detect contradictory lock ordering"
        );
    }

    #[test]
    fn test_no_false_positive() {
        let mut detector = DeadlockDetector::new();
        // Both threads acquire in same order: 1 then 2
        detector.acquire(1, 1);
        detector.acquire(2, 1);
        detector.release(2, 1);
        detector.release(1, 1);

        detector.acquire(1, 2);
        detector.acquire(2, 2);
        detector.release(2, 2);
        detector.release(1, 2);

        assert!(
            !detector.detect_potential_deadlock(),
            "Consistent ordering should not trigger deadlock detection"
        );
    }

    #[tokio::test]
    async fn test_fix_1_completes() {
        let m1 = Arc::new(Mutex::new(Vec::new()));
        let m2 = Arc::new(Mutex::new(Vec::new()));
        let result = tokio::time::timeout(Duration::from_secs(2), fix_1(m1.clone(), m2.clone())).await;
        assert!(result.is_ok(), "fix_1 should complete without deadlock");
        let v1 = m1.lock().await;
        let v2 = m2.lock().await;
        assert!(!v1.is_empty() || !v2.is_empty(), "Some values should have been pushed");
    }

    #[tokio::test]
    async fn test_fix_2_completes() {
        let data = Arc::new(Mutex::new(vec!["initial".to_string()]));
        let result = tokio::time::timeout(Duration::from_secs(2), fix_2(data.clone())).await;
        assert!(result.is_ok(), "fix_2 should complete without deadlock");
        let guard = data.lock().await;
        assert!(guard.len() > 1, "Consumer should have pushed a message");
    }

    #[tokio::test]
    async fn test_fix_3_completes() {
        let result = tokio::time::timeout(Duration::from_secs(2), fix_3()).await;
        assert!(result.is_ok(), "fix_3 should complete without deadlock");
        let values = result.unwrap();
        assert_eq!(values, (0..10).collect::<Vec<u32>>());
    }
}
