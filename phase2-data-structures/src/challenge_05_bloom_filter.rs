//! # Challenge 2.5: Bloom Filter
//!
//! Implement a probabilistic set membership filter. Used in Ethereum for
//! log bloom filters in block headers and receipt lookups.
//!
//! Time: 30 min | Difficulty: Medium

pub struct BloomFilter {
    _placeholder: (),
}

impl BloomFilter {
    pub fn new(_size_bits: usize, _num_hashes: u32) -> Self { todo!() }
    pub fn insert(&mut self, _item: &[u8]) { todo!() }
    pub fn contains(&self, _item: &[u8]) -> bool { todo!() }
    pub fn false_positive_rate(&self) -> f64 { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inserted_items_found() {
        let mut bf = BloomFilter::new(1024, 3);
        bf.insert(b"hello");
        bf.insert(b"world");
        assert!(bf.contains(b"hello"));
        assert!(bf.contains(b"world"));
    }

    #[test]
    fn test_missing_items_usually_not_found() {
        let mut bf = BloomFilter::new(8192, 5);
        for i in 0..100u32 {
            bf.insert(&i.to_le_bytes());
        }
        let mut false_positives = 0;
        for i in 1000..2000u32 {
            if bf.contains(&i.to_le_bytes()) {
                false_positives += 1;
            }
        }
        // With 8192 bits and 5 hashes, FP rate should be very low
        assert!(false_positives < 50, "too many false positives: {}", false_positives);
    }
}
