//! # Challenge 2.3: Merkle Patricia Trie
//!
//! Implement Ethereum's state trie with insert, get, delete, and proof generation.
//! This is Ethereum's core state data structure — understanding it deeply is
//! essential for any execution layer role.
//!
//! Time: 90 min | Difficulty: Hard

pub struct MerklePatriciaTrie {
    _placeholder: (),
}

impl MerklePatriciaTrie {
    pub fn new() -> Self { todo!() }
    pub fn insert(&mut self, _key: &[u8], _value: &[u8]) { todo!() }
    pub fn get(&self, _key: &[u8]) -> Option<Vec<u8>> { todo!() }
    pub fn delete(&mut self, _key: &[u8]) -> bool { todo!() }
    pub fn root_hash(&self) -> [u8; 32] { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut trie = MerklePatriciaTrie::new();
        trie.insert(b"hello", b"world");
        assert_eq!(trie.get(b"hello"), Some(b"world".to_vec()));
    }

    #[test]
    fn test_missing_key() {
        let trie = MerklePatriciaTrie::new();
        assert_eq!(trie.get(b"nope"), None);
    }

    #[test]
    fn test_delete() {
        let mut trie = MerklePatriciaTrie::new();
        trie.insert(b"key", b"value");
        assert!(trie.delete(b"key"));
        assert_eq!(trie.get(b"key"), None);
    }

    #[test]
    fn test_root_changes_on_insert() {
        let mut trie = MerklePatriciaTrie::new();
        let root1 = trie.root_hash();
        trie.insert(b"key", b"value");
        let root2 = trie.root_hash();
        assert_ne!(root1, root2);
    }

    #[test]
    fn test_deterministic_root() {
        let mut trie1 = MerklePatriciaTrie::new();
        let mut trie2 = MerklePatriciaTrie::new();
        trie1.insert(b"a", b"1");
        trie1.insert(b"b", b"2");
        trie2.insert(b"b", b"2");
        trie2.insert(b"a", b"1");
        assert_eq!(trie1.root_hash(), trie2.root_hash(), "insertion order should not matter");
    }
}
