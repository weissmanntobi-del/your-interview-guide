//! # Challenge 03: Snapshot Generator
//!
//! ## Problem
//! Implement a snapshot generator that captures the full account state at a given
//! slot boundary. The snapshot must be deterministic: the same set of accounts at
//! the same slot always produces the same hash. It must also support serialization,
//! deserialization, and integrity verification.
//!
//! ## Why This Matters
//! Solana validators periodically create snapshots so that new validators can
//! bootstrap without replaying the entire ledger from genesis. A snapshot packages
//! every live account along with a cryptographic hash for integrity. Snapshot
//! generation and verification are performance-critical paths that run during
//! epoch boundaries and during validator startup.
//!
//! ## Requirements
//! - `generate_snapshot(slot, accounts, created_at) -> Snapshot` sorts accounts
//!   by pubkey, computes a SHA-256 hash over the sorted entries, and returns a
//!   `Snapshot`.
//! - `serialize_snapshot(snapshot) -> Vec<u8>` converts the snapshot to bytes.
//! - `deserialize_snapshot(bytes) -> Option<Snapshot>` reconstructs it.
//! - `verify_snapshot(snapshot) -> bool` recomputes the hash and checks it matches.
//!
//! ## Constraints
//! - Hash computation: feed each sorted account's pubkey, lamports (LE bytes),
//!   owner, executable flag (1 byte: 0x01 or 0x00), and data into the SHA-256
//!   hasher in that order.
//! - Serialization format is up to you, but round-tripping must be lossless.
//! - Use the `sha2` crate (`use sha2::{Sha256, Digest};`).
//!
//! ## Hints
//! - For serialization, a simple length-prefixed binary format works well.
//! - `Sha256::new()` -> `hasher.update(bytes)` -> `hasher.finalize()`.
//! - Sorting accounts by pubkey before hashing ensures determinism regardless of
//!   the order accounts are provided.

use sha2::{Sha256, Digest};

/// A single account entry included in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountEntry {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub owner: [u8; 32],
    pub data: Vec<u8>,
    pub executable: bool,
}

/// A complete snapshot of all accounts at a specific slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub slot: u64,
    pub hash: [u8; 32],
    pub accounts: Vec<AccountEntry>,
    pub created_at: u64,
}

/// Generate a snapshot for the given slot.
///
/// Accounts are sorted by pubkey. The hash is computed by feeding each sorted
/// account's fields into SHA-256 in a deterministic order.
pub fn generate_snapshot(
    slot: u64,
    accounts: Vec<AccountEntry>,
    created_at: u64,
) -> Snapshot {
    todo!(
        "1. Sort accounts by pubkey. \
         2. Create a Sha256 hasher. \
         3. For each account, update the hasher with pubkey, lamports (LE), \
            owner, executable (0x01/0x00), data. \
         4. Finalize and store the hash in the Snapshot."
    )
}

/// Compute the expected hash for a snapshot (sorted accounts already inside).
fn compute_hash(accounts: &[AccountEntry]) -> [u8; 32] {
    todo!(
        "Hash all accounts in order using SHA-256. \
         Same logic as generate_snapshot's hash step."
    )
}

/// Serialize a snapshot to a byte vector.
///
/// Format (all integers little-endian):
/// [slot:8][created_at:8][hash:32][num_accounts:u64:8]
/// For each account:
///   [pubkey:32][lamports:8][owner:32][executable:1][data_len:8][data:N]
pub fn serialize_snapshot(snapshot: &Snapshot) -> Vec<u8> {
    todo!("Write the snapshot header and each account entry into a Vec<u8>")
}

/// Deserialize a snapshot from bytes. Returns None if the data is malformed.
pub fn deserialize_snapshot(bytes: &[u8]) -> Option<Snapshot> {
    todo!("Parse the byte format produced by serialize_snapshot")
}

/// Verify that the snapshot's hash matches its contents.
pub fn verify_snapshot(snapshot: &Snapshot) -> bool {
    todo!("Recompute the hash from snapshot.accounts and compare to snapshot.hash")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(pk_byte: u8, lamports: u64) -> AccountEntry {
        AccountEntry {
            pubkey: [pk_byte; 32],
            lamports,
            owner: [0xAA; 32],
            data: vec![pk_byte; 16],
            executable: false,
        }
    }

    #[test]
    fn test_generate_and_verify() {
        let accounts = vec![make_entry(3, 100), make_entry(1, 200), make_entry(2, 300)];
        let snap = generate_snapshot(42, accounts, 1000);
        assert_eq!(snap.slot, 42);
        assert_eq!(snap.created_at, 1000);
        assert!(verify_snapshot(&snap));
        // Accounts should be sorted by pubkey
        assert_eq!(snap.accounts[0].pubkey, [1u8; 32]);
        assert_eq!(snap.accounts[1].pubkey, [2u8; 32]);
        assert_eq!(snap.accounts[2].pubkey, [3u8; 32]);
    }

    #[test]
    fn test_deterministic_hash() {
        let a = vec![make_entry(2, 100), make_entry(1, 200)];
        let b = vec![make_entry(1, 200), make_entry(2, 100)];
        let snap_a = generate_snapshot(10, a, 500);
        let snap_b = generate_snapshot(10, b, 500);
        assert_eq!(snap_a.hash, snap_b.hash);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let accounts = vec![make_entry(5, 999), make_entry(10, 42)];
        let snap = generate_snapshot(77, accounts, 2000);
        let bytes = serialize_snapshot(&snap);
        let restored = deserialize_snapshot(&bytes).expect("should deserialize");
        assert_eq!(snap, restored);
    }

    #[test]
    fn test_tamper_detection() {
        let accounts = vec![make_entry(1, 100)];
        let mut snap = generate_snapshot(1, accounts, 0);
        assert!(verify_snapshot(&snap));
        // Tamper with lamports
        snap.accounts[0].lamports = 999_999;
        assert!(!verify_snapshot(&snap));
    }

    #[test]
    fn test_empty_snapshot() {
        let snap = generate_snapshot(0, vec![], 0);
        assert!(verify_snapshot(&snap));
        let bytes = serialize_snapshot(&snap);
        let restored = deserialize_snapshot(&bytes).unwrap();
        assert_eq!(restored.accounts.len(), 0);
        assert_eq!(snap.hash, restored.hash);
    }

    #[test]
    fn test_deserialize_malformed_returns_none() {
        assert!(deserialize_snapshot(&[]).is_none());
        assert!(deserialize_snapshot(&[0u8; 10]).is_none());
    }

    #[test]
    fn test_executable_flag_affects_hash() {
        let mut entry_a = make_entry(1, 100);
        entry_a.executable = false;
        let mut entry_b = make_entry(1, 100);
        entry_b.executable = true;
        let snap_a = generate_snapshot(1, vec![entry_a], 0);
        let snap_b = generate_snapshot(1, vec![entry_b], 0);
        assert_ne!(snap_a.hash, snap_b.hash);
    }
}
