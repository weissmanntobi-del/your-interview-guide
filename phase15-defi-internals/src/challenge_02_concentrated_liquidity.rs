/// Challenge 02 - Concentrated Liquidity Market Maker (CLMM)
///
/// Implement a tick-based concentrated liquidity pool (like Uniswap V3).
/// Liquidity providers choose a price range [lower_tick, upper_tick] for their liquidity.
/// Only active liquidity (where current tick is within range) participates in swaps.
///
/// Ticks represent discrete price points: price = 1.0001^tick
/// sqrt_price = 1.0001^(tick/2), stored as Q64.64 fixed-point (scaled by 2^64).

#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    pub index: i32,
    pub liquidity_net: i128, // positive when entering range, negative when leaving
}

#[derive(Debug, Clone)]
pub struct Position {
    pub owner: [u8; 32],
    pub lower_tick: i32,
    pub upper_tick: i32,
    pub liquidity: u128,
}

#[derive(Debug, Clone)]
pub struct PoolState {
    pub sqrt_price: u128,   // Q64.64 fixed-point sqrt(price)
    pub current_tick: i32,
    pub liquidity: u128,    // active liquidity at current tick
    pub positions: Vec<Position>,
    pub ticks: Vec<Tick>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClmmError {
    InvalidTickRange,
    InsufficientLiquidity,
    ZeroLiquidity,
    PositionNotFound,
}

const Q64: u128 = 1u128 << 64;

/// Convert a tick index to a sqrt_price in Q64.64 fixed-point.
/// sqrt_price = 1.0001^(tick/2) * 2^64
pub fn tick_to_sqrt_price(tick: i32) -> u128 {
    // TODO: Implement tick -> sqrt_price conversion
    // Use: price = 1.0001^tick, sqrt_price = price.sqrt() * Q64
    // Hint: compute in f64 then convert to u128
    todo!("Implement tick_to_sqrt_price")
}

/// Convert a sqrt_price (Q64.64) back to the nearest tick index.
pub fn sqrt_price_to_tick(sqrt_price: u128) -> i32 {
    // TODO: Reverse of tick_to_sqrt_price
    // price = (sqrt_price / 2^64)^2
    // tick = log(price) / log(1.0001)
    todo!("Implement sqrt_price_to_tick")
}

impl PoolState {
    pub fn new(initial_tick: i32) -> Self {
        PoolState {
            sqrt_price: tick_to_sqrt_price(initial_tick),
            current_tick: initial_tick,
            liquidity: 0,
            positions: Vec::new(),
            ticks: Vec::new(),
        }
    }

    /// Add a concentrated liquidity position in [lower_tick, upper_tick].
    /// Updates tick entries and active liquidity if current tick is in range.
    pub fn add_position(
        &mut self,
        owner: [u8; 32],
        lower_tick: i32,
        upper_tick: i32,
        liquidity: u128,
    ) -> Result<(), ClmmError> {
        // TODO: Implement add_position
        // 1. Validate lower_tick < upper_tick
        // 2. Validate liquidity > 0
        // 3. Update or insert tick at lower_tick with +liquidity_net
        // 4. Update or insert tick at upper_tick with -liquidity_net
        // 5. If current_tick is in [lower_tick, upper_tick), add to active liquidity
        // 6. Store the position
        todo!("Implement add_position")
    }

    /// Remove a position, reversing the tick and liquidity updates.
    pub fn remove_position(&mut self, index: usize) -> Result<(), ClmmError> {
        // TODO: Reverse add_position for the position at given index
        todo!("Implement remove_position")
    }

    /// Swap token A for token B (move price down).
    /// Returns amount_out. Crosses ticks as needed.
    pub fn swap(&mut self, amount_in: u128) -> Result<u128, ClmmError> {
        // TODO: Implement swap with tick crossing
        // 1. Consume liquidity at current tick
        // 2. When hitting next tick boundary, update liquidity with tick's liquidity_net
        // 3. Continue until amount_in is fully consumed
        // 4. Update sqrt_price and current_tick
        todo!("Implement swap")
    }

    fn find_or_insert_tick(&mut self, index: i32) -> &mut Tick {
        if let Some(pos) = self.ticks.iter().position(|t| t.index == index) {
            &mut self.ticks[pos]
        } else {
            self.ticks.push(Tick {
                index,
                liquidity_net: 0,
            });
            self.ticks.sort_by_key(|t| t.index);
            let pos = self.ticks.iter().position(|t| t.index == index).unwrap();
            &mut self.ticks[pos]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_conversion_roundtrip() {
        for tick in [-1000, -100, 0, 100, 1000, 5000] {
            let sqrt_p = tick_to_sqrt_price(tick);
            let recovered = sqrt_price_to_tick(sqrt_p);
            assert!(
                (recovered - tick).abs() <= 1,
                "tick {} -> sqrt_price {} -> tick {}, diff too large",
                tick, sqrt_p, recovered
            );
        }
    }

    #[test]
    fn test_tick_zero_is_one() {
        let sqrt_p = tick_to_sqrt_price(0);
        // At tick 0, price = 1.0, sqrt_price = 1.0 * Q64
        let diff = if sqrt_p > Q64 { sqrt_p - Q64 } else { Q64 - sqrt_p };
        assert!(diff < Q64 / 1000, "tick 0 should give sqrt_price near Q64");
    }

    #[test]
    fn test_add_position_updates_ticks() {
        let mut pool = PoolState::new(0);
        let owner = [1u8; 32];
        pool.add_position(owner, -100, 100, 1_000_000).unwrap();

        assert_eq!(pool.positions.len(), 1);
        assert!(pool.ticks.iter().any(|t| t.index == -100 && t.liquidity_net > 0));
        assert!(pool.ticks.iter().any(|t| t.index == 100 && t.liquidity_net < 0));
        // Current tick 0 is in range [-100, 100), so active liquidity updated
        assert_eq!(pool.liquidity, 1_000_000);
    }

    #[test]
    fn test_add_position_out_of_range() {
        let mut pool = PoolState::new(0);
        let owner = [2u8; 32];
        // Position entirely above current tick
        pool.add_position(owner, 100, 200, 500_000).unwrap();
        // Liquidity should NOT be active since current_tick=0 < 100
        assert_eq!(pool.liquidity, 0);
    }

    #[test]
    fn test_invalid_tick_range() {
        let mut pool = PoolState::new(0);
        let result = pool.add_position([3u8; 32], 100, -100, 1000);
        assert_eq!(result, Err(ClmmError::InvalidTickRange));
    }

    #[test]
    fn test_swap_within_tick() {
        let mut pool = PoolState::new(0);
        pool.add_position([1u8; 32], -1000, 1000, 10_000_000).unwrap();
        let out = pool.swap(1000).unwrap();
        assert!(out > 0, "Swap should produce output");
    }

    #[test]
    fn test_swap_crossing_tick() {
        let mut pool = PoolState::new(0);
        pool.add_position([1u8; 32], -100, 50, 5_000_000).unwrap();
        pool.add_position([2u8; 32], 50, 200, 3_000_000).unwrap();
        // Large swap should cross the tick at 50
        let out = pool.swap(100_000).unwrap();
        assert!(out > 0);
    }
}
