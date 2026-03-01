//! # Challenge 01: Priority Fee Calculator
//!
//! ## Problem
//! Implement Solana's priority fee mechanism. Transactions can optionally include
//! compute budget instructions that set a compute unit limit and a per-compute-unit
//! price (in micro-lamports). The priority fee is:
//!
//!   priority_fee = compute_units * micro_lamport_price / 1_000_000
//!
//! The total fee is base_fee + priority_fee. Validators use priority fees to rank
//! transactions during block production so that higher-paying transactions land first.
//!
//! ## Why This Matters
//! Priority fees are the core economic signal in Solana's fee market. Understanding
//! how they are calculated, how defaults apply, and how validators rank transactions
//! is essential for building efficient programs and MEV-aware systems.
//!
//! ## Requirements
//! - Parse `ComputeBudgetInstruction` variants to extract CU limit and price.
//! - Apply defaults: 200_000 CU limit, 0 micro-lamport price when not specified.
//! - `compute_priority_fee` returns the fee in lamports (integer division).
//! - `compute_total_fee` adds the base fee to the priority fee.
//! - `rank_transactions` sorts a list by priority fee descending (stable order on tie).
//! - `effective_compute_price` returns the micro-lamport price per CU actually paid.
//!
//! ## Constraints
//! - All arithmetic must avoid overflow (use u128 intermediates where needed).
//! - Priority fee uses integer (floor) division.
//! - Default base fee is 5000 lamports (one signature).
//!
//! ## Hints
//! - Scan instructions to find the *last* SetComputeUnitLimit and *last* SetComputeUnitPrice.
//! - For ranking, `sort_by` with `Ordering::reverse` on fee keeps stable order on ties.
//! - `effective_compute_price` is the inverse: (priority_fee * 1_000_000) / compute_units.

pub type Pubkey = [u8; 32];

/// Compute budget instructions that a transaction may include.
#[derive(Debug, Clone, PartialEq)]
pub enum ComputeBudgetInstruction {
    /// Set the maximum compute units the transaction may consume.
    SetComputeUnitLimit(u32),
    /// Set the price in micro-lamports per compute unit.
    SetComputeUnitPrice(u64),
}

/// All fee-relevant information extracted from a transaction.
#[derive(Debug, Clone)]
pub struct TransactionFeeInfo {
    pub signature: [u8; 64],
    pub instructions: Vec<ComputeBudgetInstruction>,
    pub base_fee: u64,
}

/// The default compute unit limit when none is specified.
pub const DEFAULT_COMPUTE_UNIT_LIMIT: u32 = 200_000;

/// The default micro-lamport price when none is specified.
pub const DEFAULT_COMPUTE_UNIT_PRICE: u64 = 0;

/// Extract the compute unit limit from the transaction's instructions.
/// Uses the *last* `SetComputeUnitLimit` found, or the default.
pub fn extract_compute_unit_limit(info: &TransactionFeeInfo) -> u32 {
    todo!("Scan info.instructions for the last SetComputeUnitLimit; return default if absent")
}

/// Extract the micro-lamport price from the transaction's instructions.
/// Uses the *last* `SetComputeUnitPrice` found, or the default.
pub fn extract_compute_unit_price(info: &TransactionFeeInfo) -> u64 {
    todo!("Scan info.instructions for the last SetComputeUnitPrice; return default if absent")
}

/// Compute the priority fee in lamports.
///
/// priority_fee = compute_units * micro_lamport_price / 1_000_000
///
/// Use u128 intermediates to avoid overflow.
pub fn compute_priority_fee(info: &TransactionFeeInfo) -> u64 {
    todo!("Extract limit and price, multiply as u128, divide by 1_000_000, cast back to u64")
}

/// Compute the total fee: base_fee + priority_fee.
pub fn compute_total_fee(info: &TransactionFeeInfo) -> u64 {
    todo!("Add base_fee to compute_priority_fee result")
}

/// Rank transactions by priority fee, descending. Ties preserve original order.
pub fn rank_transactions(txs: &[TransactionFeeInfo]) -> Vec<&TransactionFeeInfo> {
    todo!("Collect refs, sort_by priority fee descending (stable sort)")
}

/// Return the effective micro-lamport price per compute unit actually paid.
///
/// effective_price = (priority_fee * 1_000_000) / compute_units
///
/// Returns 0 if compute_units is 0.
pub fn effective_compute_price(info: &TransactionFeeInfo) -> u64 {
    todo!("Compute priority_fee, then reverse the formula to get price per CU")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_info(instructions: Vec<ComputeBudgetInstruction>, base_fee: u64) -> TransactionFeeInfo {
        TransactionFeeInfo {
            signature: [0u8; 64],
            instructions,
            base_fee,
        }
    }

    #[test]
    fn test_defaults_when_no_instructions() {
        let info = make_info(vec![], 5000);
        assert_eq!(extract_compute_unit_limit(&info), 200_000);
        assert_eq!(extract_compute_unit_price(&info), 0);
        assert_eq!(compute_priority_fee(&info), 0);
        assert_eq!(compute_total_fee(&info), 5000);
    }

    #[test]
    fn test_explicit_limit_and_price() {
        let info = make_info(
            vec![
                ComputeBudgetInstruction::SetComputeUnitLimit(400_000),
                ComputeBudgetInstruction::SetComputeUnitPrice(1_000_000), // 1 lamport/CU
            ],
            5000,
        );
        assert_eq!(extract_compute_unit_limit(&info), 400_000);
        assert_eq!(extract_compute_unit_price(&info), 1_000_000);
        // 400_000 * 1_000_000 / 1_000_000 = 400_000 lamports
        assert_eq!(compute_priority_fee(&info), 400_000);
        assert_eq!(compute_total_fee(&info), 405_000);
    }

    #[test]
    fn test_last_instruction_wins() {
        let info = make_info(
            vec![
                ComputeBudgetInstruction::SetComputeUnitLimit(100_000),
                ComputeBudgetInstruction::SetComputeUnitLimit(50_000),
                ComputeBudgetInstruction::SetComputeUnitPrice(500),
                ComputeBudgetInstruction::SetComputeUnitPrice(2_000),
            ],
            5000,
        );
        assert_eq!(extract_compute_unit_limit(&info), 50_000);
        assert_eq!(extract_compute_unit_price(&info), 2_000);
        // 50_000 * 2_000 / 1_000_000 = 100
        assert_eq!(compute_priority_fee(&info), 100);
    }

    #[test]
    fn test_integer_division_floor() {
        // 200_000 * 3 / 1_000_000 = 0.6 -> floor to 0
        let info = make_info(
            vec![ComputeBudgetInstruction::SetComputeUnitPrice(3)],
            5000,
        );
        assert_eq!(compute_priority_fee(&info), 0);
    }

    #[test]
    fn test_math_precision_large_values() {
        // Use large values that would overflow u64 multiplication
        // 4_000_000 CU * 5_000_000_000 micro-lamports = 20_000_000_000_000_000
        // / 1_000_000 = 20_000_000_000 lamports
        let info = make_info(
            vec![
                ComputeBudgetInstruction::SetComputeUnitLimit(4_000_000),
                ComputeBudgetInstruction::SetComputeUnitPrice(5_000_000_000),
            ],
            5000,
        );
        assert_eq!(compute_priority_fee(&info), 20_000_000_000);
    }

    #[test]
    fn test_zero_fee_transaction() {
        let info = make_info(
            vec![ComputeBudgetInstruction::SetComputeUnitPrice(0)],
            5000,
        );
        assert_eq!(compute_priority_fee(&info), 0);
        assert_eq!(compute_total_fee(&info), 5000);
    }

    #[test]
    fn test_rank_transactions_descending() {
        let low = make_info(vec![ComputeBudgetInstruction::SetComputeUnitPrice(100)], 5000);
        let high = make_info(vec![ComputeBudgetInstruction::SetComputeUnitPrice(5_000_000)], 5000);
        let mid = make_info(vec![ComputeBudgetInstruction::SetComputeUnitPrice(1_000_000)], 5000);

        let ranked = rank_transactions(&[low, high, mid]);
        let fees: Vec<u64> = ranked.iter().map(|t| compute_priority_fee(t)).collect();
        assert!(fees[0] >= fees[1] && fees[1] >= fees[2]);
    }

    #[test]
    fn test_rank_preserves_order_on_tie() {
        let a = make_info(vec![ComputeBudgetInstruction::SetComputeUnitPrice(100)], 5000);
        let b = make_info(vec![ComputeBudgetInstruction::SetComputeUnitPrice(100)], 5000);
        let ranked = rank_transactions(&[a.clone(), b.clone()]);
        // Both have the same fee; the original order should be preserved.
        assert_eq!(ranked.len(), 2);
    }

    #[test]
    fn test_effective_compute_price() {
        let info = make_info(
            vec![
                ComputeBudgetInstruction::SetComputeUnitLimit(200_000),
                ComputeBudgetInstruction::SetComputeUnitPrice(5_000),
            ],
            5000,
        );
        // priority_fee = 200_000 * 5_000 / 1_000_000 = 1_000
        // effective_price = 1_000 * 1_000_000 / 200_000 = 5_000
        assert_eq!(effective_compute_price(&info), 5_000);
    }

    #[test]
    fn test_effective_price_zero_units() {
        let info = make_info(
            vec![ComputeBudgetInstruction::SetComputeUnitLimit(0)],
            5000,
        );
        assert_eq!(effective_compute_price(&info), 0);
    }
}
