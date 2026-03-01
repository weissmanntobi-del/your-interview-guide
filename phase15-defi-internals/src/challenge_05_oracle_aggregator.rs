/// Challenge 05 - Oracle Aggregator with TWAP and Staleness Detection
///
/// Aggregate price feeds from multiple sources. Use the median for robustness.
/// Filter out stale feeds (older than max_staleness_secs) and halted feeds.
/// Compute a time-weighted average price (TWAP) over a configurable window.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FeedStatus {
    Trading,
    Halted,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct PriceFeed {
    pub source: String,
    pub price: u64,        // price in smallest unit (e.g., cents)
    pub confidence: u64,   // confidence interval
    pub timestamp: u64,    // unix timestamp
    pub status: FeedStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggregatedPrice {
    pub price: u64,
    pub confidence: u64,
    pub num_sources: usize,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OracleError {
    InsufficientSources,
    AllFeedsStale,
    NoFeeds,
}

pub struct OracleAggregator {
    pub max_staleness_secs: u64,
    pub min_sources: usize,
    feeds: HashMap<String, PriceFeed>,
    price_history: Vec<(u64, u64)>, // (timestamp, price) for TWAP
}

impl OracleAggregator {
    pub fn new(max_staleness_secs: u64, min_sources: usize) -> Self {
        OracleAggregator {
            max_staleness_secs,
            min_sources,
            feeds: HashMap::new(),
            price_history: Vec::new(),
        }
    }

    /// Update or insert a price feed from a named source.
    pub fn update_feed(&mut self, feed: PriceFeed) {
        // TODO: Store the feed by source name
        // Also push (timestamp, price) to price_history for TWAP
        todo!("Implement update_feed")
    }

    /// Get the aggregated price at the given current_time.
    /// 1. Filter out stale feeds (timestamp + max_staleness_secs < current_time)
    /// 2. Filter out feeds with status != Trading
    /// 3. If fewer than min_sources remain, return InsufficientSources
    /// 4. Compute median price
    /// 5. Confidence = max absolute deviation from median among valid feeds
    pub fn get_price(&self, current_time: u64) -> Result<AggregatedPrice, OracleError> {
        // TODO: Implement aggregation
        // 1. Collect valid feeds (not stale, status == Trading)
        // 2. Check min_sources
        // 3. Sort prices, take median
        // 4. Compute confidence as max |price - median|
        // 5. Return AggregatedPrice with latest timestamp among valid feeds
        todo!("Implement get_price")
    }

    /// Compute time-weighted average price over the last `window` seconds.
    /// Uses price_history entries within [current_time - window, current_time].
    pub fn get_twap(&self, current_time: u64, window: u64) -> Result<u64, OracleError> {
        // TODO: Implement TWAP
        // 1. Filter price_history to entries in the time window
        // 2. If no entries, return NoFeeds
        // 3. For each consecutive pair, weight the price by time duration
        // 4. TWAP = sum(price_i * duration_i) / total_duration
        // Simplified: just average the prices in the window if durations are tricky
        todo!("Implement get_twap")
    }

    /// Check if any feed is stale at the given time.
    pub fn is_stale(&self, source: &str, current_time: u64) -> bool {
        // TODO: Return true if feed's timestamp + max_staleness_secs < current_time
        todo!("Implement is_stale")
    }
}

fn median(sorted: &[u64]) -> u64 {
    let len = sorted.len();
    if len == 0 {
        return 0;
    }
    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_feed(source: &str, price: u64, ts: u64) -> PriceFeed {
        PriceFeed {
            source: source.to_string(),
            price,
            confidence: 10,
            timestamp: ts,
            status: FeedStatus::Trading,
        }
    }

    #[test]
    fn test_median_aggregation() {
        let mut oracle = OracleAggregator::new(60, 2);
        oracle.update_feed(make_feed("a", 1000, 100));
        oracle.update_feed(make_feed("b", 1020, 100));
        oracle.update_feed(make_feed("c", 1010, 100));
        let agg = oracle.get_price(110).unwrap();
        assert_eq!(agg.price, 1010); // median of [1000, 1010, 1020]
        assert_eq!(agg.num_sources, 3);
    }

    #[test]
    fn test_stale_excluded() {
        let mut oracle = OracleAggregator::new(60, 2);
        oracle.update_feed(make_feed("a", 1000, 50));  // stale at t=200
        oracle.update_feed(make_feed("b", 1020, 180));
        oracle.update_feed(make_feed("c", 1010, 180));
        let agg = oracle.get_price(200).unwrap();
        assert_eq!(agg.num_sources, 2); // "a" excluded as stale
    }

    #[test]
    fn test_insufficient_sources() {
        let mut oracle = OracleAggregator::new(60, 3);
        oracle.update_feed(make_feed("a", 1000, 100));
        oracle.update_feed(make_feed("b", 1020, 100));
        let result = oracle.get_price(110);
        assert_eq!(result, Err(OracleError::InsufficientSources));
    }

    #[test]
    fn test_twap() {
        let mut oracle = OracleAggregator::new(600, 1);
        oracle.update_feed(make_feed("a", 1000, 100));
        oracle.update_feed(make_feed("a", 1100, 200));
        oracle.update_feed(make_feed("a", 1200, 300));
        let twap = oracle.get_twap(350, 300).unwrap();
        // Average of prices in [50, 350]: 1000, 1100, 1200
        assert!(twap >= 1000 && twap <= 1200, "TWAP should be between min and max prices, got {}", twap);
    }

    #[test]
    fn test_halted_excluded() {
        let mut oracle = OracleAggregator::new(60, 2);
        oracle.update_feed(make_feed("a", 1000, 100));
        oracle.update_feed(make_feed("b", 1020, 100));
        oracle.update_feed(PriceFeed {
            source: "c".to_string(),
            price: 9999,
            confidence: 10,
            timestamp: 100,
            status: FeedStatus::Halted,
        });
        let agg = oracle.get_price(110).unwrap();
        assert_eq!(agg.num_sources, 2); // "c" excluded
        // Median of [1000, 1020]
        assert_eq!(agg.price, 1010);
    }

    #[test]
    fn test_is_stale() {
        let mut oracle = OracleAggregator::new(60, 1);
        oracle.update_feed(make_feed("a", 1000, 100));
        assert!(!oracle.is_stale("a", 150));
        assert!(oracle.is_stale("a", 200)); // 100 + 60 < 200
    }

    #[test]
    fn test_no_feeds() {
        let oracle = OracleAggregator::new(60, 1);
        let result = oracle.get_price(100);
        assert!(result.is_err());
    }
}
