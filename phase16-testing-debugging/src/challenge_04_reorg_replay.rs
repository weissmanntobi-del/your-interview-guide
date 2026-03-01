/// Challenge 04 - Reorg & Replay: Simulate Fork + Rollback
///
/// Implement a state tracker that applies blocks sequentially, can detect forks
/// (when a new block's parent doesn't match the current chain tip), and can
/// rollback to a previous slot to handle chain reorganizations.
///
/// Scenario: Apply blocks A->B->C, then receive block D with parent=A (fork!).
/// Rollback to A, apply D->E, giving us the new canonical chain A->D->E.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SimpleTransaction {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub slot: u64,
    pub parent_slot: u64,
    pub transactions: Vec<SimpleTransaction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReorgError {
    InvalidParent,
    SlotAlreadyExists,
    SlotNotFound,
    InsufficientBalance,
}

/// Tracks balances at each slot for rollback capability.
pub struct StateTracker {
    /// Balance snapshots per slot: slot -> (pubkey -> balance)
    snapshots: HashMap<u64, HashMap<[u8; 32], u64>>,
    /// Ordered list of applied slot numbers (the current chain)
    chain: Vec<u64>,
    /// Current balances
    balances: HashMap<[u8; 32], u64>,
}

impl StateTracker {
    pub fn new() -> Self {
        let mut tracker = StateTracker {
            snapshots: HashMap::new(),
            chain: vec![0],
            balances: HashMap::new(),
        };
        // Snapshot at genesis (slot 0)
        tracker.snapshots.insert(0, HashMap::new());
        tracker
    }

    /// Apply a block on top of the current chain.
    /// The block's parent_slot must match the current tip.
    /// Executes all transactions and takes a snapshot.
    pub fn apply_block(&mut self, block: &Block) -> Result<(), ReorgError> {
        // TODO: Implement apply_block
        // 1. Verify block.parent_slot == current tip (*self.chain.last())
        // 2. Verify block.slot not already in chain
        // 3. For each transaction: debit `from`, credit `to`
        //    (return InsufficientBalance if from doesn't have enough)
        // 4. Take snapshot of current balances at this slot
        // 5. Append slot to chain
        todo!("Implement apply_block")
    }

    /// Rollback the chain to the given slot (inclusive).
    /// Removes all blocks after that slot and restores the balance snapshot.
    pub fn rollback_to_slot(&mut self, slot: u64) -> Result<(), ReorgError> {
        // TODO: Implement rollback
        // 1. Find the slot in self.chain
        // 2. Remove all slots after it from chain and snapshots
        // 3. Restore balances from the snapshot at `slot`
        todo!("Implement rollback_to_slot")
    }

    /// Get a pubkey's balance. If `slot` is Some, return historical balance.
    /// If None, return current balance.
    pub fn get_balance(&self, pubkey: &[u8; 32], slot: Option<u64>) -> u64 {
        // TODO: Implement get_balance
        // If slot is provided, look up from snapshots
        // Otherwise return from current balances
        todo!("Implement get_balance")
    }

    /// Return the current chain tip slot.
    pub fn get_current_slot(&self) -> u64 {
        *self.chain.last().unwrap_or(&0)
    }

    /// Detect if a new block would cause a reorg (parent doesn't match tip).
    pub fn detect_reorg(&self, new_block: &Block) -> bool {
        // TODO: Return true if new_block.parent_slot != current tip
        // AND parent_slot exists somewhere in our chain (it's a fork)
        todo!("Implement detect_reorg")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alice() -> [u8; 32] { [1u8; 32] }
    fn bob() -> [u8; 32] { [2u8; 32] }
    fn charlie() -> [u8; 32] { [3u8; 32] }

    fn fund_account(tracker: &mut StateTracker, pubkey: [u8; 32], amount: u64) {
        *tracker.balances.entry(pubkey).or_insert(0) += amount;
    }

    #[test]
    fn test_sequential_blocks() {
        let mut tracker = StateTracker::new();
        fund_account(&mut tracker, alice(), 1000);

        let block1 = Block {
            slot: 1,
            parent_slot: 0,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 100 }],
        };
        tracker.apply_block(&block1).unwrap();
        assert_eq!(tracker.get_balance(&alice(), None), 900);
        assert_eq!(tracker.get_balance(&bob(), None), 100);
        assert_eq!(tracker.get_current_slot(), 1);

        let block2 = Block {
            slot: 2,
            parent_slot: 1,
            transactions: vec![SimpleTransaction { from: bob(), to: charlie(), amount: 50 }],
        };
        tracker.apply_block(&block2).unwrap();
        assert_eq!(tracker.get_balance(&bob(), None), 50);
        assert_eq!(tracker.get_balance(&charlie(), None), 50);
    }

    #[test]
    fn test_rollback() {
        let mut tracker = StateTracker::new();
        fund_account(&mut tracker, alice(), 1000);

        let block1 = Block {
            slot: 1, parent_slot: 0,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 100 }],
        };
        let block2 = Block {
            slot: 2, parent_slot: 1,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 200 }],
        };
        tracker.apply_block(&block1).unwrap();
        tracker.apply_block(&block2).unwrap();
        assert_eq!(tracker.get_balance(&alice(), None), 700);

        tracker.rollback_to_slot(1).unwrap();
        assert_eq!(tracker.get_current_slot(), 1);
        assert_eq!(tracker.get_balance(&alice(), None), 900);
        assert_eq!(tracker.get_balance(&bob(), None), 100);
    }

    #[test]
    fn test_detect_fork() {
        let mut tracker = StateTracker::new();
        fund_account(&mut tracker, alice(), 1000);

        let block1 = Block {
            slot: 1, parent_slot: 0,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 100 }],
        };
        let block2 = Block {
            slot: 2, parent_slot: 1,
            transactions: vec![],
        };
        tracker.apply_block(&block1).unwrap();
        tracker.apply_block(&block2).unwrap();

        // Fork block: parent is slot 1, not slot 2 (current tip)
        let fork_block = Block {
            slot: 3, parent_slot: 1,
            transactions: vec![],
        };
        assert!(tracker.detect_reorg(&fork_block));

        // Normal block: parent is current tip
        let normal_block = Block {
            slot: 3, parent_slot: 2,
            transactions: vec![],
        };
        assert!(!tracker.detect_reorg(&normal_block));
    }

    #[test]
    fn test_full_reorg_scenario() {
        let mut tracker = StateTracker::new();
        fund_account(&mut tracker, alice(), 1000);

        // Original chain: 0 -> 1 -> 2 -> 3
        tracker.apply_block(&Block {
            slot: 1, parent_slot: 0,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 100 }],
        }).unwrap();
        tracker.apply_block(&Block {
            slot: 2, parent_slot: 1,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 200 }],
        }).unwrap();
        tracker.apply_block(&Block {
            slot: 3, parent_slot: 2,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 300 }],
        }).unwrap();
        assert_eq!(tracker.get_balance(&alice(), None), 400);
        assert_eq!(tracker.get_balance(&bob(), None), 600);

        // Fork at slot 1: rollback to 1, apply new blocks
        tracker.rollback_to_slot(1).unwrap();
        assert_eq!(tracker.get_balance(&alice(), None), 900);
        assert_eq!(tracker.get_balance(&bob(), None), 100);

        // New chain: 0 -> 1 -> 4 -> 5
        tracker.apply_block(&Block {
            slot: 4, parent_slot: 1,
            transactions: vec![SimpleTransaction { from: alice(), to: charlie(), amount: 500 }],
        }).unwrap();
        assert_eq!(tracker.get_balance(&alice(), None), 400);
        assert_eq!(tracker.get_balance(&charlie(), None), 500);
        assert_eq!(tracker.get_current_slot(), 4);
    }

    #[test]
    fn test_historical_balance() {
        let mut tracker = StateTracker::new();
        fund_account(&mut tracker, alice(), 1000);

        tracker.apply_block(&Block {
            slot: 1, parent_slot: 0,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 100 }],
        }).unwrap();
        tracker.apply_block(&Block {
            slot: 2, parent_slot: 1,
            transactions: vec![SimpleTransaction { from: alice(), to: bob(), amount: 200 }],
        }).unwrap();

        // Historical queries
        assert_eq!(tracker.get_balance(&alice(), Some(1)), 900);
        assert_eq!(tracker.get_balance(&alice(), Some(2)), 700);
        assert_eq!(tracker.get_balance(&bob(), Some(1)), 100);
    }
}
