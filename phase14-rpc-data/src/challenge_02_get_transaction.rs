/// # Challenge 02: getTransaction — Parse Compiled Transaction
///
/// Solana transactions use a compact binary format with "compiled instructions"
/// where account addresses are referenced by index into a shared account_keys
/// table. This challenge implements parsing that format: resolving account indices
/// back to full pubkeys, formatting signatures and blockhashes, and handling
/// multi-instruction transactions.
///
/// Key concepts:
/// - Compiled instruction format with program_id_index and account index vectors
/// - Index-based account key resolution
/// - Transaction metadata: signature, slot, block_time

use std::collections::HashMap;

/// A raw on-chain transaction as stored in a block.
#[derive(Debug, Clone, PartialEq)]
pub struct RawTransaction {
    pub signature: [u8; 64],
    pub message: TransactionMessage,
    pub slot: u64,
    pub block_time: Option<i64>,
}

/// The message portion of a transaction containing account keys and instructions.
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionMessage {
    pub account_keys: Vec<[u8; 32]>,
    pub instructions: Vec<CompiledInstruction>,
    pub recent_blockhash: [u8; 32],
}

/// A compiled instruction referencing accounts by index into account_keys.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

/// A human-readable instruction with resolved pubkeys as hex strings.
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedInstruction {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub data: String,
}

/// A fully parsed and formatted transaction ready for JSON serialization.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedTransaction {
    pub signature: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub recent_blockhash: String,
    pub instructions: Vec<FormattedInstruction>,
    pub account_keys: Vec<String>,
}

/// Encode a byte slice as a hex string.
fn hex_encode(bytes: &[u8]) -> String {
    todo!("Map each byte to two lowercase hex characters and collect into a String")
}

/// Resolve a compiled instruction's account indices to hex-encoded pubkeys.
///
/// The program_id_index selects the program account from account_keys.
/// Each entry in the accounts vector is an index into account_keys.
/// Returns a FormattedInstruction with all keys hex-encoded and data hex-encoded.
pub fn resolve_instruction(
    instruction: &CompiledInstruction,
    account_keys: &[[u8; 32]],
) -> FormattedInstruction {
    todo!("Use program_id_index to look up the program key, map each account index to its key via account_keys, hex-encode data bytes. Return FormattedInstruction.")
}

/// Parse a raw transaction into a fully formatted ParsedTransaction.
///
/// Steps:
/// 1. Hex-encode the 64-byte signature
/// 2. Hex-encode the recent_blockhash
/// 3. Hex-encode each account key
/// 4. Resolve each compiled instruction using resolve_instruction
pub fn parse_transaction(raw: &RawTransaction) -> ParsedTransaction {
    todo!("Hex-encode signature, blockhash, and account_keys. Resolve each compiled instruction. Assemble and return ParsedTransaction.")
}

/// Extract the list of program IDs invoked by a transaction (deduplicated).
pub fn extract_program_ids(raw: &RawTransaction) -> Vec<String> {
    todo!("For each instruction, resolve its program_id_index to a hex-encoded key. Deduplicate and return.")
}

/// Count how many times each account is referenced across all instructions.
pub fn account_reference_counts(raw: &RawTransaction) -> HashMap<String, usize> {
    todo!("Iterate all instructions, resolve each account index to hex string, count occurrences in a HashMap")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_raw_tx(num_instructions: usize) -> RawTransaction {
        let key_a = [0xAAu8; 32]; // program
        let key_b = [0xBBu8; 32]; // account 1
        let key_c = [0xCCu8; 32]; // account 2

        let mut instructions = Vec::new();
        for i in 0..num_instructions {
            instructions.push(CompiledInstruction {
                program_id_index: 0,
                accounts: vec![1, 2],
                data: vec![i as u8, 0xFF],
            });
        }

        RawTransaction {
            signature: [0x11u8; 64],
            message: TransactionMessage {
                account_keys: vec![key_a, key_b, key_c],
                instructions,
                recent_blockhash: [0xFFu8; 32],
            },
            slot: 100,
            block_time: Some(1_700_000_000),
        }
    }

    #[test]
    fn test_parse_single_instruction() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        assert_eq!(parsed.instructions.len(), 1);
        assert_eq!(parsed.slot, 100);
        assert_eq!(parsed.block_time, Some(1_700_000_000));
    }

    #[test]
    fn test_parse_multi_instruction() {
        let raw = make_raw_tx(3);
        let parsed = parse_transaction(&raw);
        assert_eq!(parsed.instructions.len(), 3);
    }

    #[test]
    fn test_resolve_program_id() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        let expected_program = hex_encode(&[0xAAu8; 32]);
        assert_eq!(parsed.instructions[0].program_id, expected_program);
    }

    #[test]
    fn test_resolve_accounts() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        let ix = &parsed.instructions[0];
        assert_eq!(ix.accounts.len(), 2);
        assert_eq!(ix.accounts[0], hex_encode(&[0xBBu8; 32]));
        assert_eq!(ix.accounts[1], hex_encode(&[0xCCu8; 32]));
    }

    #[test]
    fn test_signature_encoding() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        assert_eq!(parsed.signature.len(), 128); // 64 bytes -> 128 hex chars
    }

    #[test]
    fn test_blockhash_encoding() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        assert_eq!(parsed.recent_blockhash.len(), 64); // 32 bytes -> 64 hex chars
    }

    #[test]
    fn test_account_keys_encoded() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        assert_eq!(parsed.account_keys.len(), 3);
        for key in &parsed.account_keys {
            assert_eq!(key.len(), 64);
        }
    }

    #[test]
    fn test_extract_program_ids_dedup() {
        let raw = make_raw_tx(3); // all use program_id_index 0
        let ids = extract_program_ids(&raw);
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn test_instruction_data_hex_encoded() {
        let raw = make_raw_tx(1);
        let parsed = parse_transaction(&raw);
        // data was [0, 0xFF] -> "00ff"
        assert_eq!(parsed.instructions[0].data, "00ff");
    }

    #[test]
    fn test_account_reference_counts() {
        let raw = make_raw_tx(2);
        let counts = account_reference_counts(&raw);
        let key_b_hex = hex_encode(&[0xBBu8; 32]);
        assert_eq!(*counts.get(&key_b_hex).unwrap(), 2);
    }
}
