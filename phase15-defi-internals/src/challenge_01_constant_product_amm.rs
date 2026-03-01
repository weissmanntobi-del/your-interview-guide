/// Challenge 01 - Constant Product AMM (x * y = k)
///
/// Implement a constant-product automated market maker (like Uniswap V2).
/// The invariant is: reserve_a * reserve_b = k (constant) before and after every swap.
///
/// Swap formula (fee-adjusted):
///   amount_out = (amount_in * fee_factor * reserve_out) / (reserve_in + amount_in * fee_factor)
///   where fee_factor = (10000 - fee_bps)
///
/// Liquidity providers deposit both tokens and receive LP tokens proportional to their share.

#[derive(Debug, Clone)]
pub struct Pool {
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_bps: u16,
    pub lp_total_supply: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwapResult {
    pub amount_out: u64,
    pub fee: u64,
    pub price_impact_bps: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AmmError {
    InsufficientLiquidity,
    SlippageExceeded,
    ZeroAmount,
}

/// Swap token A for token B.
/// `min_out` is the minimum acceptable output (slippage protection).
/// Returns a SwapResult on success.
pub fn swap_a_to_b(pool: &mut Pool, amount_in: u64, min_out: u64) -> Result<SwapResult, AmmError> {
    // TODO: Implement constant product swap
    // 1. Check amount_in > 0
    // 2. Compute fee = amount_in * fee_bps / 10000
    // 3. Compute amount_in_after_fee = amount_in - fee
    // 4. Compute amount_out using: (amount_in_after_fee * reserve_b) / (reserve_a + amount_in_after_fee)
    // 5. Check amount_out <= reserve_b (InsufficientLiquidity)
    // 6. Check amount_out >= min_out (SlippageExceeded)
    // 7. Compute price_impact_bps from the price change
    // 8. Update pool reserves
    // 9. Return SwapResult
    todo!("Implement swap_a_to_b")
}

/// Add liquidity to the pool.
/// Returns the number of LP tokens minted.
/// If pool is empty (first deposit), LP tokens = sqrt(amount_a * amount_b).
/// Otherwise, LP tokens = min(amount_a / reserve_a, amount_b / reserve_b) * lp_total_supply.
pub fn add_liquidity(pool: &mut Pool, amount_a: u64, amount_b: u64) -> Result<u64, AmmError> {
    // TODO: Implement add_liquidity
    // 1. Check both amounts > 0
    // 2. If first deposit, mint sqrt(a * b) LP tokens
    // 3. Otherwise compute proportional LP tokens
    // 4. Update reserves and lp_total_supply
    // 5. Return LP tokens minted
    todo!("Implement add_liquidity")
}

/// Remove liquidity by burning LP tokens.
/// Returns (amount_a, amount_b) withdrawn.
pub fn remove_liquidity(pool: &mut Pool, lp_tokens: u64) -> Result<(u64, u64), AmmError> {
    // TODO: Implement remove_liquidity
    // 1. Check lp_tokens > 0
    // 2. Compute share = lp_tokens / lp_total_supply
    // 3. amount_a = reserve_a * share, amount_b = reserve_b * share
    // 4. Update reserves and lp_total_supply
    // 5. Return amounts
    todo!("Implement remove_liquidity")
}

/// Get current prices: (price_a_in_b, price_b_in_a).
pub fn get_price(pool: &Pool) -> (f64, f64) {
    // TODO: price_a_in_b = reserve_b / reserve_a, price_b_in_a = reserve_a / reserve_b
    todo!("Implement get_price")
}

fn integer_sqrt(n: u128) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_pool() -> Pool {
        let mut pool = Pool {
            reserve_a: 0,
            reserve_b: 0,
            fee_bps: 30, // 0.3%
            lp_total_supply: 0,
        };
        add_liquidity(&mut pool, 1_000_000, 1_000_000).unwrap();
        pool
    }

    #[test]
    fn test_swap_preserves_k() {
        let mut pool = setup_pool();
        let k_before = pool.reserve_a as u128 * pool.reserve_b as u128;
        swap_a_to_b(&mut pool, 10_000, 0).unwrap();
        let k_after = pool.reserve_a as u128 * pool.reserve_b as u128;
        // k should increase or stay same (fees make it grow)
        assert!(k_after >= k_before, "k must not decrease after swap");
    }

    #[test]
    fn test_fee_deducted() {
        let mut pool = setup_pool();
        let result = swap_a_to_b(&mut pool, 10_000, 0).unwrap();
        assert!(result.fee > 0, "Fee should be > 0");
        // With 30bps fee on 10_000 input, fee = 10_000 * 30 / 10_000 = 30
        assert_eq!(result.fee, 30);
    }

    #[test]
    fn test_slippage_rejection() {
        let mut pool = setup_pool();
        // Ask for more out than possible
        let result = swap_a_to_b(&mut pool, 10_000, 999_999);
        assert_eq!(result, Err(AmmError::SlippageExceeded));
    }

    #[test]
    fn test_add_remove_liquidity() {
        let mut pool = setup_pool();
        let initial_a = pool.reserve_a;
        let initial_b = pool.reserve_b;
        let lp = add_liquidity(&mut pool, 500_000, 500_000).unwrap();
        assert!(lp > 0);
        let (got_a, got_b) = remove_liquidity(&mut pool, lp).unwrap();
        assert_eq!(got_a, 500_000);
        assert_eq!(got_b, 500_000);
        assert_eq!(pool.reserve_a, initial_a);
        assert_eq!(pool.reserve_b, initial_b);
    }

    #[test]
    fn test_zero_amount() {
        let mut pool = setup_pool();
        assert_eq!(swap_a_to_b(&mut pool, 0, 0), Err(AmmError::ZeroAmount));
        assert_eq!(add_liquidity(&mut pool, 0, 100), Err(AmmError::ZeroAmount));
        assert_eq!(remove_liquidity(&mut pool, 0), Err(AmmError::ZeroAmount));
    }

    #[test]
    fn test_get_price() {
        let pool = Pool {
            reserve_a: 1_000_000,
            reserve_b: 2_000_000,
            fee_bps: 30,
            lp_total_supply: 1_000,
        };
        let (a_in_b, b_in_a) = get_price(&pool);
        assert!((a_in_b - 2.0).abs() < 0.001);
        assert!((b_in_a - 0.5).abs() < 0.001);
    }
}
