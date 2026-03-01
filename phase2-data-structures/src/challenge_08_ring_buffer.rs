//! # Challenge 2.8: Ring Buffer — Fixed-size circular buffer. Time: 30 min | Easy
pub struct RingBuffer<T> { _p: std::marker::PhantomData<T> }
impl<T> RingBuffer<T> {
    pub fn new(_capacity: usize) -> Self { todo!() }
    pub fn push(&mut self, _item: T) { todo!("Overwrite oldest if full") }
    pub fn iter(&self) -> impl Iterator<Item = &T> { std::iter::empty() }
    pub fn len(&self) -> usize { todo!() }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_overwrite() { let mut rb = RingBuffer::new(3); rb.push(1); rb.push(2); rb.push(3); rb.push(4); // overwrites 1
        let items: Vec<_> = rb.iter().copied().collect(); assert_eq!(items, vec![2, 3, 4]); }
}
