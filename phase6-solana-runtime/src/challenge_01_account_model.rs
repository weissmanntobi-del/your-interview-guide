//! # Challenge 7.1: Solana Account Model
//!
//! ## Problem
//! Implement Solana's account model with ownership, rent, lamport transfers,
//! and Program Derived Addresses (PDAs).
//!
//! ## Why This Matters
//! Everything on Solana is an account. Understanding the account model —
//! ownership, rent, executable flags, data layout — is the foundation of
//! all Solana development, especially at the runtime level.
//!
//! ## Requirements
//! - Account struct with: pubkey, lamports, data, owner, executable, rent_epoch
//! - Transfer lamports between accounts (with validation)
//! - Assign account ownership
//! - PDA derivation: find_program_address(seeds, program_id)
//! - Rent calculation: minimum balance for rent exemption

use sha2::{Sha256, Digest};

pub type Pubkey = [u8; 32];

pub const SYSTEM_PROGRAM_ID: Pubkey = [0u8; 32];

#[derive(Debug, Clone)]
pub struct Account {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, PartialEq)]
pub enum AccountError {
    InsufficientLamports { required: u64, available: u64 },
    AccountNotOwned,
    InvalidPdaSeeds,
    AccountAlreadyExists,
}

impl Account {
    pub fn new_system_account(_pubkey: Pubkey, _lamports: u64) -> Self {
        todo!("Create account owned by System Program with empty data")
    }

    pub fn new_program_account(_pubkey: Pubkey, _data: Vec<u8>, _owner: Pubkey) -> Self {
        todo!("Create non-executable account owned by a program")
    }
}

/// Transfer lamports from one account to another.
/// The `from` account must have sufficient balance.
pub fn transfer(
    _from: &mut Account,
    _to: &mut Account,
    _amount: u64,
) -> Result<(), AccountError> {
    todo!("Debit from, credit to, check sufficient balance")
}

/// Assign a new owner to an account.
/// Only the current owner (System Program) can reassign.
pub fn assign(
    _account: &mut Account,
    _new_owner: Pubkey,
) -> Result<(), AccountError> {
    todo!("Change owner, only if current owner is System Program")
}

/// Derive a Program Derived Address (PDA).
/// Returns (derived_address, bump_seed).
/// PDAs must NOT be on the ed25519 curve — we try bump seeds from 255 down to 0.
///
/// For simplicity, we'll use SHA-256 and check if the hash has a specific property
/// (real Solana checks if the point is on the curve; we'll simulate by checking
/// if the first byte is even — this is a simplification for the exercise).
pub fn find_program_address(
    _seeds: &[&[u8]],
    _program_id: &Pubkey,
) -> Result<(Pubkey, u8), AccountError> {
    todo!("Try bump from 255 to 0, hash(seeds + [bump] + program_id), return first valid PDA")
}

/// Calculate minimum lamports for rent exemption.
/// Formula: (base_rent + data_len * per_byte_cost) * exemption_multiplier
pub fn minimum_balance_for_rent_exemption(_data_len: usize) -> u64 {
    const LAMPORTS_PER_BYTE_YEAR: u64 = 3480;
    const BASE_RENT_PER_YEAR: u64 = 2439600;
    const EXEMPTION_YEARS: u64 = 2;

    todo!("Calculate: (base + data_len * per_byte) * exemption_years")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_system_account() {
        let acc = Account::new_system_account([1u8; 32], 1_000_000);
        assert_eq!(acc.lamports, 1_000_000);
        assert_eq!(acc.owner, SYSTEM_PROGRAM_ID);
        assert!(acc.data.is_empty());
        assert!(!acc.executable);
    }

    #[test]
    fn test_transfer_success() {
        let mut from = Account::new_system_account([1u8; 32], 1000);
        let mut to = Account::new_system_account([2u8; 32], 500);
        transfer(&mut from, &mut to, 300).unwrap();
        assert_eq!(from.lamports, 700);
        assert_eq!(to.lamports, 800);
    }

    #[test]
    fn test_transfer_insufficient() {
        let mut from = Account::new_system_account([1u8; 32], 100);
        let mut to = Account::new_system_account([2u8; 32], 0);
        let err = transfer(&mut from, &mut to, 200).unwrap_err();
        assert_eq!(
            err,
            AccountError::InsufficientLamports {
                required: 200,
                available: 100
            }
        );
    }

    #[test]
    fn test_assign_owner() {
        let mut acc = Account::new_system_account([1u8; 32], 0);
        let new_owner = [0xAA; 32];
        assign(&mut acc, new_owner).unwrap();
        assert_eq!(acc.owner, new_owner);
    }

    #[test]
    fn test_assign_fails_if_not_system_owned() {
        let mut acc = Account::new_program_account([1u8; 32], vec![], [0xBB; 32]);
        let err = assign(&mut acc, [0xCC; 32]).unwrap_err();
        assert_eq!(err, AccountError::AccountNotOwned);
    }

    #[test]
    fn test_pda_derivation() {
        let program_id = [0xAA; 32];
        let (pda, bump) = find_program_address(&[b"seed"], &program_id).unwrap();
        // PDA should be deterministic
        let (pda2, bump2) = find_program_address(&[b"seed"], &program_id).unwrap();
        assert_eq!(pda, pda2);
        assert_eq!(bump, bump2);
        // Different seeds should give different PDAs
        let (pda3, _) = find_program_address(&[b"other"], &program_id).unwrap();
        assert_ne!(pda, pda3);
    }

    #[test]
    fn test_rent_exemption() {
        let min_balance_0 = minimum_balance_for_rent_exemption(0);
        let min_balance_100 = minimum_balance_for_rent_exemption(100);
        assert!(min_balance_100 > min_balance_0);
        assert!(min_balance_0 > 0);
    }
}
