//! # Challenge 3.1: Transaction Dependency Graph
//!
//! Given a set of transactions with account access lists, produce a valid
//! execution order using topological sort. Detect circular dependencies.
//!
//! Time: 45 min | Difficulty: Medium

use std::collections::{HashMap, HashSet};

pub type TxId = u64;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TxId,
    pub reads: Vec<String>,  // accounts read
    pub writes: Vec<String>, // accounts written
}

#[derive(Debug, PartialEq)]
pub enum ScheduleError {
    CyclicDependency,
}

/// Produce a valid execution order respecting dependencies.
/// A transaction B depends on A if A writes to an account that B reads or writes.
/// Returns ordered list of TxIds, or error if cyclic dependency detected.
pub fn topological_order(_transactions: &[Transaction]) -> Result<Vec<TxId>, ScheduleError> {
    todo!("Build dependency graph, topological sort, detect cycles")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_independent_transactions() {
        let txs = vec![
            Transaction { id: 0, reads: vec![], writes: vec!["A".into()] },
            Transaction { id: 1, reads: vec![], writes: vec!["B".into()] },
        ];
        let order = topological_order(&txs).unwrap();
        assert_eq!(order.len(), 2);
    }

    #[test]
    fn test_sequential_dependency() {
        let txs = vec![
            Transaction { id: 0, reads: vec![], writes: vec!["A".into()] },
            Transaction { id: 1, reads: vec!["A".into()], writes: vec!["B".into()] },
            Transaction { id: 2, reads: vec!["B".into()], writes: vec![] },
        ];
        let order = topological_order(&txs).unwrap();
        let pos = |id: u64| order.iter().position(|&x| x == id).unwrap();
        assert!(pos(0) < pos(1));
        assert!(pos(1) < pos(2));
    }

    #[test]
    fn test_write_write_conflict() {
        let txs = vec![
            Transaction { id: 0, reads: vec![], writes: vec!["A".into()] },
            Transaction { id: 1, reads: vec![], writes: vec!["A".into()] },
        ];
        // Both write A — one must come before the other
        let order = topological_order(&txs).unwrap();
        assert_eq!(order.len(), 2);
    }
}
