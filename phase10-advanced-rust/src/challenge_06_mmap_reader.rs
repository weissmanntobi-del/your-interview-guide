//! # Challenge 10.6: Memory-Mapped File Reader
//!
//! ## Problem
//! Simulate memory-mapped reading of a large ledger file. Parse fixed-format records
//! from a byte buffer without copying — return references that borrow from the buffer.
//!
//! ## Why This Matters
//! Geyser plugins and blockstore use memory-mapped I/O extensively in the Solana
//! validator. Reading gigabytes of ledger data efficiently requires zero-copy parsing
//! where records reference the mapped memory directly.
//!
//! ## Requirements
//! - `MmapReader::new(data: Vec<u8>)` — wrap a byte buffer (simulating mmap)
//! - Records are stored sequentially: [slot: u64][hash: [u8;32]][data_len: u32][data: &[u8]]
//! - `record_count(&self) -> usize`
//! - `get_record(index) -> Option<RecordRef>` where RecordRef borrows from the buffer
//! - `iter_records()` yields zero-copy record references
//! - `records_in_slot_range(start, end)` — range query
//!
//! ## Constraints
//! - RecordRef must borrow from the MmapReader's buffer (no copying)
//! - Handle malformed data gracefully (return None)
//! - Use unsafe to reinterpret bytes as record headers
//!
//! ## Hints
//! - Record header is slot(8) + hash(32) + data_len(4) = 44 bytes fixed
//! - Walk the buffer to build an offset index on construction
//! - Use `std::ptr::read_unaligned` or `from_le_bytes` for safe reading

/// A zero-copy reference to a record stored in the memory-mapped buffer.
#[derive(Debug)]
pub struct RecordRef<'a> {
    pub slot: u64,
    pub hash: &'a [u8; 32],
    pub data: &'a [u8],
}

/// Header size: slot(8) + hash(32) + data_len(4)
pub const HEADER_SIZE: usize = 44;

pub struct MmapReader {
    // TODO: implement fields
    // - data: Vec<u8> (the backing buffer)
    // - offsets: Vec<usize> (byte offset of each record)
    _placeholder: (),
}

impl MmapReader {
    /// Create a new reader from raw bytes. Scans the buffer to build
    /// an offset index of all valid records.
    pub fn new(_data: Vec<u8>) -> Self {
        todo!(
            "Store data, walk the buffer to find each record's starting offset. \
             Each record is: [slot:8][hash:32][data_len:4][data:data_len]"
        )
    }

    /// Number of valid records in the buffer.
    pub fn record_count(&self) -> usize {
        todo!()
    }

    /// Get a zero-copy reference to the record at the given index.
    pub fn get_record(&self, _index: usize) -> Option<RecordRef<'_>> {
        todo!("Parse record at offsets[index], return RecordRef borrowing from self.data")
    }

    /// Iterate over all records as zero-copy references.
    pub fn iter_records(&self) -> impl Iterator<Item = RecordRef<'_>> {
        todo!("Return an iterator that yields RecordRef for each record");
        // placeholder to make it compile:
        std::iter::empty()
    }

    /// Find all records with slot in [start, end) (exclusive end).
    pub fn records_in_slot_range(&self, _start: u64, _end: u64) -> Vec<RecordRef<'_>> {
        todo!("Filter records by slot range")
    }

    /// Total bytes in the backing buffer.
    pub fn buffer_size(&self) -> usize {
        todo!()
    }
}

/// Helper to build test data: serialize a record into bytes.
pub fn encode_record(slot: u64, hash: &[u8; 32], data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_SIZE + data.len());
    buf.extend_from_slice(&slot.to_le_bytes());
    buf.extend_from_slice(hash);
    buf.extend_from_slice(&(data.len() as u32).to_le_bytes());
    buf.extend_from_slice(data);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hash(byte: u8) -> [u8; 32] {
        [byte; 32]
    }

    fn build_buffer(records: &[(u64, u8, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        for &(slot, hash_byte, data) in records {
            buf.extend(encode_record(slot, &make_hash(hash_byte), data));
        }
        buf
    }

    #[test]
    fn test_single_record() {
        let buf = build_buffer(&[(1, 0xAA, &[10, 20, 30])]);
        let reader = MmapReader::new(buf);
        assert_eq!(reader.record_count(), 1);
        let rec = reader.get_record(0).unwrap();
        assert_eq!(rec.slot, 1);
        assert_eq!(rec.hash, &make_hash(0xAA));
        assert_eq!(rec.data, &[10, 20, 30]);
    }

    #[test]
    fn test_multiple_records() {
        let buf = build_buffer(&[
            (1, 0x01, &[1, 2]),
            (2, 0x02, &[3, 4, 5]),
            (3, 0x03, &[6]),
        ]);
        let reader = MmapReader::new(buf);
        assert_eq!(reader.record_count(), 3);
        assert_eq!(reader.get_record(0).unwrap().slot, 1);
        assert_eq!(reader.get_record(1).unwrap().slot, 2);
        assert_eq!(reader.get_record(2).unwrap().slot, 3);
    }

    #[test]
    fn test_out_of_bounds() {
        let buf = build_buffer(&[(1, 0x01, &[1])]);
        let reader = MmapReader::new(buf);
        assert!(reader.get_record(1).is_none());
        assert!(reader.get_record(100).is_none());
    }

    #[test]
    fn test_empty_buffer() {
        let reader = MmapReader::new(Vec::new());
        assert_eq!(reader.record_count(), 0);
        assert!(reader.get_record(0).is_none());
    }

    #[test]
    fn test_iteration() {
        let buf = build_buffer(&[(10, 0xAA, &[1]), (20, 0xBB, &[2]), (30, 0xCC, &[3])]);
        let reader = MmapReader::new(buf);
        let slots: Vec<u64> = reader.iter_records().map(|r| r.slot).collect();
        assert_eq!(slots, vec![10, 20, 30]);
    }

    #[test]
    fn test_slot_range_query() {
        let buf = build_buffer(&[
            (5, 0x01, &[1]),
            (10, 0x02, &[2]),
            (15, 0x03, &[3]),
            (20, 0x04, &[4]),
        ]);
        let reader = MmapReader::new(buf);
        let results = reader.records_in_slot_range(10, 20);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].slot, 10);
        assert_eq!(results[1].slot, 15);
    }

    #[test]
    fn test_empty_data_record() {
        let buf = build_buffer(&[(1, 0xFF, &[])]);
        let reader = MmapReader::new(buf);
        let rec = reader.get_record(0).unwrap();
        assert!(rec.data.is_empty());
    }

    #[test]
    fn test_large_data_record() {
        let data = vec![0xAB; 10_000];
        let buf = build_buffer(&[(42, 0x01, &data)]);
        let reader = MmapReader::new(buf);
        let rec = reader.get_record(0).unwrap();
        assert_eq!(rec.data.len(), 10_000);
        assert_eq!(rec.data[0], 0xAB);
    }
}
