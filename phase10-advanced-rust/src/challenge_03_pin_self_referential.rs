//! # Challenge 10.3: Pin and Self-Referential Structs
//!
//! ## Problem
//! Build a struct that contains a reference to its own data, using Pin to ensure
//! safety. Also implement a manual Future to understand how Pin interacts with
//! async Rust.
//!
//! ## Why This Matters
//! Understanding Pin is critical for async Rust and some Agave internal data
//! structures. Self-referential structs arise naturally when you need a value
//! that points to part of itself (e.g., a parsed view into an owned buffer).
//! Most Rust developers never build these — doing so shows deep language mastery.
//!
//! ## Requirements
//! - `SelfRef` struct: contains a `String` and a `*const str` pointing into it
//! - Must use `Pin<Box<SelfRef>>` to prevent moves that invalidate the pointer
//! - `new(data: String) -> Pin<Box<SelfRef>>` — pin then set the pointer
//! - `get_data(&self) -> &str` — safely read through the self-referential pointer
//! - `SimpleFuture` — a manually implemented Future that takes two polls to complete
//!
//! ## Constraints
//! - The self-referential pointer must be set AFTER pinning (not before)
//! - Use unsafe only where necessary and document why it's sound
//! - The SimpleFuture must wake the waker on Pending to be correct
//!
//! ## Hints
//! - Pin the struct first with `Box::pin(...)`, then use `unsafe` to set the pointer
//! - `Pin::as_ref()` and `Pin::get_ref()` for safe access to pinned data
//! - For SimpleFuture, use an enum state (NotStarted → Running → Done)
//! - Call `cx.waker().wake_by_ref()` when returning Pending so the executor re-polls

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A struct that holds a String and a raw pointer into that String.
/// Must be pinned to be safe — if the struct moves, the pointer is invalidated.
pub struct SelfRef {
    // TODO: implement fields
    // - data: String
    // - data_ptr: *const str (points into `data`)
    _placeholder: (),
}

impl SelfRef {
    /// Create a new pinned SelfRef.
    ///
    /// Steps:
    /// 1. Create the struct with a null/dangling pointer
    /// 2. Pin it in a Box
    /// 3. Use unsafe to set data_ptr to point into the pinned data
    pub fn new(_data: String) -> Pin<Box<SelfRef>> {
        todo!(
            "Create struct, pin it, then use unsafe to set data_ptr \
             to point at the pinned String's contents"
        )
    }

    /// Read data through the self-referential pointer.
    ///
    /// This is safe because Pin guarantees we haven't moved since
    /// setting the pointer.
    pub fn get_data(&self) -> &str {
        todo!("Dereference data_ptr safely — guaranteed valid because we are pinned")
    }
}

/// States for the manually-implemented future.
pub enum FutureState {
    NotStarted,
    Running,
    Done,
}

/// A simple future that takes two polls to complete.
/// First poll: NotStarted → Running, returns Pending.
/// Second poll: Running → Done, returns Ready(value).
pub struct SimpleFuture {
    // TODO: implement fields
    // - state: FutureState
    // - value: Option<String>
    _placeholder: (),
}

impl SimpleFuture {
    /// Create a new SimpleFuture that will eventually produce the given value.
    pub fn new(_value: String) -> Self {
        todo!("Create a SimpleFuture in NotStarted state with the given value")
    }
}

impl Future for SimpleFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!(
            "Match on state: \
             NotStarted → set Running, wake the waker, return Pending; \
             Running → set Done, return Ready(value.take()); \
             Done → panic (should not be polled after completion)"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_ref_basic() {
        let pinned = SelfRef::new("hello world".to_string());
        assert_eq!(pinned.get_data(), "hello world");
    }

    #[test]
    fn test_self_ref_long_string() {
        let long = "a".repeat(10_000);
        let pinned = SelfRef::new(long.clone());
        assert_eq!(pinned.get_data(), long.as_str());
    }

    #[test]
    fn test_multiple_self_refs_independent() {
        let a = SelfRef::new("alpha".to_string());
        let b = SelfRef::new("beta".to_string());
        let c = SelfRef::new("gamma".to_string());
        assert_eq!(a.get_data(), "alpha");
        assert_eq!(b.get_data(), "beta");
        assert_eq!(c.get_data(), "gamma");
    }

    #[test]
    fn test_self_ref_empty_string() {
        let pinned = SelfRef::new(String::new());
        assert_eq!(pinned.get_data(), "");
    }

    #[tokio::test]
    async fn test_simple_future_resolves() {
        let future = SimpleFuture::new("result".to_string());
        let output = future.await;
        assert_eq!(output, "result");
    }

    #[tokio::test]
    async fn test_simple_future_different_values() {
        let f1 = SimpleFuture::new("first".to_string());
        let f2 = SimpleFuture::new("second".to_string());
        assert_eq!(f1.await, "first");
        assert_eq!(f2.await, "second");
    }

    #[test]
    fn test_simple_future_poll_sequence() {
        use std::task::{RawWaker, RawWakerVTable, Waker};

        fn noop_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(p: *const ()) -> RawWaker {
                RawWaker::new(p, &VTABLE)
            }
            static VTABLE: RawWakerVTable =
                RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
        let mut cx = Context::from_waker(&waker);

        let mut future = SimpleFuture::new("done".to_string());
        let mut pinned = unsafe { Pin::new_unchecked(&mut future) };

        // First poll should return Pending
        assert!(pinned.as_mut().poll(&mut cx).is_pending());

        // Second poll should return Ready
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(val) => assert_eq!(val, "done"),
            Poll::Pending => panic!("expected Ready on second poll"),
        }
    }
}
