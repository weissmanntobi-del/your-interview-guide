/// # Challenge 05: Token Account Index — SPL Token Account Indexing
///
/// Solana's SPL Token program stores token balances in individual "token accounts,"
/// each associated with a mint (token type) and an owner (wallet). RPCs like
/// getTokenAccountsByOwner and getTokenLargestAccounts require efficient indexes
/// over these accounts. This challenge builds a TokenAccountIndex that supports
/// multi-key lookups (by owner, by mint, by both), ranked queries (largest holders),
/// and aggregate queries (total supply).
///
/// Key concepts:
/// - Token account structure: address, mint, owner, amount, delegate, state
/// - Multi-index: O(1) lookups by owner and by mint using HashMaps of Vecs
/// - Ranked queries: get top-N holders sorted by amount descending
/// - Aggregate: total supply = sum of all amounts for a given mint

use std::collections::HashMap;

/// State of an SPL token account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountState {
    Initialized,
    Frozen,
}

/// An SPL token account holding a balance of a specific mint.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenAccount {
    pub address: [u8; 32],
    pub mint: [u8; 32],
    pub owner: [u8; 32],
    pub amount: u64,
    pub delegate: Option<[u8; 32]>,
    pub state: AccountState,
}

/// An index over token accounts supporting efficient multi-key lookups.
pub struct TokenAccountIndex {
    /// All accounts keyed by their address.
    accounts: HashMap<[u8; 32], TokenAccount>,
    /// Addresses of token accounts grouped by owner.
    by_owner: HashMap<[u8; 32], Vec<[u8; 32]>>,
    /// Addresses of token accounts grouped by mint.
    by_mint: HashMap<[u8; 32], Vec<[u8; 32]>>,
}

impl TokenAccountIndex {
    /// Create a new empty index.
    pub fn new() -> Self {
        todo!("Initialize with empty HashMaps for accounts, by_owner, and by_mint")
    }

    /// Index a token account. If an account with the same address already exists,
    /// remove the old entry from the owner/mint indexes before inserting the new one.
    pub fn index_account(&mut self, account: TokenAccount) {
        todo!("If address exists, remove old index entries. Insert account into accounts map. Add address to by_owner[owner] and by_mint[mint] vectors.")
    }

    /// Remove a token account by address. Returns true if found and removed.
    /// Also cleans up the owner and mint index entries.
    pub fn remove_account(&mut self, address: &[u8; 32]) -> bool {
        todo!("Look up account by address. If found, remove from by_owner and by_mint vectors, then remove from accounts map. Return true/false.")
    }

    /// Get all token accounts owned by the given owner pubkey.
    pub fn get_by_owner(&self, owner: &[u8; 32]) -> Vec<&TokenAccount> {
        todo!("Look up addresses in by_owner, then resolve each to a TokenAccount reference from accounts map")
    }

    /// Get all token accounts for a given mint.
    pub fn get_by_mint(&self, mint: &[u8; 32]) -> Vec<&TokenAccount> {
        todo!("Look up addresses in by_mint, then resolve each to a TokenAccount reference from accounts map")
    }

    /// Get token accounts that match both the given owner AND mint.
    pub fn get_by_owner_and_mint(
        &self,
        owner: &[u8; 32],
        mint: &[u8; 32],
    ) -> Vec<&TokenAccount> {
        todo!("Get accounts by owner, then filter to those matching the given mint")
    }

    /// Return the top N token accounts by amount for a given mint, sorted
    /// descending by amount.
    pub fn get_largest_holders(&self, mint: &[u8; 32], n: usize) -> Vec<&TokenAccount> {
        todo!("Get all accounts for mint, sort by amount descending, take first n")
    }

    /// Compute the total supply for a mint (sum of all token account amounts).
    pub fn total_supply(&self, mint: &[u8; 32]) -> u64 {
        todo!("Sum the amount field of all token accounts for the given mint")
    }

    /// Return total number of indexed accounts.
    pub fn len(&self) -> usize {
        todo!("Return accounts map length")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_account(addr: u8, mint: u8, owner: u8, amount: u64) -> TokenAccount {
        TokenAccount {
            address: [addr; 32],
            mint: [mint; 32],
            owner: [owner; 32],
            amount,
            delegate: None,
            state: AccountState::Initialized,
        }
    }

    #[test]
    fn test_index_and_get_by_owner() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 500));
        idx.index_account(make_account(2, 10, 100, 300));
        idx.index_account(make_account(3, 10, 200, 100));
        let owned = idx.get_by_owner(&[100u8; 32]);
        assert_eq!(owned.len(), 2);
    }

    #[test]
    fn test_get_by_mint() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 500));
        idx.index_account(make_account(2, 20, 100, 300));
        idx.index_account(make_account(3, 10, 200, 100));
        let by_mint = idx.get_by_mint(&[10u8; 32]);
        assert_eq!(by_mint.len(), 2);
    }

    #[test]
    fn test_get_by_owner_and_mint() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 500));
        idx.index_account(make_account(2, 20, 100, 300));
        idx.index_account(make_account(3, 10, 100, 100));
        let matches = idx.get_by_owner_and_mint(&[100u8; 32], &[10u8; 32]);
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_largest_holders_sorted() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 100));
        idx.index_account(make_account(2, 10, 101, 500));
        idx.index_account(make_account(3, 10, 102, 300));
        idx.index_account(make_account(4, 10, 103, 800));
        let top = idx.get_largest_holders(&[10u8; 32], 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].amount, 800);
        assert_eq!(top[1].amount, 500);
    }

    #[test]
    fn test_total_supply() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 100));
        idx.index_account(make_account(2, 10, 101, 200));
        idx.index_account(make_account(3, 10, 102, 300));
        assert_eq!(idx.total_supply(&[10u8; 32]), 600);
    }

    #[test]
    fn test_remove_updates_indexes() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 500));
        idx.index_account(make_account(2, 10, 100, 300));
        assert!(idx.remove_account(&[1u8; 32]));
        assert_eq!(idx.get_by_owner(&[100u8; 32]).len(), 1);
        assert_eq!(idx.get_by_mint(&[10u8; 32]).len(), 1);
        assert_eq!(idx.total_supply(&[10u8; 32]), 300);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut idx = TokenAccountIndex::new();
        assert!(!idx.remove_account(&[99u8; 32]));
    }

    #[test]
    fn test_reindex_account_updates() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 500));
        // Re-index same address with new owner
        let mut updated = make_account(1, 10, 200, 999);
        idx.index_account(updated);
        assert_eq!(idx.get_by_owner(&[100u8; 32]).len(), 0);
        assert_eq!(idx.get_by_owner(&[200u8; 32]).len(), 1);
        assert_eq!(idx.len(), 1);
    }

    #[test]
    fn test_total_supply_empty_mint() {
        let idx = TokenAccountIndex::new();
        assert_eq!(idx.total_supply(&[99u8; 32]), 0);
    }

    #[test]
    fn test_largest_holders_fewer_than_n() {
        let mut idx = TokenAccountIndex::new();
        idx.index_account(make_account(1, 10, 100, 500));
        let top = idx.get_largest_holders(&[10u8; 32], 5);
        assert_eq!(top.len(), 1);
    }
}
