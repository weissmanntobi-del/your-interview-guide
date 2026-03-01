/// # Challenge 04: Slot Subscription — Slot Tracking with Forks
///
/// Solana uses a slot-based block production system where validators produce blocks
/// in sequential slots. Forks occur when multiple leaders produce competing blocks
/// for the same parent. The network resolves forks through voting, eventually
/// finalizing one chain. This challenge implements a SlotTracker that processes
/// slot updates, tracks parent-child relationships, detects forks, and maintains
/// the finalized slot.
///
/// Key concepts:
/// - Slot progression: parent -> child relationships form a chain
/// - Fork detection: two slots sharing the same parent
/// - Status promotion: Processed -> Confirmed -> Finalized
/// - Fork switching: the main fork can change when a competing fork gains votes

use std::collections::HashMap;

/// Status of a slot as it progresses through consensus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotStatus {
    Processed,
    Confirmed,
    Finalized,
}

/// Information about a slot received from the validator.
#[derive(Debug, Clone, PartialEq)]
pub struct SlotInfo {
    pub slot: u64,
    pub parent: u64,
    pub status: SlotStatus,
}

/// Fork metadata for a given slot.
#[derive(Debug, Clone, PartialEq)]
pub struct ForkInfo {
    pub slot: u64,
    pub parent: u64,
    pub is_main_fork: bool,
}

/// Internal record for a tracked slot.
#[derive(Debug, Clone)]
struct SlotRecord {
    parent: u64,
    status: SlotStatus,
    is_main_fork: bool,
}

/// Tracks slot progression, ancestry chains, fork detection, and finalization.
pub struct SlotTracker {
    slots: HashMap<u64, SlotRecord>,
    finalized_slot: u64,
}

impl SlotTracker {
    /// Create a new SlotTracker with no slots tracked.
    pub fn new() -> Self {
        todo!("Initialize with empty slots map and finalized_slot = 0")
    }

    /// Process a new slot update. If the slot is already tracked, update its
    /// status if the new status is higher (Processed < Confirmed < Finalized).
    /// New slots are added and marked as main fork by default.
    /// If status is Finalized, update finalized_slot.
    pub fn process_slot(&mut self, info: SlotInfo) {
        todo!("Insert or update slot record. Promote status if new status is higher. Update finalized_slot if Finalized. Mark new slots as main fork.")
    }

    /// Get the current status of a slot, or None if not tracked.
    pub fn get_status(&self, slot: u64) -> Option<SlotStatus> {
        todo!("Look up slot in the map and return its status")
    }

    /// Detect whether two slots represent a fork (i.e., they share the same parent
    /// but are different slots). Both slots must exist in the tracker.
    pub fn detect_fork(&self, slot_a: u64, slot_b: u64) -> bool {
        todo!("Look up both slots. Return true if they exist, are different slots, and share the same parent.")
    }

    /// Walk backwards from the given slot through parent links to build the
    /// ancestry chain. Returns the chain from the given slot back to the
    /// earliest known ancestor, inclusive.
    pub fn get_chain(&self, slot: u64) -> Vec<u64> {
        todo!("Starting from slot, follow parent links while the parent exists in the map. Collect all slot numbers into a Vec. Stop when parent is not tracked or slot == parent.")
    }

    /// Return the most recently finalized slot number.
    pub fn finalized_slot(&self) -> u64 {
        todo!("Return self.finalized_slot")
    }

    /// Switch the main fork designation: mark all slots in the chain ending at
    /// `new_tip` as main fork, and mark all slots in the chain ending at
    /// `old_tip` as not main fork (unless they are common ancestors).
    pub fn handle_fork_switch(&mut self, old_tip: u64, new_tip: u64) {
        todo!("Get chains for both tips. Find the common prefix. Mark old_tip's unique slots as not main fork. Mark new_tip's unique slots as main fork.")
    }

    /// Get fork info for a specific slot.
    pub fn get_fork_info(&self, slot: u64) -> Option<ForkInfo> {
        todo!("Look up slot record and build ForkInfo with slot, parent, is_main_fork")
    }
}

/// Compare slot statuses for ordering: Processed < Confirmed < Finalized.
fn status_rank(status: SlotStatus) -> u8 {
    todo!("Return 0 for Processed, 1 for Confirmed, 2 for Finalized")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_progression() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 2, parent: 1, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 3, parent: 2, status: SlotStatus::Processed });
        assert_eq!(tracker.get_status(3), Some(SlotStatus::Processed));
    }

    #[test]
    fn test_status_promotion() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Confirmed });
        assert_eq!(tracker.get_status(1), Some(SlotStatus::Confirmed));
    }

    #[test]
    fn test_status_no_demotion() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Confirmed });
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        assert_eq!(tracker.get_status(1), Some(SlotStatus::Confirmed));
    }

    #[test]
    fn test_fork_detection() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 2, parent: 1, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 3, parent: 1, status: SlotStatus::Processed }); // fork!
        assert!(tracker.detect_fork(2, 3));
        assert!(!tracker.detect_fork(1, 2)); // different parents
    }

    #[test]
    fn test_finalized_tracking() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Finalized });
        assert_eq!(tracker.finalized_slot(), 1);
        tracker.process_slot(SlotInfo { slot: 2, parent: 1, status: SlotStatus::Finalized });
        assert_eq!(tracker.finalized_slot(), 2);
    }

    #[test]
    fn test_get_chain() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 2, parent: 1, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 3, parent: 2, status: SlotStatus::Processed });
        let chain = tracker.get_chain(3);
        assert_eq!(chain, vec![3, 2, 1]);
    }

    #[test]
    fn test_fork_switch() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        tracker.process_slot(SlotInfo { slot: 2, parent: 1, status: SlotStatus::Processed }); // main
        tracker.process_slot(SlotInfo { slot: 3, parent: 1, status: SlotStatus::Processed }); // fork
        tracker.handle_fork_switch(2, 3);
        let info2 = tracker.get_fork_info(2).unwrap();
        let info3 = tracker.get_fork_info(3).unwrap();
        assert!(!info2.is_main_fork);
        assert!(info3.is_main_fork);
    }

    #[test]
    fn test_unknown_slot_returns_none() {
        let tracker = SlotTracker::new();
        assert_eq!(tracker.get_status(999), None);
    }

    #[test]
    fn test_get_chain_single_slot() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 5, parent: 0, status: SlotStatus::Processed });
        let chain = tracker.get_chain(5);
        assert_eq!(chain, vec![5]);
    }

    #[test]
    fn test_detect_fork_nonexistent_slot() {
        let mut tracker = SlotTracker::new();
        tracker.process_slot(SlotInfo { slot: 1, parent: 0, status: SlotStatus::Processed });
        assert!(!tracker.detect_fork(1, 99));
    }
}
