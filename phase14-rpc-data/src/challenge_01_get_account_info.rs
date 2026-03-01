/// # Challenge 01: getAccountInfo RPC Implementation
///
/// Implements the core getAccountInfo RPC method used to fetch on-chain account
/// data. This is the most fundamental RPC call in Solana — every wallet, explorer,
/// and dApp uses it to read state. The challenge covers pubkey encoding (hex),
/// account data encoding (Base58, Base64, JsonParsed), and response formatting.
///
/// Key concepts:
/// - Account model: pubkey, lamports, owner, data, executable, rent_epoch
/// - Multiple encoding formats for binary data transport over JSON-RPC
/// - Base64 encoding implemented from scratch (no external crate)

use std::collections::HashMap;

/// On-chain account data stored in the validator's account database.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountData {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub owner: [u8; 32],
    pub data: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: u64,
}

/// Encoding format for account data in the RPC response.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Encoding {
    Base58,
    Base64,
    JsonParsed,
}

/// JSON-serializable account response returned by the RPC endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct AccountResponse {
    pub pubkey: String,
    pub lamports: u64,
    pub owner: String,
    pub data: Vec<String>,
    pub executable: bool,
    pub rent_epoch: u64,
}

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode a 32-byte pubkey as a hex string.
pub fn encode_pubkey(pubkey: &[u8; 32]) -> String {
    todo!("Convert each byte of pubkey to two hex characters and return the concatenated string")
}

/// Encode raw bytes using Base64 from scratch (no external crate).
///
/// Process input in 3-byte groups, producing 4 Base64 characters each.
/// Pad with '=' as needed for inputs whose length is not a multiple of 3.
pub fn base64_encode(input: &[u8]) -> String {
    todo!("Implement Base64 encoding: split into 3-byte chunks, map to 4 sextets, index into BASE64_ALPHABET, pad with '='")
}

/// Encode account data bytes according to the requested encoding format.
///
/// Returns a Vec<String> where:
/// - Base58: [base58_string, "base58"]
/// - Base64: [base64_string, "base64"]
/// - JsonParsed: [hex_string, "jsonParsed"]
pub fn encode_data(data: &[u8], encoding: Encoding) -> Vec<String> {
    todo!("Match on encoding variant. For Base58 use simplified hex fallback, for Base64 use base64_encode, for JsonParsed use hex encoding. Return [encoded_data, encoding_label]")
}

/// Simple Base58 encoding (simplified: uses hex as a stand-in for the real
/// Base58 alphabet since full Base58 requires bignum arithmetic).
pub fn base58_encode_simple(input: &[u8]) -> String {
    todo!("Encode bytes as hex string prefixed with 'bs58:' to distinguish from raw hex")
}

/// Fetch an account from the in-memory store and format the response.
///
/// Looks up the account by its 32-byte pubkey in the provided HashMap,
/// then encodes all fields into the JSON-friendly AccountResponse.
/// Returns None if the pubkey is not found.
pub fn get_account_info(
    accounts: &HashMap<[u8; 32], AccountData>,
    pubkey: &[u8; 32],
    encoding: Encoding,
) -> Option<AccountResponse> {
    todo!("Look up pubkey in accounts map. If found, encode pubkey and owner with encode_pubkey, encode data with encode_data, and build AccountResponse. Return None if not found.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_account() -> AccountData {
        AccountData {
            pubkey: [1u8; 32],
            lamports: 1_000_000,
            owner: [2u8; 32],
            data: vec![0xDE, 0xAD, 0xBE, 0xEF],
            executable: false,
            rent_epoch: 42,
        }
    }

    fn make_store() -> HashMap<[u8; 32], AccountData> {
        let acct = sample_account();
        let mut map = HashMap::new();
        map.insert(acct.pubkey, acct);
        map
    }

    #[test]
    fn test_encode_pubkey_hex() {
        let pubkey = [0xABu8; 32];
        let encoded = encode_pubkey(&pubkey);
        assert_eq!(encoded.len(), 64);
        assert!(encoded.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_base64_encode_empty() {
        assert_eq!(base64_encode(&[]), "");
    }

    #[test]
    fn test_base64_encode_padding() {
        // "M" -> "TQ=="  (1 byte -> 2 chars + 2 padding)
        assert_eq!(base64_encode(b"M"), "TQ==");
        // "Ma" -> "TWE=" (2 bytes -> 3 chars + 1 padding)
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        // "Man" -> "TWFu" (3 bytes -> 4 chars, no padding)
        assert_eq!(base64_encode(b"Man"), "TWFu");
    }

    #[test]
    fn test_encode_data_base64() {
        let data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let encoded = encode_data(&data, Encoding::Base64);
        assert_eq!(encoded.len(), 2);
        assert_eq!(encoded[1], "base64");
        assert!(!encoded[0].is_empty());
    }

    #[test]
    fn test_encode_data_json_parsed() {
        let data = vec![0xFF, 0x00];
        let encoded = encode_data(&data, Encoding::JsonParsed);
        assert_eq!(encoded[1], "jsonParsed");
    }

    #[test]
    fn test_get_account_info_found() {
        let store = make_store();
        let resp = get_account_info(&store, &[1u8; 32], Encoding::Base64);
        assert!(resp.is_some());
        let resp = resp.unwrap();
        assert_eq!(resp.lamports, 1_000_000);
        assert!(!resp.executable);
        assert_eq!(resp.rent_epoch, 42);
    }

    #[test]
    fn test_get_account_info_not_found() {
        let store = make_store();
        let resp = get_account_info(&store, &[99u8; 32], Encoding::Base64);
        assert!(resp.is_none());
    }

    #[test]
    fn test_get_account_info_base58_encoding() {
        let store = make_store();
        let resp = get_account_info(&store, &[1u8; 32], Encoding::Base58).unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[1], "base58");
    }

    #[test]
    fn test_pubkey_encoding_deterministic() {
        let pk = [0x42u8; 32];
        assert_eq!(encode_pubkey(&pk), encode_pubkey(&pk));
    }

    #[test]
    fn test_base64_known_vector() {
        // "Hello" -> "SGVsbG8="
        assert_eq!(base64_encode(b"Hello"), "SGVsbG8=");
    }
}
