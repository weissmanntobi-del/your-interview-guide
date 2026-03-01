//! # Challenge 01: Append-Only Storage (AppendVec)
//!
//! ## Problem
//! Implement an append-only storage vector that serializes account data into a
//! contiguous byte buffer. Accounts are written sequentially and can be read back
//! by offset. Once written, data is never modified in place.
//!
//! ## Why This Matters
//! AppendVec is the core on-disk storage primitive in Solana's AccountsDB. Every
//! account stored on a validator lives inside an AppendVec. The append-only design
//! enables lock-free reads, simple crash recovery, and straightforward memory-mapped
//! I/O. Understanding this structure is essential for anyone working on Solana
//! validator internals or building similar high-throughput storage systems.
//!
//! ## Requirements
//! - `AppendVec::new(capacity)` creates a buffer with the given byte capacity.
//! - `append(&mut self, account) -> Option<usize>` serializes the account into the
//!   internal buffer and returns the byte offset, or `None` if there is not enough
//!   remaining capacity.
//! - `get_account(&self, offset) -> Option<StoredAccountMeta>` deserializes the
//!   account stored at the given byte offset.
//! - `accounts_count(&self) -> usize` returns the number of stored accounts.
//! - `bytes_used(&self) -> usize` returns total bytes consumed so far.
//! - `remaining_bytes(&self) -> usize` returns capacity minus bytes used.
//! - `iter()` returns an iterator over all stored accounts in append order.
//!
//! ## Internal Binary Format (per account)
//! ```text
//! [data_len: u64 (8 bytes)]
//! [lamports: u64 (8 bytes)]
//! [owner:    32 bytes]
//! [pubkey:   32 bytes]
//! [data:     data_len bytes]
//! ```
//! The fixed header is therefore 80 bytes, followed by variable-length account data.
//!
//! ## Constraints
//! - No `unsafe` code; use standard byte manipulation.
//! - The buffer is a `Vec<u8>` pre-allocated to `capacity`.
//! - Do NOT allow partial writes: if an account does not fit, return `None` and
//!   leave the buffer unchanged.
//!
//! ## Hints
//! - Use `u64::to_le_bytes()` / `u64::from_le_bytes()` for integer serialization.
//! - The total size of one entry is `80 + data.len()`.
//! - For the iterator, track a cursor offset and keep reading until you reach
//!   `bytes_used`.

/// Fixed-size header: 8 (data_len) + 8 (lamports) + 32 (owner) + 32 (pubkey).
const HEADER_SIZE: usize = 80;

/// Metadata for a single stored account, returned when reading back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredAccountMeta {
    pub offset: usize,
    pub lamports: u64,
    pub owner: [u8; 32],
    pub pubkey: [u8; 32],
    pub data: Vec<u8>,
}

/// An account to be appended (input type).
#[derive(Debug, Clone)]
pub struct AccountToStore {
    pub lamports: u64,
    pub owner: [u8; 32],
    pub pubkey: [u8; 32],
    pub data: Vec<u8>,
}

/// Append-only byte-level storage for accounts.
pub struct AppendVec {
    data: Vec<u8>,
    capacity: usize,
    len: usize,
    count: usize,
}

impl AppendVec {
    /// Create a new AppendVec with the given byte capacity.
    pub fn new(capacity: usize) -> Self {
        todo!("Allocate a Vec<u8> of the given capacity and initialize bookkeeping fields")
    }

    /// Serialize and append an account. Returns the byte offset on success.
    pub fn append(&mut self, account: &AccountToStore) -> Option<usize> {
        todo!(
            "Calculate entry size (HEADER_SIZE + data.len()). \
             If it exceeds remaining_bytes(), return None. \
             Otherwise write data_len, lamports, owner, pubkey, data \
             in little-endian format and return the starting offset."
        )
    }

    /// Read back an account stored at the given byte offset.
    pub fn get_account(&self, offset: usize) -> Option<StoredAccountMeta> {
        todo!(
            "Read the header at `offset`, extract data_len, lamports, owner, pubkey, \
             then read `data_len` bytes of account data. Return None if offset is out of range."
        )
    }

    /// Number of accounts stored.
    pub fn accounts_count(&self) -> usize {
        todo!("Return the count of appended accounts")
    }

    /// Total bytes written so far.
    pub fn bytes_used(&self) -> usize {
        todo!("Return the current write cursor position")
    }

    /// Remaining writable bytes.
    pub fn remaining_bytes(&self) -> usize {
        todo!("Return capacity minus bytes used")
    }

    /// Iterator over all stored accounts in append order.
    pub fn iter(&self) -> AppendVecIter<'_> {
        todo!("Return an iterator that walks the buffer from offset 0 to bytes_used")
    }
}

/// Iterator over accounts stored in an AppendVec.
pub struct AppendVecIter<'a> {
    vec: &'a AppendVec,
    offset: usize,
}

impl<'a> Iterator for AppendVecIter<'a> {
    type Item = StoredAccountMeta;

    fn next(&mut self) -> Option<Self::Item> {
        todo!(
            "If offset >= vec.bytes_used(), return None. \
             Otherwise read the account at current offset, advance offset by entry size, \
             and return the account."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_account(pubkey_byte: u8, data_size: usize) -> AccountToStore {
        AccountToStore {
            lamports: 1_000_000,
            owner: [pubkey_byte.wrapping_add(1); 32],
            pubkey: [pubkey_byte; 32],
            data: vec![0xAB; data_size],
        }
    }

    #[test]
    fn test_append_and_retrieve_single() {
        let mut av = AppendVec::new(1024);
        let acct = sample_account(1, 64);
        let offset = av.append(&acct).expect("should fit");
        assert_eq!(offset, 0);
        let stored = av.get_account(offset).expect("should exist");
        assert_eq!(stored.lamports, 1_000_000);
        assert_eq!(stored.pubkey, [1u8; 32]);
        assert_eq!(stored.data.len(), 64);
    }

    #[test]
    fn test_append_multiple_accounts() {
        let mut av = AppendVec::new(4096);
        let offsets: Vec<usize> = (0..5)
            .map(|i| av.append(&sample_account(i, 32)).unwrap())
            .collect();
        assert_eq!(av.accounts_count(), 5);
        for (i, &off) in offsets.iter().enumerate() {
            let stored = av.get_account(off).unwrap();
            assert_eq!(stored.pubkey, [i as u8; 32]);
        }
    }

    #[test]
    fn test_capacity_exhaustion() {
        // Only enough room for one account with 16 bytes of data: HEADER_SIZE + 16 = 96
        let mut av = AppendVec::new(96);
        assert!(av.append(&sample_account(1, 16)).is_some());
        assert!(av.append(&sample_account(2, 16)).is_none());
        assert_eq!(av.accounts_count(), 1);
    }

    #[test]
    fn test_bytes_used_and_remaining() {
        let mut av = AppendVec::new(1024);
        assert_eq!(av.bytes_used(), 0);
        assert_eq!(av.remaining_bytes(), 1024);
        av.append(&sample_account(1, 20)).unwrap();
        assert_eq!(av.bytes_used(), HEADER_SIZE + 20);
        assert_eq!(av.remaining_bytes(), 1024 - HEADER_SIZE - 20);
    }

    #[test]
    fn test_iterator_returns_all_accounts() {
        let mut av = AppendVec::new(4096);
        for i in 0..4 {
            av.append(&sample_account(i, 10)).unwrap();
        }
        let collected: Vec<StoredAccountMeta> = av.iter().collect();
        assert_eq!(collected.len(), 4);
        for (i, acct) in collected.iter().enumerate() {
            assert_eq!(acct.pubkey, [i as u8; 32]);
        }
    }

    #[test]
    fn test_get_account_invalid_offset() {
        let av = AppendVec::new(1024);
        assert!(av.get_account(0).is_none());
        assert!(av.get_account(9999).is_none());
    }

    #[test]
    fn test_zero_length_data() {
        let mut av = AppendVec::new(512);
        let acct = sample_account(7, 0);
        let offset = av.append(&acct).unwrap();
        let stored = av.get_account(offset).unwrap();
        assert_eq!(stored.data.len(), 0);
        assert_eq!(stored.lamports, 1_000_000);
    }

    #[test]
    fn test_offsets_are_sequential() {
        let mut av = AppendVec::new(4096);
        let off1 = av.append(&sample_account(1, 10)).unwrap();
        let off2 = av.append(&sample_account(2, 20)).unwrap();
        assert_eq!(off1, 0);
        assert_eq!(off2, HEADER_SIZE + 10);
    }
}
