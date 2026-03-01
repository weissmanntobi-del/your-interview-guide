//! # Challenge 2.4: Priority Queue (Min-Heap)
//!
//! Implement a min-heap priority queue for transaction fee ordering.
//! Used in mempool implementations to process highest-fee transactions first.
//!
//! Time: 40 min | Difficulty: Medium

pub struct PriorityQueue<T> {
    _placeholder: std::marker::PhantomData<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self { todo!() }
    pub fn push(&mut self, _item: T) { todo!() }
    pub fn pop(&mut self) -> Option<T> { todo!() }
    pub fn peek(&self) -> Option<&T> { todo!() }
    pub fn len(&self) -> usize { todo!() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap_ordering() {
        let mut pq = PriorityQueue::new();
        pq.push(5);
        pq.push(1);
        pq.push(3);
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), Some(5));
    }

    #[test]
    fn test_empty() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        assert!(pq.is_empty());
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut pq = PriorityQueue::new();
        pq.push(3);
        pq.push(1);
        assert_eq!(pq.peek(), Some(&1));
        assert_eq!(pq.len(), 2); // peek doesn't remove
    }
}
