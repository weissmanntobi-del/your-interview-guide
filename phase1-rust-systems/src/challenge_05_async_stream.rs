//! # Challenge 1.5: Async Stream Processor
//!
//! ## Problem
//! Implement an async stream processor that reads items from a channel,
//! processes them with bounded concurrency, and handles backpressure.
//!
//! ## Why This Matters
//! Blockchain data pipelines (Geyser plugins, indexers, RPC servers) process
//! streams of blocks, transactions, and account updates. Backpressure handling
//! ensures slow consumers don't cause unbounded memory growth.
//!
//! ## Requirements
//! - `StreamProcessor::new(concurrency_limit)` — create processor
//! - `process(stream, handler)` — process items from stream with bounded concurrency
//! - Backpressure: if handler is slow, naturally slow down consumption
//! - Cancellation: if the processor is dropped, in-flight tasks should be cancelled

use tokio::sync::mpsc;

pub struct StreamProcessor {
    // TODO: implement fields
    _concurrency: usize,
}

impl StreamProcessor {
    pub fn new(_concurrency_limit: usize) -> Self {
        todo!("Store concurrency limit")
    }

    /// Process items from a receiver with bounded concurrency.
    /// `handler` is called for each item, and at most `concurrency_limit`
    /// handlers run simultaneously.
    pub async fn process<T, F, Fut>(
        &self,
        _rx: mpsc::Receiver<T>,
        _handler: F,
    ) where
        T: Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        todo!("Consume from rx, spawn up to concurrency_limit concurrent handlers")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_processes_all_items() {
        let (tx, rx) = mpsc::channel(100);
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();

        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
        drop(tx); // Close channel

        let processor = StreamProcessor::new(4);
        processor
            .process(rx, move |_item| {
                let c = count_clone.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                }
            })
            .await;

        assert_eq!(count.load(Ordering::SeqCst), 10);
    }

    #[tokio::test]
    async fn test_bounded_concurrency() {
        let (tx, rx) = mpsc::channel(100);
        let active = Arc::new(AtomicUsize::new(0));
        let max_active = Arc::new(AtomicUsize::new(0));

        for i in 0..20 {
            tx.send(i).await.unwrap();
        }
        drop(tx);

        let active_clone = active.clone();
        let max_clone = max_active.clone();

        let processor = StreamProcessor::new(3); // Max 3 concurrent
        processor
            .process(rx, move |_item| {
                let a = active_clone.clone();
                let m = max_clone.clone();
                async move {
                    let current = a.fetch_add(1, Ordering::SeqCst) + 1;
                    m.fetch_max(current, Ordering::SeqCst);
                    sleep(Duration::from_millis(10)).await;
                    a.fetch_sub(1, Ordering::SeqCst);
                }
            })
            .await;

        assert!(
            max_active.load(Ordering::SeqCst) <= 3,
            "should never exceed concurrency limit of 3"
        );
    }
}
