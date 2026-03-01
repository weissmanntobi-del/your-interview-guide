//! # Challenge 05: Fee Estimator
//!
//! ## Problem
//! Build a fee estimator that tracks recent priority fees across slots and provides
//! statistical estimates. The estimator maintains a sliding window of recent slot data
//! and can answer queries like "what fee (in micro-lamports per CU) should I pay to
//! land in the Nth percentile of recent transactions?"
//!
//! ## Why This Matters
//! Wallets and dApps need to suggest appropriate priority fees to users. Too low and
//! the transaction does not land; too high and the user overpays. A percentile-based
//! estimator using recent slot data is how `getRecentPrioritizationFees` works in
//! practice. This challenge teaches sliding window statistics and FIFO eviction.
//!
//! ## Requirements
//! - `FeeEstimator::new(window_size)` creates an estimator that tracks the most
//!   recent `window_size` slots.
//! - `record_slot(slot_data)` adds fee data for a slot. If the window is full,
//!   the oldest slot is evicted (FIFO).
//! - `estimate_fee(percentile)` returns the fee at the given percentile (0.0 to 1.0)
//!   across all recorded fees. Returns 0 if no data is available.
//! - `average_fee()` returns the mean fee across all recorded fees (0 if empty).
//! - `min_fee()` and `max_fee()` return extremes (0 if empty).
//! - `slots_tracked()` returns the number of slots currently in the window.
//!
//! ## Constraints
//! - Percentile 0.0 returns the minimum, 1.0 returns the maximum.
//! - Percentile uses nearest-rank method: index = ceil(percentile * N) - 1, clamped.
//! - All fees are `u64` (micro-lamports per compute unit).
//! - Window size must be at least 1.
//!
//! ## Hints
//! - Use a `VecDeque<SlotFeeData>` for the sliding window.
//! - For `estimate_fee`, collect all fees from all slots, sort them, then pick by index.
//! - Nearest-rank: index = max(0, ceil(p * n) - 1) where n = total number of fees.
//! - `average_fee` should use integer division (floor).

use std::collections::VecDeque;

pub type Pubkey = [u8; 32];

/// Fee data collected from a single slot.
#[derive(Debug, Clone)]
pub struct SlotFeeData {
    /// The slot number.
    pub slot: u64,
    /// Priority fees observed in this slot (micro-lamports per CU).
    pub fees: Vec<u64>,
}

/// A sliding-window fee estimator.
#[derive(Debug)]
pub struct FeeEstimator {
    window_size: usize,
    slots: VecDeque<SlotFeeData>,
}

impl FeeEstimator {
    /// Create a new fee estimator with the given window size.
    ///
    /// The estimator will track at most `window_size` recent slots.
    /// Panics if `window_size` is 0.
    pub fn new(window_size: usize) -> Self {
        todo!("Initialize the estimator with an empty VecDeque and store window_size")
    }

    /// Record fee data for a slot.
    ///
    /// If the window is already full, evict the oldest slot first (FIFO).
    pub fn record_slot(&mut self, slot_data: SlotFeeData) {
        todo!(
            "If slots.len() == window_size, pop_front. Then push_back the new slot_data."
        )
    }

    /// Estimate the fee at a given percentile (0.0 to 1.0).
    ///
    /// Collects all fees from all tracked slots, sorts them, and returns the value
    /// at the nearest-rank index. Returns 0 if no fees are recorded.
    ///
    /// Nearest-rank method: index = max(0, ceil(percentile * n) - 1)
    pub fn estimate_fee(&self, percentile: f64) -> u64 {
        todo!(
            "Collect all fees into a single Vec, sort, compute index using nearest-rank, return"
        )
    }

    /// Return the arithmetic mean of all recorded fees (integer division).
    /// Returns 0 if no fees exist.
    pub fn average_fee(&self) -> u64 {
        todo!("Sum all fees from all slots, divide by count")
    }

    /// Return the minimum fee across all recorded slots. Returns 0 if empty.
    pub fn min_fee(&self) -> u64 {
        todo!("Iterate all fees, find minimum")
    }

    /// Return the maximum fee across all recorded slots. Returns 0 if empty.
    pub fn max_fee(&self) -> u64 {
        todo!("Iterate all fees, find maximum")
    }

    /// Return the number of slots currently tracked in the window.
    pub fn slots_tracked(&self) -> usize {
        todo!("Return slots.len()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_estimator_returns_zero() {
        let estimator = FeeEstimator::new(10);
        assert_eq!(estimator.estimate_fee(0.5), 0);
        assert_eq!(estimator.average_fee(), 0);
        assert_eq!(estimator.min_fee(), 0);
        assert_eq!(estimator.max_fee(), 0);
        assert_eq!(estimator.slots_tracked(), 0);
    }

    #[test]
    fn test_single_slot_statistics() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData {
            slot: 100,
            fees: vec![1000, 2000, 3000, 4000, 5000],
        });
        assert_eq!(estimator.slots_tracked(), 1);
        assert_eq!(estimator.min_fee(), 1000);
        assert_eq!(estimator.max_fee(), 5000);
        assert_eq!(estimator.average_fee(), 3000);
    }

    #[test]
    fn test_multi_slot_aggregation() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData { slot: 1, fees: vec![100, 200] });
        estimator.record_slot(SlotFeeData { slot: 2, fees: vec![300, 400] });

        assert_eq!(estimator.slots_tracked(), 2);
        assert_eq!(estimator.min_fee(), 100);
        assert_eq!(estimator.max_fee(), 400);
        // average: (100 + 200 + 300 + 400) / 4 = 250
        assert_eq!(estimator.average_fee(), 250);
    }

    #[test]
    fn test_percentile_boundaries() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData {
            slot: 1,
            fees: vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
        });

        // Percentile 0.0 should return the minimum
        assert_eq!(estimator.estimate_fee(0.0), 10);
        // Percentile 1.0 should return the maximum
        assert_eq!(estimator.estimate_fee(1.0), 100);
    }

    #[test]
    fn test_percentile_50th() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData {
            slot: 1,
            fees: vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
        });

        // 50th percentile: ceil(0.5 * 10) - 1 = 5 - 1 = 4 -> value at index 4 = 50
        assert_eq!(estimator.estimate_fee(0.5), 50);
    }

    #[test]
    fn test_percentile_75th() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData {
            slot: 1,
            fees: vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
        });

        // 75th percentile: ceil(0.75 * 10) - 1 = 8 - 1 = 7 -> value at index 7 = 80
        assert_eq!(estimator.estimate_fee(0.75), 80);
    }

    #[test]
    fn test_window_eviction() {
        let mut estimator = FeeEstimator::new(2);
        estimator.record_slot(SlotFeeData { slot: 1, fees: vec![100] });
        estimator.record_slot(SlotFeeData { slot: 2, fees: vec![200] });
        assert_eq!(estimator.slots_tracked(), 2);

        // This should evict slot 1
        estimator.record_slot(SlotFeeData { slot: 3, fees: vec![300] });
        assert_eq!(estimator.slots_tracked(), 2);

        // Minimum should now be 200 (slot 1 was evicted)
        assert_eq!(estimator.min_fee(), 200);
        assert_eq!(estimator.max_fee(), 300);
    }

    #[test]
    fn test_slot_with_empty_fees() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData { slot: 1, fees: vec![] });
        assert_eq!(estimator.slots_tracked(), 1);
        assert_eq!(estimator.estimate_fee(0.5), 0);
        assert_eq!(estimator.average_fee(), 0);
    }

    #[test]
    fn test_single_fee_value() {
        let mut estimator = FeeEstimator::new(10);
        estimator.record_slot(SlotFeeData { slot: 1, fees: vec![42] });
        assert_eq!(estimator.estimate_fee(0.0), 42);
        assert_eq!(estimator.estimate_fee(0.5), 42);
        assert_eq!(estimator.estimate_fee(1.0), 42);
        assert_eq!(estimator.average_fee(), 42);
    }

    #[test]
    #[should_panic]
    fn test_zero_window_size_panics() {
        FeeEstimator::new(0);
    }
}
