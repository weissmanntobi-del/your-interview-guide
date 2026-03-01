/// Challenge 04 - Liquidation Engine
///
/// Implement a collateral monitoring and liquidation system for a lending protocol.
/// Each position has collateral and debt denominated in different tokens.
/// The health factor determines if a position is safe or liquidatable.
///
/// health_factor = (collateral_value * 10000) / (debt_value * liquidation_threshold_bps / 10000)
/// A position is healthy if health_factor >= 10000 (i.e., 1.0 in basis points).

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Position {
    pub owner: [u8; 32],
    pub collateral: u64,          // amount of collateral token
    pub debt: u64,                // amount of debt token
    pub collateral_mint: [u8; 32],
    pub debt_mint: [u8; 32],
}

/// Maps token mint -> price in cents (e.g., 100_00 = $100.00)
pub type PriceOracle = HashMap<[u8; 32], u64>;

#[derive(Debug, Clone)]
pub struct LiquidationParams {
    pub liquidation_threshold_bps: u16,  // e.g., 8000 = 80% LTV triggers liquidation
    pub liquidation_bonus_bps: u16,      // e.g., 500 = 5% bonus for liquidator
    pub max_liquidation_pct: u16,        // e.g., 5000 = can liquidate up to 50%
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiquidationResult {
    pub debt_repaid: u64,
    pub collateral_seized: u64,
    pub bonus_amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiquidationError {
    PositionHealthy,
    ExceedsMaxLiquidation,
    InsufficientCollateral,
    PriceNotAvailable,
    ZeroAmount,
}

/// Compute the health factor of a position (in basis points, 10000 = 1.0).
/// health_factor = (collateral_value * 10000) / (debt_value * threshold_bps / 10000)
/// Returns 10000+ if healthy, <10000 if undercollateralized.
pub fn health_factor(
    position: &Position,
    prices: &PriceOracle,
    params: &LiquidationParams,
) -> Result<u64, LiquidationError> {
    // TODO: Implement health factor calculation
    // 1. Look up prices for both mints (return PriceNotAvailable if missing)
    // 2. collateral_value = position.collateral * collateral_price
    // 3. debt_value = position.debt * debt_price
    // 4. If debt_value == 0, return u64::MAX (no debt = infinitely healthy)
    // 5. health = collateral_value * 10000 * 10000 / (debt_value * threshold_bps)
    // 6. Return health
    todo!("Implement health_factor")
}

/// Check if a position can be liquidated (health_factor < 10000).
pub fn is_liquidatable(
    position: &Position,
    prices: &PriceOracle,
    params: &LiquidationParams,
) -> Result<bool, LiquidationError> {
    // TODO: Return true if health_factor < 10000
    todo!("Implement is_liquidatable")
}

/// Execute a liquidation: repay `amount` of debt and seize collateral + bonus.
pub fn liquidate(
    position: &mut Position,
    amount: u64,
    prices: &PriceOracle,
    params: &LiquidationParams,
) -> Result<LiquidationResult, LiquidationError> {
    // TODO: Implement liquidation
    // 1. Check amount > 0
    // 2. Check position is liquidatable
    // 3. Check amount <= position.debt * max_liquidation_pct / 10000
    // 4. Compute collateral_to_seize based on debt_price/collateral_price ratio
    // 5. Add liquidation bonus: bonus = collateral_to_seize * liquidation_bonus_bps / 10000
    // 6. Total seized = collateral_to_seize + bonus
    // 7. Check position has enough collateral
    // 8. Update position: reduce debt by amount, reduce collateral by total seized
    // 9. Return LiquidationResult
    todo!("Implement liquidate")
}

/// Find indices of all liquidatable positions in a list.
pub fn find_liquidatable_positions(
    positions: &[Position],
    prices: &PriceOracle,
    params: &LiquidationParams,
) -> Vec<usize> {
    // TODO: Return indices where is_liquidatable returns Ok(true)
    todo!("Implement find_liquidatable_positions")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usdc_mint() -> [u8; 32] { [1u8; 32] }
    fn sol_mint() -> [u8; 32] { [2u8; 32] }

    fn setup_prices(sol_price_cents: u64) -> PriceOracle {
        let mut prices = HashMap::new();
        prices.insert(usdc_mint(), 100); // $1.00
        prices.insert(sol_mint(), sol_price_cents);
        prices
    }

    fn default_params() -> LiquidationParams {
        LiquidationParams {
            liquidation_threshold_bps: 8000,
            liquidation_bonus_bps: 500,
            max_liquidation_pct: 5000,
        }
    }

    #[test]
    fn test_healthy_position() {
        let position = Position {
            owner: [0u8; 32],
            collateral: 10,              // 10 SOL
            debt: 500,                   // 500 USDC
            collateral_mint: sol_mint(),
            debt_mint: usdc_mint(),
        };
        let prices = setup_prices(10000); // SOL = $100
        let params = default_params();
        // collateral_value=1000, debt_value=500, threshold=80%
        // health = (1000 * 10000 * 10000) / (500 * 8000) = 25000
        let hf = health_factor(&position, &prices, &params).unwrap();
        assert!(hf >= 10000, "Position should be healthy, got hf={}", hf);
        assert!(!is_liquidatable(&position, &prices, &params).unwrap());
    }

    #[test]
    fn test_underwater_position() {
        let position = Position {
            owner: [0u8; 32],
            collateral: 10,
            debt: 1200,
            collateral_mint: sol_mint(),
            debt_mint: usdc_mint(),
        };
        let prices = setup_prices(10000); // SOL = $100
        let params = default_params();
        let hf = health_factor(&position, &prices, &params).unwrap();
        assert!(hf < 10000, "Position should be underwater, got hf={}", hf);
        assert!(is_liquidatable(&position, &prices, &params).unwrap());
    }

    #[test]
    fn test_liquidation_seizes_collateral() {
        let mut position = Position {
            owner: [0u8; 32],
            collateral: 100,
            debt: 9000,
            collateral_mint: sol_mint(),
            debt_mint: usdc_mint(),
        };
        let prices = setup_prices(10000);
        let params = default_params();
        let result = liquidate(&mut position, 1000, &prices, &params).unwrap();
        assert_eq!(result.debt_repaid, 1000);
        assert!(result.collateral_seized > 0);
        assert!(result.bonus_amount > 0);
        assert_eq!(position.debt, 8000);
    }

    #[test]
    fn test_max_liquidation_enforced() {
        let mut position = Position {
            owner: [0u8; 32],
            collateral: 100,
            debt: 9000,
            collateral_mint: sol_mint(),
            debt_mint: usdc_mint(),
        };
        let prices = setup_prices(10000);
        let params = default_params();
        // Try to liquidate more than 50% of debt
        let result = liquidate(&mut position, 5000, &prices, &params);
        assert_eq!(result, Err(LiquidationError::ExceedsMaxLiquidation));
    }

    #[test]
    fn test_price_change_triggers_liquidation() {
        let position = Position {
            owner: [0u8; 32],
            collateral: 10,
            debt: 800,
            collateral_mint: sol_mint(),
            debt_mint: usdc_mint(),
        };
        let params = default_params();
        // At $100, healthy
        let high_prices = setup_prices(10000);
        assert!(!is_liquidatable(&position, &high_prices, &params).unwrap());
        // At $50, liquidatable
        let low_prices = setup_prices(5000);
        assert!(is_liquidatable(&position, &low_prices, &params).unwrap());
    }

    #[test]
    fn test_find_liquidatable() {
        let positions = vec![
            Position { owner: [1u8; 32], collateral: 100, debt: 500, collateral_mint: sol_mint(), debt_mint: usdc_mint() },
            Position { owner: [2u8; 32], collateral: 5, debt: 800, collateral_mint: sol_mint(), debt_mint: usdc_mint() },
            Position { owner: [3u8; 32], collateral: 3, debt: 500, collateral_mint: sol_mint(), debt_mint: usdc_mint() },
        ];
        let prices = setup_prices(10000);
        let params = default_params();
        let liquidatable = find_liquidatable_positions(&positions, &prices, &params);
        // Position 1 (idx=1) and 2 (idx=2) should be liquidatable
        assert!(liquidatable.contains(&1));
        assert!(liquidatable.contains(&2));
        assert!(!liquidatable.contains(&0));
    }
}
