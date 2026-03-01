//! # Challenge 1.4: Thread Pool
//!
//! ## Problem
//! Implement a thread pool that accepts closures and distributes work
//! across a fixed number of worker threads using channels.
//!
//! ## Why This Matters
//! Thread pools are used everywhere in blockchain clients: parallel signature
//! verification, concurrent account loading, batch hashing. Agave's banking
//! stage uses thread pools for parallel transaction execution.
//!
//! ## Requirements
//! - `ThreadPool::new(num_threads)` — spawn worker threads
//! - `pool.execute(closure)` — submit work to be executed on any worker
//! - `pool.shutdown()` — gracefully stop all workers after current tasks complete
//! - Workers should pick up tasks using a channel (mpsc)

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    // TODO: implement fields
    _placeholder: (),
}

impl ThreadPool {
    pub fn new(_num_threads: usize) -> Self {
        todo!("Spawn worker threads, set up job channel")
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!("Send closure through channel to a worker")
    }

    pub fn shutdown(self) {
        todo!("Signal workers to stop, join all threads")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_basic_execution() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();
        pool.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
        pool.shutdown();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_many_tasks() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));
        for _ in 0..100 {
            let c = counter.clone();
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
        pool.shutdown();
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_tasks_run_on_different_threads() {
        let pool = ThreadPool::new(4);
        let thread_ids = Arc::new(Mutex::new(Vec::new()));
        for _ in 0..20 {
            let ids = thread_ids.clone();
            pool.execute(move || {
                let id = thread::current().id();
                ids.lock().unwrap().push(id);
            });
        }
        pool.shutdown();
        let ids = thread_ids.lock().unwrap();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert!(unique.len() > 1, "tasks should run on multiple threads");
    }
}
