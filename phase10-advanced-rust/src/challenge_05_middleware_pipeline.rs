//! # Challenge 10.5: Middleware Pipeline
//!
//! ## Problem
//! Build a composable service pipeline where middleware layers (rate limiter,
//! retry, timeout, logger) wrap an inner handler, forming a nested service stack.
//!
//! ## Why This Matters
//! Tower-style middleware is the backbone of Solana RPC infrastructure and validator
//! networking. Composable request processing is a pattern every senior Rust engineer
//! should know. This tests trait objects, generics, and interior mutability.
//!
//! ## Requirements
//! - `Service` trait with `fn call(&self, request: Request) -> Result<Response, ServiceError>`
//! - `RateLimiter<S>`: rejects after N requests
//! - `RetryMiddleware<S>`: retries on failure up to N times
//! - `TimeoutMiddleware<S>`: returns Timeout if payload contains "slow"
//! - `Logger<S>`: records request/response log entries
//! - Middleware composable in any order
//!
//! ## Constraints
//! - Use `Cell`/`RefCell` for interior mutability (state tracking in &self methods)
//! - Each middleware must implement the `Service` trait
//!
//! ## Hints
//! - Each middleware holds `inner: S` where `S: Service`
//! - RateLimiter uses Cell<usize> for request count
//! - Logger uses RefCell<Vec<String>> for recording

use std::cell::{Cell, RefCell};

#[derive(Debug, Clone)]
pub struct Request {
    pub id: u64,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub request_id: u64,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    RateLimited,
    Timeout,
    Internal(String),
}

pub trait Service {
    fn call(&self, request: Request) -> Result<Response, ServiceError>;
}

/// Echo handler — returns the payload as the response body.
pub struct EchoHandler;

impl Service for EchoHandler {
    fn call(&self, request: Request) -> Result<Response, ServiceError> {
        todo!("Return a Response echoing the payload")
    }
}

/// A handler that fails for the first N calls, then succeeds.
pub struct FailingHandler {
    // TODO: call_count: Cell<usize>, succeed_after: usize
    _placeholder: (),
}

impl FailingHandler {
    pub fn new(_succeed_after: usize) -> Self {
        todo!("Track call count, succeed after N failures")
    }

    pub fn call_count(&self) -> usize {
        todo!()
    }
}

impl Service for FailingHandler {
    fn call(&self, _request: Request) -> Result<Response, ServiceError> {
        todo!("Fail with Internal error for first N calls, then succeed")
    }
}

pub struct RateLimiter<S: Service> {
    inner: S,
    max_requests: usize,
    current_count: Cell<usize>,
}

impl<S: Service> RateLimiter<S> {
    pub fn new(inner: S, max_requests: usize) -> Self {
        todo!()
    }

    pub fn reset(&self) {
        todo!("Reset current_count to 0")
    }
}

impl<S: Service> Service for RateLimiter<S> {
    fn call(&self, _request: Request) -> Result<Response, ServiceError> {
        todo!("Check count < max, increment, delegate or reject")
    }
}

pub struct RetryMiddleware<S: Service> {
    inner: S,
    max_retries: usize,
}

impl<S: Service> RetryMiddleware<S> {
    pub fn new(inner: S, max_retries: usize) -> Self {
        todo!()
    }
}

impl<S: Service> Service for RetryMiddleware<S> {
    fn call(&self, _request: Request) -> Result<Response, ServiceError> {
        todo!("Retry inner.call up to max_retries times on failure")
    }
}

pub struct TimeoutMiddleware<S: Service> {
    inner: S,
}

impl<S: Service> TimeoutMiddleware<S> {
    pub fn new(inner: S) -> Self {
        todo!()
    }
}

impl<S: Service> Service for TimeoutMiddleware<S> {
    fn call(&self, _request: Request) -> Result<Response, ServiceError> {
        todo!("If payload contains 'slow', return Timeout. Otherwise delegate.")
    }
}

pub struct Logger<S: Service> {
    inner: S,
    log: RefCell<Vec<String>>,
}

impl<S: Service> Logger<S> {
    pub fn new(inner: S) -> Self {
        todo!()
    }

    pub fn entries(&self) -> Vec<String> {
        todo!("Return clone of log entries")
    }
}

impl<S: Service> Service for Logger<S> {
    fn call(&self, _request: Request) -> Result<Response, ServiceError> {
        todo!("Log request, call inner, log result, return")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_handler() {
        let handler = EchoHandler;
        let req = Request { id: 1, payload: "hello".to_string() };
        let resp = handler.call(req).unwrap();
        assert_eq!(resp.request_id, 1);
        assert_eq!(resp.body, "hello");
    }

    #[test]
    fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new(EchoHandler, 3);
        for i in 0..3 {
            let req = Request { id: i, payload: "ok".to_string() };
            assert!(limiter.call(req).is_ok());
        }
    }

    #[test]
    fn test_rate_limiter_rejects_excess() {
        let limiter = RateLimiter::new(EchoHandler, 2);
        limiter.call(Request { id: 1, payload: "a".into() }).unwrap();
        limiter.call(Request { id: 2, payload: "b".into() }).unwrap();
        assert_eq!(
            limiter.call(Request { id: 3, payload: "c".into() }).unwrap_err(),
            ServiceError::RateLimited,
        );
    }

    #[test]
    fn test_timeout_triggers_on_slow() {
        let timeout = TimeoutMiddleware::new(EchoHandler);
        let req = Request { id: 1, payload: "this is slow".into() };
        assert_eq!(timeout.call(req).unwrap_err(), ServiceError::Timeout);
    }

    #[test]
    fn test_timeout_passes_fast() {
        let timeout = TimeoutMiddleware::new(EchoHandler);
        let req = Request { id: 1, payload: "fast request".into() };
        assert!(timeout.call(req).is_ok());
    }

    #[test]
    fn test_retry_recovers() {
        let handler = FailingHandler::new(2);
        let retry = RetryMiddleware::new(handler, 3);
        let req = Request { id: 1, payload: "test".into() };
        assert!(retry.call(req).is_ok());
    }

    #[test]
    fn test_retry_exhausted() {
        let handler = FailingHandler::new(5);
        let retry = RetryMiddleware::new(handler, 2);
        let req = Request { id: 1, payload: "test".into() };
        assert!(retry.call(req).is_err());
    }

    #[test]
    fn test_logger_records() {
        let logger = Logger::new(EchoHandler);
        let req = Request { id: 42, payload: "hello".into() };
        let _ = logger.call(req);
        let entries = logger.entries();
        assert!(entries.len() >= 2);
        assert!(entries.iter().any(|e| e.contains("42")));
    }

    #[test]
    fn test_composed_pipeline() {
        let handler = EchoHandler;
        let timeout = TimeoutMiddleware::new(handler);
        let limiter = RateLimiter::new(timeout, 10);
        let logger = Logger::new(limiter);

        let req = Request { id: 1, payload: "normal".into() };
        assert!(logger.call(req).is_ok());

        let slow = Request { id: 2, payload: "slow request".into() };
        assert_eq!(logger.call(slow).unwrap_err(), ServiceError::Timeout);
    }
}
