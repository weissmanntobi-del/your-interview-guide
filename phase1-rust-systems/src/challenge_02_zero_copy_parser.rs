//! # Challenge 1.2: Zero-Copy Parser
//!
//! ## Problem
//! Implement a zero-copy byte parser that reads structured data from a &[u8]
//! without allocating memory. All parsed references point into the original buffer.
//!
//! ## Why This Matters
//! Blockchain clients parse millions of transactions, blocks, and account data.
//! Allocating new Strings and Vecs for each parse is a performance killer.
//! Zero-copy parsing reads data directly from the input buffer using references
//! and byte slices, which is how reth and Agave handle high-throughput data.
//!
//! ## Requirements
//! Implement a `Parser<'a>` that borrows a `&'a [u8]` and provides:
//! - `read_u8()`, `read_u16_le()`, `read_u32_le()`, `read_u64_le()` — read fixed-size integers
//! - `read_bytes(n)` — return &'a [u8] slice of n bytes (zero-copy!)
//! - `read_bool()` — read a single byte as boolean
//! - `remaining()` — bytes left to parse
//! - `position()` — current read position
//!
//! All methods return `Result<T, ParseError>` and advance the internal cursor.
//!
//! ## Constraints
//! - NO heap allocation allowed (no Vec, String, Box)
//! - Returned byte slices must borrow from the original input
//! - Must handle insufficient data gracefully (return Err, not panic)

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Not enough bytes remaining to fulfill the read
    InsufficientData { needed: usize, available: usize },
    /// Invalid data encountered
    InvalidData(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InsufficientData { needed, available } => {
                write!(f, "need {} bytes but only {} available", needed, available)
            }
            ParseError::InvalidData(msg) => write!(f, "invalid data: {}", msg),
        }
    }
}

pub struct Parser<'a> {
    // TODO: implement fields
    // You'll need:
    // - The input buffer reference: &'a [u8]
    // - A position tracker: usize
    _placeholder: std::marker::PhantomData<&'a ()>,
}

impl<'a> Parser<'a> {
    /// Create a new parser from a byte slice.
    pub fn new(_data: &'a [u8]) -> Self {
        todo!("Store reference and initialize position to 0")
    }

    /// Read a single u8.
    pub fn read_u8(&mut self) -> Result<u8, ParseError> {
        todo!("Read 1 byte, advance cursor")
    }

    /// Read a u16 in little-endian byte order.
    pub fn read_u16_le(&mut self) -> Result<u16, ParseError> {
        todo!("Read 2 bytes as little-endian u16")
    }

    /// Read a u32 in little-endian byte order.
    pub fn read_u32_le(&mut self) -> Result<u32, ParseError> {
        todo!("Read 4 bytes as little-endian u32")
    }

    /// Read a u64 in little-endian byte order.
    pub fn read_u64_le(&mut self) -> Result<u64, ParseError> {
        todo!("Read 8 bytes as little-endian u64")
    }

    /// Read n bytes as a slice. Zero-copy — returns a reference into the original buffer.
    pub fn read_bytes(&mut self, _n: usize) -> Result<&'a [u8], ParseError> {
        todo!("Return &'a [u8] slice from original buffer, advance cursor by n")
    }

    /// Read a single byte as a boolean (0 = false, 1 = true, other = error).
    pub fn read_bool(&mut self) -> Result<bool, ParseError> {
        todo!("Read 1 byte, interpret as bool")
    }

    /// Return the number of bytes remaining.
    pub fn remaining(&self) -> usize {
        todo!("Return data.len() - position")
    }

    /// Return the current read position.
    pub fn position(&self) -> usize {
        todo!("Return current position")
    }
}

/// Example: A simplified transaction header that can be parsed zero-copy.
/// Implement the `parse` method using your Parser.
pub struct TxHeader<'a> {
    pub version: u8,
    pub nonce: u64,
    pub gas_limit: u64,
    pub to: &'a [u8; 20], // 20-byte address, borrowed from input
    pub value: u64,
    pub data: &'a [u8], // Variable-length calldata, borrowed from input
}

impl<'a> TxHeader<'a> {
    /// Parse a transaction header from raw bytes.
    /// Format: version(1) | nonce(8) | gas_limit(8) | to(20) | value(8) | data_len(4) | data(N)
    pub fn parse(_input: &'a [u8]) -> Result<Self, ParseError> {
        todo!("Use Parser to read each field sequentially")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let data = [0x42];
        let mut p = Parser::new(&data);
        assert_eq!(p.read_u8().unwrap(), 0x42);
        assert_eq!(p.remaining(), 0);
    }

    #[test]
    fn test_read_u16_le() {
        let data = [0x34, 0x12]; // 0x1234 in little-endian
        let mut p = Parser::new(&data);
        assert_eq!(p.read_u16_le().unwrap(), 0x1234);
    }

    #[test]
    fn test_read_u32_le() {
        let data = [0x78, 0x56, 0x34, 0x12];
        let mut p = Parser::new(&data);
        assert_eq!(p.read_u32_le().unwrap(), 0x12345678);
    }

    #[test]
    fn test_read_u64_le() {
        let data = 1000u64.to_le_bytes();
        let mut p = Parser::new(&data);
        assert_eq!(p.read_u64_le().unwrap(), 1000);
    }

    #[test]
    fn test_read_bytes_zero_copy() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut p = Parser::new(&data);
        let slice = p.read_bytes(4).unwrap();
        assert_eq!(slice, &[1, 2, 3, 4]);
        // Verify it's actually zero-copy: the pointer should be into the original data
        let original_ptr = data.as_ptr();
        let slice_ptr = slice.as_ptr();
        assert_eq!(original_ptr, slice_ptr, "must be zero-copy");
    }

    #[test]
    fn test_read_bool() {
        let data = [0x00, 0x01, 0x02];
        let mut p = Parser::new(&data);
        assert_eq!(p.read_bool().unwrap(), false);
        assert_eq!(p.read_bool().unwrap(), true);
        assert!(p.read_bool().is_err()); // 0x02 is invalid for bool
    }

    #[test]
    fn test_insufficient_data() {
        let data = [0x42];
        let mut p = Parser::new(&data);
        let err = p.read_u32_le().unwrap_err();
        assert_eq!(
            err,
            ParseError::InsufficientData {
                needed: 4,
                available: 1
            }
        );
    }

    #[test]
    fn test_position_tracking() {
        let data = [0; 16];
        let mut p = Parser::new(&data);
        assert_eq!(p.position(), 0);
        p.read_u32_le().unwrap();
        assert_eq!(p.position(), 4);
        p.read_u64_le().unwrap();
        assert_eq!(p.position(), 12);
        assert_eq!(p.remaining(), 4);
    }

    #[test]
    fn test_sequential_reads() {
        let mut data = Vec::new();
        data.push(0x01u8); // version
        data.extend_from_slice(&42u64.to_le_bytes()); // nonce
        data.extend_from_slice(&[0xAA; 4]); // some bytes

        let mut p = Parser::new(&data);
        assert_eq!(p.read_u8().unwrap(), 1);
        assert_eq!(p.read_u64_le().unwrap(), 42);
        assert_eq!(p.read_bytes(4).unwrap(), &[0xAA; 4]);
        assert_eq!(p.remaining(), 0);
    }

    #[test]
    fn test_empty_input() {
        let data: [u8; 0] = [];
        let mut p = Parser::new(&data);
        assert_eq!(p.remaining(), 0);
        assert!(p.read_u8().is_err());
    }

    #[test]
    fn test_tx_header_parse() {
        let mut data = Vec::new();
        data.push(0x02); // version
        data.extend_from_slice(&10u64.to_le_bytes()); // nonce
        data.extend_from_slice(&21000u64.to_le_bytes()); // gas_limit
        data.extend_from_slice(&[0xAB; 20]); // to address
        data.extend_from_slice(&1000u64.to_le_bytes()); // value
        data.extend_from_slice(&3u32.to_le_bytes()); // data_len
        data.extend_from_slice(&[0xDE, 0xAD, 0xFF]); // data

        let tx = TxHeader::parse(&data).unwrap();
        assert_eq!(tx.version, 2);
        assert_eq!(tx.nonce, 10);
        assert_eq!(tx.gas_limit, 21000);
        assert_eq!(tx.to, &[0xAB; 20]);
        assert_eq!(tx.value, 1000);
        assert_eq!(tx.data, &[0xDE, 0xAD, 0xFF]);
    }
}
