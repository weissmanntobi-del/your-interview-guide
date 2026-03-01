/// Challenge 03 - Order Book with Price-Time Priority Matching
///
/// Implement a limit order book that matches orders using price-time priority:
/// - Bids (buy orders) are sorted high-to-low by price, then FIFO at same price.
/// - Asks (sell orders) are sorted low-to-high by price, then FIFO at same price.
/// - A new order "crosses" the book when a bid >= best ask, or ask <= best bid.

use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub owner: [u8; 32],
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    pub maker_id: u64,
    pub taker_id: u64,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderBookError {
    OrderNotFound,
    ZeroQuantity,
    ZeroPrice,
}

/// A price-time priority order book.
/// Bids: BTreeMap<price, VecDeque<Order>> (iterated high to low).
/// Asks: BTreeMap<price, VecDeque<Order>> (iterated low to high).
pub struct OrderBook {
    pub bids: BTreeMap<u64, VecDeque<Order>>,
    pub asks: BTreeMap<u64, VecDeque<Order>>,
    next_id: u64,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Place an order. Returns a list of fills if the order crosses existing orders.
    /// Any remaining quantity rests on the book.
    pub fn place_order(
        &mut self,
        owner: [u8; 32],
        side: Side,
        price: u64,
        quantity: u64,
        timestamp: u64,
    ) -> Result<Vec<Fill>, OrderBookError> {
        // TODO: Implement order placement and matching
        // 1. Validate price > 0 and quantity > 0
        // 2. Create the order with auto-incrementing ID
        // 3. Try to match against the opposite side:
        //    - For a Bid: match against asks where ask_price <= bid_price (lowest ask first)
        //    - For an Ask: match against bids where bid_price >= ask_price (highest bid first)
        // 4. For each match, fill at the maker's price, deducting from both quantities
        // 5. Remove fully-filled maker orders
        // 6. If remaining quantity > 0, insert as resting order
        // 7. Return all fills
        todo!("Implement place_order")
    }

    /// Cancel an order by ID. Returns Err if not found.
    pub fn cancel_order(&mut self, order_id: u64) -> Result<Order, OrderBookError> {
        // TODO: Search both sides for the order, remove it
        todo!("Implement cancel_order")
    }

    /// Best bid price (highest buy order), or None if no bids.
    pub fn best_bid(&self) -> Option<u64> {
        // TODO: Return the highest key in bids that has a non-empty queue
        todo!("Implement best_bid")
    }

    /// Best ask price (lowest sell order), or None if no asks.
    pub fn best_ask(&self) -> Option<u64> {
        // TODO: Return the lowest key in asks that has a non-empty queue
        todo!("Implement best_ask")
    }

    /// Spread = best_ask - best_bid, or None if either side is empty.
    pub fn spread(&self) -> Option<u64> {
        // TODO: Compute spread from best_bid and best_ask
        todo!("Implement spread")
    }

    /// Return the top `levels` price levels for the given side.
    /// Each entry is (price, total_quantity_at_that_price).
    /// Bids: highest first. Asks: lowest first.
    pub fn depth(&self, side: Side, levels: usize) -> Vec<(u64, u64)> {
        // TODO: Aggregate quantities per price level
        todo!("Implement depth")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alice() -> [u8; 32] { [1u8; 32] }
    fn bob() -> [u8; 32] { [2u8; 32] }

    #[test]
    fn test_crossing_order_fills() {
        let mut book = OrderBook::new();
        // Alice places ask at 100
        book.place_order(alice(), Side::Ask, 100, 50, 1).unwrap();
        // Bob places bid at 100 => should match
        let fills = book.place_order(bob(), Side::Bid, 100, 50, 2).unwrap();
        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].price, 100);
        assert_eq!(fills[0].quantity, 50);
    }

    #[test]
    fn test_resting_when_no_match() {
        let mut book = OrderBook::new();
        // Bid at 90, Ask at 100 => no match
        let fills = book.place_order(alice(), Side::Bid, 90, 50, 1).unwrap();
        assert!(fills.is_empty());
        let fills = book.place_order(bob(), Side::Ask, 100, 50, 2).unwrap();
        assert!(fills.is_empty());
        assert_eq!(book.best_bid(), Some(90));
        assert_eq!(book.best_ask(), Some(100));
        assert_eq!(book.spread(), Some(10));
    }

    #[test]
    fn test_partial_fill() {
        let mut book = OrderBook::new();
        book.place_order(alice(), Side::Ask, 100, 100, 1).unwrap();
        let fills = book.place_order(bob(), Side::Bid, 100, 60, 2).unwrap();
        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].quantity, 60);
        // 40 remaining on the ask side
        assert_eq!(book.best_ask(), Some(100));
        let depth = book.depth(Side::Ask, 1);
        assert_eq!(depth[0], (100, 40));
    }

    #[test]
    fn test_price_time_priority() {
        let mut book = OrderBook::new();
        // Two asks at price 100, first Alice then Bob
        let f1 = book.place_order(alice(), Side::Ask, 100, 50, 1).unwrap();
        let f2 = book.place_order(bob(), Side::Ask, 100, 50, 2).unwrap();
        assert!(f1.is_empty());
        assert!(f2.is_empty());
        // Charlie buys 70 at 100 => should fill Alice's 50 first, then 20 from Bob
        let charlie = [3u8; 32];
        let fills = book.place_order(charlie, Side::Bid, 100, 70, 3).unwrap();
        assert_eq!(fills.len(), 2);
        assert_eq!(fills[0].quantity, 50); // Alice's full order
        assert_eq!(fills[1].quantity, 20); // Partial from Bob
    }

    #[test]
    fn test_cancel_order() {
        let mut book = OrderBook::new();
        book.place_order(alice(), Side::Bid, 90, 50, 1).unwrap();
        // Cancel order ID 1
        let cancelled = book.cancel_order(1).unwrap();
        assert_eq!(cancelled.price, 90);
        assert_eq!(book.best_bid(), None);
    }

    #[test]
    fn test_cancel_not_found() {
        let mut book = OrderBook::new();
        assert_eq!(book.cancel_order(999), Err(OrderBookError::OrderNotFound));
    }

    #[test]
    fn test_spread_calculation() {
        let mut book = OrderBook::new();
        book.place_order(alice(), Side::Bid, 95, 10, 1).unwrap();
        book.place_order(bob(), Side::Ask, 105, 10, 2).unwrap();
        assert_eq!(book.spread(), Some(10));
    }

    #[test]
    fn test_zero_quantity_rejected() {
        let mut book = OrderBook::new();
        let result = book.place_order(alice(), Side::Bid, 100, 0, 1);
        assert_eq!(result, Err(OrderBookError::ZeroQuantity));
    }
}
