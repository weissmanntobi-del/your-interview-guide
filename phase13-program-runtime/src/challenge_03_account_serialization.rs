/// # Challenge 03: BPF Account Input Serialization
///
/// When the Solana runtime invokes a BPF program, it serializes all input accounts
/// into a contiguous byte buffer that the program reads from linear memory. This
/// challenge implements that serialization format.
///
/// ## Wire Format
///
/// The buffer begins with a little-endian `u64` count of accounts, followed by each
/// account serialized as:
///
/// | Field        | Size (bytes) | Encoding        |
/// |--------------|-------------|-----------------|
/// | is_signer    | 1           | 0x00 or 0x01    |
/// | is_writable  | 1           | 0x00 or 0x01    |
/// | key          | 32          | raw bytes       |
/// | lamports     | 8           | little-endian   |
/// | data_len     | 8           | little-endian   |
/// | data         | data_len    | raw bytes       |
/// | owner        | 32          | raw bytes       |
/// | executable   | 1           | 0x00 or 0x01    |
/// | rent_epoch   | 8           | little-endian   |
///
/// Note: `data` is variable-length, determined by `data_len`.

/// Errors that can occur during deserialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// The buffer ended unexpectedly while reading a field.
    UnexpectedEof { field: String, expected: usize, available: usize },
    /// The account count or data length is unreasonably large.
    InvalidLength(u64),
    /// A boolean field contained a value other than 0 or 1.
    InvalidBool(u8),
}

/// Represents a single account as passed to a BPF program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputAccount {
    pub is_signer: bool,
    pub is_writable: bool,
    pub key: [u8; 32],
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: [u8; 32],
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Serialize a list of accounts into the BPF input format.
///
/// Layout: `account_count (u64 LE)` followed by each account in wire format.
pub fn serialize_accounts(accounts: &[InputAccount]) -> Vec<u8> {
    todo!()
}

/// Deserialize a byte buffer back into a list of `InputAccount`s.
///
/// Returns `SerializationError` if the buffer is malformed or truncated.
pub fn deserialize_accounts(data: &[u8]) -> Result<Vec<InputAccount>, SerializationError> {
    todo!()
}

/// Compute the byte offset within `serialized` where the `data` field of the
/// account at position `index` begins. Returns `None` if the index is out of range
/// or the buffer is too short to reach that account.
///
/// This is useful for programs that want to do zero-copy access to account data.
pub fn account_data_offset(serialized: &[u8], index: usize) -> Option<usize> {
    todo!()
}

/// Helper: compute the serialized size of a single account.
fn account_wire_size(account: &InputAccount) -> usize {
    // is_signer(1) + is_writable(1) + key(32) + lamports(8)
    // + data_len(8) + data(N) + owner(32) + executable(1) + rent_epoch(8)
    1 + 1 + 32 + 8 + 8 + account.data.len() + 32 + 1 + 8
}

/// Helper: read a little-endian u64 from a slice at the given offset.
fn read_u64_le(data: &[u8], offset: usize) -> Result<u64, SerializationError> {
    todo!()
}

/// Helper: create a test account with predictable values.
#[cfg(test)]
fn make_test_account(id: u8, data_len: usize) -> InputAccount {
    InputAccount {
        is_signer: id % 2 == 0,
        is_writable: id % 3 == 0,
        key: [id; 32],
        lamports: id as u64 * 1_000_000,
        data: vec![id; data_len],
        owner: [id.wrapping_add(1); 32],
        executable: false,
        rent_epoch: 42,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_single_account() {
        let account = make_test_account(1, 64);
        let serialized = serialize_accounts(&[account.clone()]);
        let deserialized = deserialize_accounts(&serialized).unwrap();
        assert_eq!(deserialized.len(), 1);
        assert_eq!(deserialized[0], account);
    }

    #[test]
    fn test_round_trip_multiple_accounts() {
        let accounts: Vec<_> = (0..5).map(|i| make_test_account(i, 32 * i as usize)).collect();
        let serialized = serialize_accounts(&accounts);
        let deserialized = deserialize_accounts(&serialized).unwrap();
        assert_eq!(deserialized, accounts);
    }

    #[test]
    fn test_round_trip_empty() {
        let serialized = serialize_accounts(&[]);
        let deserialized = deserialize_accounts(&serialized).unwrap();
        assert!(deserialized.is_empty());
        // Should just be a u64 zero
        assert_eq!(serialized.len(), 8);
        assert_eq!(u64::from_le_bytes(serialized[..8].try_into().unwrap()), 0);
    }

    #[test]
    fn test_account_data_offset_calculation() {
        let accts = vec![
            make_test_account(1, 10),
            make_test_account(2, 20),
            make_test_account(3, 30),
        ];
        let serialized = serialize_accounts(&accts);

        // First account data starts after: 8 (count) + 1+1+32+8+8 = 58 bytes
        let offset_0 = account_data_offset(&serialized, 0).unwrap();
        assert_eq!(offset_0, 8 + 1 + 1 + 32 + 8 + 8);

        // Verify we can read the data at that offset
        let data_slice = &serialized[offset_0..offset_0 + 10];
        assert_eq!(data_slice, &[1u8; 10]);

        // Second account offset
        let offset_1 = account_data_offset(&serialized, 1).unwrap();
        let data_slice = &serialized[offset_1..offset_1 + 20];
        assert_eq!(data_slice, &[2u8; 20]);
    }

    #[test]
    fn test_truncated_buffer() {
        let account = make_test_account(1, 64);
        let serialized = serialize_accounts(&[account]);
        // Truncate in the middle of the data field
        let truncated = &serialized[..40];
        let result = deserialize_accounts(truncated);
        assert!(matches!(result, Err(SerializationError::UnexpectedEof { .. })));
    }

    #[test]
    fn test_no_accounts_just_count() {
        // Buffer with count = 0
        let data = 0u64.to_le_bytes().to_vec();
        let result = deserialize_accounts(&data).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_offset_out_of_range() {
        let accts = vec![make_test_account(1, 10)];
        let serialized = serialize_accounts(&accts);
        // Index 1 does not exist
        assert_eq!(account_data_offset(&serialized, 1), None);
    }

    #[test]
    fn test_large_data_field() {
        let account = make_test_account(7, 1024);
        let serialized = serialize_accounts(&[account.clone()]);
        let deserialized = deserialize_accounts(&serialized).unwrap();
        assert_eq!(deserialized[0].data.len(), 1024);
        assert_eq!(deserialized[0], account);
    }

    #[test]
    fn test_buffer_too_short_for_count() {
        let data = vec![0u8; 4]; // need 8 bytes for count
        let result = deserialize_accounts(&data);
        assert!(matches!(result, Err(SerializationError::UnexpectedEof { .. })));
    }
}
