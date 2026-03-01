//! # Challenge 1.7: Error Handling Framework
//!
//! ## Problem
//! Design a layered error handling system for a blockchain client with:
//! - Module-specific error types
//! - Automatic conversion between error layers
//! - Context preservation (which operation failed and why)
//!
//! ## Why This Matters
//! Production blockchain clients can't unwrap() everywhere. Errors must propagate
//! with enough context to debug issues in production.
//!
//! ## Requirements
//! Implement error types for three layers:
//! - `StorageError` — database failures
//! - `ExecutionError` — transaction execution failures
//! - `NodeError` — top-level node errors that wrap the above

use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    NotFound { key: String },
    Corruption { details: String },
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum ExecutionError {
    OutOfGas { used: u64, limit: u64 },
    InvalidNonce { expected: u64, got: u64 },
    InsufficientBalance { required: u64, available: u64 },
    StorageFailure(StorageError),
}

#[derive(Debug)]
pub enum NodeError {
    Execution(ExecutionError),
    Storage(StorageError),
    NetworkTimeout { endpoint: String },
}

// TODO: Implement Display for all three error types
// TODO: Implement From<StorageError> for ExecutionError
// TODO: Implement From<ExecutionError> for NodeError
// TODO: Implement From<StorageError> for NodeError

impl fmt::Display for StorageError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Format each variant with useful context")
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Format each variant with useful context")
    }
}

impl fmt::Display for NodeError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Format each variant with useful context")
    }
}

impl From<StorageError> for ExecutionError {
    fn from(_err: StorageError) -> Self {
        todo!("Wrap StorageError in ExecutionError::StorageFailure")
    }
}

impl From<ExecutionError> for NodeError {
    fn from(_err: ExecutionError) -> Self {
        todo!("Wrap ExecutionError in NodeError::Execution")
    }
}

impl From<StorageError> for NodeError {
    fn from(_err: StorageError) -> Self {
        todo!("Wrap StorageError in NodeError::Storage")
    }
}

/// Simulates a storage lookup that may fail.
pub fn get_account_balance(_address: &str) -> Result<u64, StorageError> {
    todo!("Return Ok(balance) or Err(StorageError::NotFound)")
}

/// Simulates transaction execution that uses storage and may fail.
pub fn execute_transfer(
    _from: &str,
    _to: &str,
    _amount: u64,
    _gas_limit: u64,
) -> Result<(), ExecutionError> {
    todo!("Use ? to propagate StorageError into ExecutionError automatically")
}

/// Top-level node handler that wraps execution errors.
pub fn handle_transaction(
    _from: &str,
    _to: &str,
    _amount: u64,
) -> Result<(), NodeError> {
    todo!("Use ? to propagate ExecutionError into NodeError automatically")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error_display() {
        let err = StorageError::NotFound {
            key: "0xabc".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("0xabc"), "should contain the key");
    }

    #[test]
    fn test_execution_error_display() {
        let err = ExecutionError::OutOfGas {
            used: 21000,
            limit: 20000,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("21000") || msg.contains("20000"));
    }

    #[test]
    fn test_storage_to_execution_conversion() {
        let storage_err = StorageError::NotFound {
            key: "test".to_string(),
        };
        let exec_err: ExecutionError = storage_err.into();
        match exec_err {
            ExecutionError::StorageFailure(_) => {} // correct
            _ => panic!("should convert to StorageFailure variant"),
        }
    }

    #[test]
    fn test_execution_to_node_conversion() {
        let exec_err = ExecutionError::OutOfGas {
            used: 100,
            limit: 50,
        };
        let node_err: NodeError = exec_err.into();
        match node_err {
            NodeError::Execution(_) => {} // correct
            _ => panic!("should convert to Execution variant"),
        }
    }

    #[test]
    fn test_error_propagation_with_question_mark() {
        // This tests that ? operator works through the From impls
        fn inner() -> Result<(), NodeError> {
            let _: u64 = get_account_balance("nonexistent")?;
            Ok(())
        }
        // Should compile and return an error
        assert!(inner().is_err());
    }
}
