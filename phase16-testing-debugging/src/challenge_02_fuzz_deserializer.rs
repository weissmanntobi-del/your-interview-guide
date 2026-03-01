/// Challenge 02 - Fuzz Deserializer: Find Bugs in a Buggy Parser
///
/// This challenge provides two deserializers:
/// - BuggyDeserializer: has intentional bugs (no bounds checks, integer overflow)
/// - SafeDeserializer: correctly handles all edge cases
///
/// Students write fuzz tests (using proptest) to find inputs that crash the buggy
/// version, and verify the safe version handles them gracefully.

use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub owner: [u8; 32],
    pub lamports: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeserializeError {
    BufferTooShort,
    InvalidLength,
    Overflow,
}

// ============================================================
// BUGGY DESERIALIZER (intentionally flawed - DO NOT FIX these)
// ============================================================

pub struct BuggyDeserializer;

impl BuggyDeserializer {
    /// BUG: Does not check if `len` exceeds remaining buffer.
    /// Will panic on out-of-bounds slice access.
    pub fn deserialize_string(data: &[u8]) -> String {
        if data.len() < 4 {
            return String::new();
        }
        let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        // BUG: No bounds check on len! If len > data.len() - 4, this panics.
        let bytes = &data[4..4 + len];
        String::from_utf8_lossy(bytes).to_string()
    }

    /// BUG: Integer overflow when computing len * 8.
    /// If len is large enough, len * 8 wraps around in usize.
    pub fn deserialize_vec_u64(data: &[u8]) -> Vec<u64> {
        if data.len() < 4 {
            return vec![];
        }
        let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        // BUG: len * 8 can overflow on 32-bit or with huge len values
        let byte_len = len * 8;
        if data.len() < 4 + byte_len {
            return vec![];
        }
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            let offset = 4 + i * 8;
            let val = u64::from_le_bytes(
                data[offset..offset + 8].try_into().unwrap(),
            );
            result.push(val);
        }
        result
    }

    /// BUG: Reads data length from buffer but doesn't validate against remaining bytes.
    pub fn deserialize_account(data: &[u8]) -> Account {
        let mut pos = 0;
        let mut owner = [0u8; 32];
        // BUG: No check that data has 32 bytes
        owner.copy_from_slice(&data[pos..pos + 32]);
        pos += 32;

        let lamports = u64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;

        let data_len = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;

        // BUG: data_len could exceed remaining buffer
        let account_data = data[pos..pos + data_len].to_vec();

        Account {
            owner,
            lamports,
            data: account_data,
        }
    }
}

// ============================================================
// SAFE DESERIALIZER (student implements)
// ============================================================

pub struct SafeDeserializer;

impl SafeDeserializer {
    /// Safely deserialize a length-prefixed string.
    pub fn deserialize_string(data: &[u8]) -> Result<String, DeserializeError> {
        // TODO: Implement safe version
        // 1. Check data.len() >= 4
        // 2. Read len from first 4 bytes
        // 3. Check 4 + len <= data.len()
        // 4. Return the string
        todo!("Implement safe deserialize_string")
    }

    /// Safely deserialize a length-prefixed vector of u64.
    pub fn deserialize_vec_u64(data: &[u8]) -> Result<Vec<u64>, DeserializeError> {
        // TODO: Implement safe version
        // 1. Check data.len() >= 4
        // 2. Read len from first 4 bytes
        // 3. Use checked_mul for len * 8 to avoid overflow
        // 4. Validate total size fits in buffer
        // 5. Parse each u64
        todo!("Implement safe deserialize_vec_u64")
    }

    /// Safely deserialize an Account.
    pub fn deserialize_account(data: &[u8]) -> Result<Account, DeserializeError> {
        // TODO: Implement safe version
        // 1. Check minimum size (32 + 8 + 4 = 44)
        // 2. Read owner, lamports, data_len with bounds checks
        // 3. Validate data_len fits in remaining buffer
        // 4. Return Account
        todo!("Implement safe deserialize_account")
    }
}

/// Fuzz function: feed random bytes to both deserializers, assert safe never panics.
pub fn fuzz_deserialize(data: &[u8]) {
    // Safe version should never panic, only return errors
    let _ = SafeDeserializer::deserialize_string(data);
    let _ = SafeDeserializer::deserialize_vec_u64(data);
    let _ = SafeDeserializer::deserialize_account(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buggy_string_panics_on_crafted_input() {
        // Craft input where length field says 1000 but only 4 bytes of data
        let mut data = vec![0u8; 4];
        data[0] = 0xE8; // 1000 in little-endian
        data[1] = 0x03;
        let result = std::panic::catch_unwind(|| {
            BuggyDeserializer::deserialize_string(&data);
        });
        assert!(result.is_err(), "Buggy deserializer should panic");
    }

    #[test]
    fn test_safe_string_handles_gracefully() {
        let mut data = vec![0u8; 4];
        data[0] = 0xE8;
        data[1] = 0x03;
        let result = SafeDeserializer::deserialize_string(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_buggy_account_panics_on_short_input() {
        let data = vec![0u8; 10]; // Way too short for an Account
        let result = std::panic::catch_unwind(|| {
            BuggyDeserializer::deserialize_account(&data);
        });
        assert!(result.is_err(), "Buggy deserializer should panic on short input");
    }

    #[test]
    fn test_safe_account_handles_short_input() {
        let data = vec![0u8; 10];
        let result = SafeDeserializer::deserialize_account(&data);
        assert_eq!(result, Err(DeserializeError::BufferTooShort));
    }

    #[test]
    fn test_safe_deserializers_valid_data() {
        // Valid string
        let mut string_data = vec![];
        string_data.extend_from_slice(&5u32.to_le_bytes());
        string_data.extend_from_slice(b"hello");
        assert_eq!(SafeDeserializer::deserialize_string(&string_data).unwrap(), "hello");

        // Valid vec_u64
        let mut vec_data = vec![];
        vec_data.extend_from_slice(&2u32.to_le_bytes());
        vec_data.extend_from_slice(&42u64.to_le_bytes());
        vec_data.extend_from_slice(&99u64.to_le_bytes());
        let v = SafeDeserializer::deserialize_vec_u64(&vec_data).unwrap();
        assert_eq!(v, vec![42, 99]);
    }

    proptest! {
        /// Fuzz: safe deserializer never panics on any input.
        #[test]
        fn prop_fuzz_safe_never_panics(data in prop::collection::vec(any::<u8>(), 0..512)) {
            // This should never panic
            fuzz_deserialize(&data);
        }
    }
}
