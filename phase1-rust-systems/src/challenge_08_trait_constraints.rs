//! # Challenge 1.8: Generic Trait Constraints
//!
//! ## Problem
//! Implement a generic `EventBus` that dispatches typed events to handlers,
//! exercising advanced trait bounds: Send + Sync + 'static, trait objects,
//! downcasting, and generic constraints.
//!
//! ## Why This Matters
//! Blockchain clients use event systems for notifying subsystems about
//! new blocks, transactions, state changes. Geyser plugins, ExExes, and
//! RPC subscriptions all rely on typed event dispatch.
//!
//! ## Requirements
//! - Register handlers for specific event types
//! - Dispatch events to all handlers registered for that type
//! - Thread-safe: handlers can be registered and events dispatched from any thread
//! - Type-safe: handlers only receive events of their registered type

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Marker trait for events that can be dispatched.
pub trait Event: Any + Send + Sync + Clone + 'static {}

/// A type-erased event handler.
type BoxedHandler = Box<dyn Any + Send>;

pub struct EventBus {
    // TODO: implement fields
    // Hint: HashMap<TypeId, Vec<BoxedHandler>> protected by a Mutex
    _placeholder: (),
}

impl EventBus {
    pub fn new() -> Self {
        todo!("Initialize empty handler map")
    }

    /// Register a handler for a specific event type.
    pub fn subscribe<E: Event, F>(&self, _handler: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        todo!("Store handler indexed by TypeId of E")
    }

    /// Dispatch an event to all registered handlers for its type.
    /// Returns the number of handlers that were called.
    pub fn publish<E: Event>(&self, _event: &E) -> usize {
        todo!("Look up handlers by TypeId, downcast, and call each one")
    }
}

// Example event types for testing
#[derive(Clone, Debug)]
pub struct NewBlock {
    pub number: u64,
    pub hash: [u8; 32],
}
impl Event for NewBlock {}

#[derive(Clone, Debug)]
pub struct NewTransaction {
    pub hash: [u8; 32],
    pub from: String,
    pub value: u64,
}
impl Event for NewTransaction {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_subscribe_and_publish() {
        let bus = EventBus::new();
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();

        bus.subscribe(move |_event: &NewBlock| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let block = NewBlock {
            number: 1,
            hash: [0; 32],
        };
        let handled = bus.publish(&block);
        assert_eq!(handled, 1);
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiple_handlers_same_type() {
        let bus = EventBus::new();
        let count = Arc::new(AtomicUsize::new(0));

        for _ in 0..3 {
            let c = count.clone();
            bus.subscribe(move |_: &NewBlock| {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        let block = NewBlock { number: 1, hash: [0; 32] };
        let handled = bus.publish(&block);
        assert_eq!(handled, 3);
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_different_event_types_isolated() {
        let bus = EventBus::new();
        let block_count = Arc::new(AtomicUsize::new(0));
        let tx_count = Arc::new(AtomicUsize::new(0));

        let bc = block_count.clone();
        bus.subscribe(move |_: &NewBlock| { bc.fetch_add(1, Ordering::SeqCst); });

        let tc = tx_count.clone();
        bus.subscribe(move |_: &NewTransaction| { tc.fetch_add(1, Ordering::SeqCst); });

        let block = NewBlock { number: 1, hash: [0; 32] };
        bus.publish(&block);

        assert_eq!(block_count.load(Ordering::SeqCst), 1);
        assert_eq!(tx_count.load(Ordering::SeqCst), 0); // Tx handler NOT called
    }

    #[test]
    fn test_no_handlers_returns_zero() {
        let bus = EventBus::new();
        let block = NewBlock { number: 1, hash: [0; 32] };
        assert_eq!(bus.publish(&block), 0);
    }

    #[test]
    fn test_handler_receives_correct_data() {
        let bus = EventBus::new();
        let received_number = Arc::new(AtomicUsize::new(0));
        let rn = received_number.clone();

        bus.subscribe(move |event: &NewBlock| {
            rn.store(event.number as usize, Ordering::SeqCst);
        });

        let block = NewBlock { number: 42, hash: [0; 32] };
        bus.publish(&block);
        assert_eq!(received_number.load(Ordering::SeqCst), 42);
    }
}
