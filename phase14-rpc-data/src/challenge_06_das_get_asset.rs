/// # Challenge 06: DAS getAsset — Digital Asset Standard for Compressed NFTs
///
/// Compressed NFTs on Solana use Merkle trees (via State Compression) to store
/// asset data off-chain while keeping a root hash on-chain. The Digital Asset
/// Standard (DAS) API provides methods like getAsset and getAssetsByOwner that
/// query this data. Verifying ownership requires validating a Merkle proof
/// against the on-chain root. This challenge implements an AssetStore with
/// insertion, retrieval, ownership queries, Merkle proof verification using SHA-256,
/// and asset transfers.
///
/// Key concepts:
/// - Compressed NFTs: data stored off-chain, Merkle root on-chain
/// - Merkle proof verification: hash leaf with siblings up to root
/// - SHA-256 hashing for proof nodes
/// - Asset transfers update owner and data_hash

use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// A compressed NFT asset stored off-chain.
#[derive(Debug, Clone, PartialEq)]
pub struct CompressedAsset {
    pub id: [u8; 32],
    pub owner: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub leaf_index: u32,
    pub tree_id: [u8; 32],
}

/// A Merkle proof for a leaf in a concurrent Merkle tree.
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleProof {
    pub leaf: [u8; 32],
    pub proof: Vec<[u8; 32]>,
    pub root: [u8; 32],
    pub leaf_index: u32,
}

/// Result of a verified asset lookup.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetResponse {
    pub id: String,
    pub owner: String,
    pub data_hash: String,
    pub creator_hash: String,
    pub leaf_index: u32,
    pub tree_id: String,
}

/// Stores compressed assets and supports DAS-style queries.
pub struct AssetStore {
    assets: HashMap<[u8; 32], CompressedAsset>,
}

impl AssetStore {
    /// Create a new empty AssetStore.
    pub fn new() -> Self {
        todo!("Initialize with empty assets HashMap")
    }

    /// Insert a compressed asset into the store.
    pub fn insert_asset(&mut self, asset: CompressedAsset) {
        todo!("Insert the asset keyed by its id")
    }

    /// Retrieve an asset by its 32-byte id and format it as an AssetResponse.
    /// Returns None if the asset does not exist.
    pub fn get_asset(&self, id: &[u8; 32]) -> Option<AssetResponse> {
        todo!("Look up asset by id. If found, hex-encode all byte fields and build AssetResponse. Return None if missing.")
    }

    /// Get all assets owned by the given owner, returned as AssetResponses.
    pub fn get_assets_by_owner(&self, owner: &[u8; 32]) -> Vec<AssetResponse> {
        todo!("Filter all assets by owner match, convert each to AssetResponse, collect and return")
    }

    /// Transfer an asset to a new owner. Returns false if asset not found.
    /// Updates the owner field of the stored asset.
    pub fn transfer_asset(&mut self, id: &[u8; 32], new_owner: [u8; 32]) -> bool {
        todo!("Look up asset by id. If found, update its owner to new_owner and return true. Otherwise return false.")
    }

    /// Return the total number of stored assets.
    pub fn len(&self) -> usize {
        todo!("Return assets map length")
    }
}

/// Hex-encode a byte slice into a lowercase hex string.
fn hex_encode(bytes: &[u8]) -> String {
    todo!("Map each byte to two lowercase hex characters and collect into String")
}

/// Compute SHA-256 hash of the concatenation of two 32-byte values.
///
/// This is the core operation for Merkle proof verification: hashing a node
/// with its sibling to produce the parent.
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    todo!("Create Sha256 hasher, update with left then right, finalize, convert to [u8; 32]")
}

/// Verify a Merkle proof by walking from the leaf up to the root.
///
/// At each level, determine whether the current hash is the left or right child
/// based on the leaf_index bit at that level. Hash the pair and move up.
/// After processing all proof nodes, compare the result with the expected root.
pub fn verify_proof(proof: &MerkleProof) -> bool {
    todo!("Start with current_hash = proof.leaf. For each (i, sibling) in proof.proof, check bit i of leaf_index: if 0, hash(current, sibling), if 1, hash(sibling, current). After all levels, compare current_hash == proof.root.")
}

/// Build a simple Merkle tree from a list of leaves and return the root.
/// Leaves are hashed pairwise; if odd count, the last leaf is duplicated.
pub fn build_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    todo!("Iteratively hash pairs of nodes bottom-up. If odd number, duplicate last node. Continue until one root remains.")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_asset(id: u8, owner: u8, leaf_index: u32) -> CompressedAsset {
        CompressedAsset {
            id: [id; 32],
            owner: [owner; 32],
            data_hash: [id.wrapping_add(1); 32],
            creator_hash: [id.wrapping_add(2); 32],
            leaf_index,
            tree_id: [0xAA; 32],
        }
    }

    #[test]
    fn test_insert_and_retrieve() {
        let mut store = AssetStore::new();
        store.insert_asset(make_asset(1, 10, 0));
        let resp = store.get_asset(&[1u8; 32]);
        assert!(resp.is_some());
        let resp = resp.unwrap();
        assert_eq!(resp.leaf_index, 0);
    }

    #[test]
    fn test_get_nonexistent() {
        let store = AssetStore::new();
        assert!(store.get_asset(&[99u8; 32]).is_none());
    }

    #[test]
    fn test_get_by_owner() {
        let mut store = AssetStore::new();
        store.insert_asset(make_asset(1, 10, 0));
        store.insert_asset(make_asset(2, 10, 1));
        store.insert_asset(make_asset(3, 20, 2));
        let owned = store.get_assets_by_owner(&[10u8; 32]);
        assert_eq!(owned.len(), 2);
    }

    #[test]
    fn test_transfer_asset() {
        let mut store = AssetStore::new();
        store.insert_asset(make_asset(1, 10, 0));
        assert!(store.transfer_asset(&[1u8; 32], [20u8; 32]));
        let resp = store.get_asset(&[1u8; 32]).unwrap();
        assert_eq!(resp.owner, hex_encode(&[20u8; 32]));
        assert_eq!(store.get_assets_by_owner(&[10u8; 32]).len(), 0);
    }

    #[test]
    fn test_transfer_nonexistent() {
        let mut store = AssetStore::new();
        assert!(!store.transfer_asset(&[99u8; 32], [20u8; 32]));
    }

    #[test]
    fn test_hash_pair_deterministic() {
        let a = [0x01u8; 32];
        let b = [0x02u8; 32];
        let h1 = hash_pair(&a, &b);
        let h2 = hash_pair(&a, &b);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hash_pair_order_matters() {
        let a = [0x01u8; 32];
        let b = [0x02u8; 32];
        assert_ne!(hash_pair(&a, &b), hash_pair(&b, &a));
    }

    #[test]
    fn test_verify_valid_proof() {
        // Build a 4-leaf tree and verify leaf 0
        let leaves: Vec<[u8; 32]> = (0u8..4).map(|i| [i; 32]).collect();
        let root = build_merkle_root(&leaves);

        // Proof for leaf 0: sibling is leaf 1, then hash of (leaf2, leaf3)
        let sibling_0 = leaves[1];
        let parent_right = hash_pair(&leaves[2], &leaves[3]);
        let proof = MerkleProof {
            leaf: leaves[0],
            proof: vec![sibling_0, parent_right],
            root,
            leaf_index: 0,
        };
        assert!(verify_proof(&proof));
    }

    #[test]
    fn test_reject_invalid_proof() {
        let leaves: Vec<[u8; 32]> = (0u8..4).map(|i| [i; 32]).collect();
        let root = build_merkle_root(&leaves);

        let proof = MerkleProof {
            leaf: leaves[0],
            proof: vec![[0xFFu8; 32], [0xFFu8; 32]],
            root,
            leaf_index: 0,
        };
        assert!(!verify_proof(&proof));
    }

    #[test]
    fn test_build_merkle_root_two_leaves() {
        let a = [0x01u8; 32];
        let b = [0x02u8; 32];
        let root = build_merkle_root(&[a, b]);
        assert_eq!(root, hash_pair(&a, &b));
    }

    #[test]
    fn test_hex_encode_known() {
        let bytes = [0xAB, 0xCD];
        assert_eq!(hex_encode(&bytes), "abcd");
    }
}
