//! # Challenge 2.2: Merkle Tree
//!
//! ## Problem
//! Implement a binary Merkle tree with proof generation and verification.
//!
//! ## Why This Matters
//! Merkle trees are THE fundamental data structure of blockchain. They verify
//! data integrity in Ethereum state, Solana account proofs, Bitcoin transactions,
//! compressed NFTs — everything. If you can't implement and reason about Merkle
//! trees, you can't work on blockchain infrastructure.
//!
//! ## Requirements
//! - `MerkleTree::from_leaves(leaves)` — build tree from leaf data
//! - `root()` — return the Merkle root hash
//! - `proof(index)` — generate inclusion proof for leaf at index
//! - `verify(root, leaf, proof)` — verify a proof against a root (static method)
//!
//! ## Hash Function
//! Use Keccak-256 (Ethereum standard). For internal nodes: hash(left || right).
//! For leaves: hash(leaf_data).

use sha3::{Digest, Keccak256};

pub type Hash = [u8; 32];

pub fn keccak256(data: &[u8]) -> Hash {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[derive(Debug, Clone)]
pub struct MerkleProof {
    /// Sibling hashes from leaf to root.
    /// Each entry is (hash, is_left) where is_left means the sibling is on the left.
    pub siblings: Vec<(Hash, bool)>,
}

pub struct MerkleTree {
    // TODO: implement
    // Hint: Store all nodes in a flat Vec<Hash> (like a binary heap)
    // Or store layers: Vec<Vec<Hash>>
    _placeholder: (),
}

impl MerkleTree {
    /// Build a Merkle tree from leaf data.
    /// If the number of leaves is odd, duplicate the last leaf.
    pub fn from_leaves(_leaves: &[&[u8]]) -> Self {
        todo!("Hash leaves, then iteratively hash pairs up to root")
    }

    /// Return the Merkle root.
    pub fn root(&self) -> Hash {
        todo!("Return the root hash")
    }

    /// Generate a proof of inclusion for the leaf at the given index.
    pub fn proof(&self, _index: usize) -> MerkleProof {
        todo!("Collect sibling hashes from leaf level to root")
    }

    /// Verify a Merkle proof.
    pub fn verify(root: &Hash, leaf_data: &[u8], proof: &MerkleProof) -> bool {
        todo!("Recompute root from leaf + proof, compare to expected root")
    }

    /// Return the number of leaves.
    pub fn leaf_count(&self) -> usize {
        todo!("Return number of leaves")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_leaf() {
        let tree = MerkleTree::from_leaves(&[b"hello"]);
        let expected = keccak256(b"hello");
        assert_eq!(tree.root(), expected);
    }

    #[test]
    fn test_two_leaves() {
        let tree = MerkleTree::from_leaves(&[b"left", b"right"]);
        let left_hash = keccak256(b"left");
        let right_hash = keccak256(b"right");
        let mut combined = Vec::new();
        combined.extend_from_slice(&left_hash);
        combined.extend_from_slice(&right_hash);
        let expected_root = keccak256(&combined);
        assert_eq!(tree.root(), expected_root);
    }

    #[test]
    fn test_four_leaves() {
        let tree = MerkleTree::from_leaves(&[b"a", b"b", b"c", b"d"]);
        assert_eq!(tree.leaf_count(), 4);
        // Root should be deterministic
        let root = tree.root();
        let tree2 = MerkleTree::from_leaves(&[b"a", b"b", b"c", b"d"]);
        assert_eq!(tree2.root(), root);
    }

    #[test]
    fn test_proof_verification() {
        let leaves: Vec<&[u8]> = vec![b"tx1", b"tx2", b"tx3", b"tx4"];
        let tree = MerkleTree::from_leaves(&leaves);
        let root = tree.root();

        // Verify proof for each leaf
        for (i, leaf) in leaves.iter().enumerate() {
            let proof = tree.proof(i);
            assert!(
                MerkleTree::verify(&root, leaf, &proof),
                "proof should verify for leaf {}", i
            );
        }
    }

    #[test]
    fn test_proof_rejects_wrong_leaf() {
        let tree = MerkleTree::from_leaves(&[b"a", b"b", b"c", b"d"]);
        let root = tree.root();
        let proof = tree.proof(0); // Proof for leaf "a"
        // Verify with wrong leaf data should fail
        assert!(!MerkleTree::verify(&root, b"wrong", &proof));
    }

    #[test]
    fn test_proof_rejects_wrong_root() {
        let tree = MerkleTree::from_leaves(&[b"a", b"b", b"c", b"d"]);
        let proof = tree.proof(0);
        let wrong_root = [0xFFu8; 32];
        assert!(!MerkleTree::verify(&wrong_root, b"a", &proof));
    }

    #[test]
    fn test_odd_number_of_leaves() {
        // Odd leaf count: last leaf should be duplicated
        let tree = MerkleTree::from_leaves(&[b"a", b"b", b"c"]);
        let root = tree.root();
        let proof = tree.proof(2); // Proof for "c"
        assert!(MerkleTree::verify(&root, b"c", &proof));
    }

    #[test]
    fn test_large_tree() {
        let leaves: Vec<Vec<u8>> = (0u32..64).map(|i| i.to_le_bytes().to_vec()).collect();
        let leaf_refs: Vec<&[u8]> = leaves.iter().map(|l| l.as_slice()).collect();
        let tree = MerkleTree::from_leaves(&leaf_refs);
        assert_eq!(tree.leaf_count(), 64);

        // Spot check a few proofs
        for i in [0, 15, 31, 63] {
            let proof = tree.proof(i);
            assert!(MerkleTree::verify(&tree.root(), &leaf_refs[i], &proof));
        }
    }
}
