/// Challenge 01 - Property-Based Testing with Proptest
///
/// Define strategies to generate random account data and transactions,
/// then write property tests that verify invariants hold for ALL generated inputs.
///
/// Key properties:
/// - Transfers conserve total lamports (no creation/destruction of value).
/// - Transfers fail when the sender has insufficient funds.
/// - Serialize then deserialize is identity (roundtrip).

use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AccountData {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: [u8; 32],
    pub to: [u8; 32],
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferError {
    InsufficientFunds,
    SelfTransfer,
}

/// Validate that an account has reasonable fields.
pub fn validate_account(account: &AccountData) -> bool {
    // Data length should be under 10MB
    account.data.len() <= 10 * 1024 * 1024
}

/// Apply a transfer between two accounts. Returns error if insufficient funds.
pub fn apply_transfer(
    from: &mut AccountData,
    to: &mut AccountData,
    amount: u64,
) -> Result<(), TransferError> {
    if from.pubkey == to.pubkey {
        return Err(TransferError::SelfTransfer);
    }
    if from.lamports < amount {
        return Err(TransferError::InsufficientFunds);
    }
    from.lamports -= amount;
    to.lamports += amount;
    Ok(())
}

/// Serialize an account to bytes (simple format).
pub fn serialize_account(account: &AccountData) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&account.pubkey);
    buf.extend_from_slice(&account.lamports.to_le_bytes());
    buf.extend_from_slice(&(account.data.len() as u32).to_le_bytes());
    buf.extend_from_slice(&account.data);
    buf.extend_from_slice(&account.owner);
    buf
}

/// Deserialize an account from bytes.
pub fn deserialize_account(data: &[u8]) -> Option<AccountData> {
    if data.len() < 32 + 8 + 4 + 32 {
        return None;
    }
    let mut pos = 0;
    let mut pubkey = [0u8; 32];
    pubkey.copy_from_slice(&data[pos..pos + 32]);
    pos += 32;

    let lamports = u64::from_le_bytes(data[pos..pos + 8].try_into().ok()?);
    pos += 8;

    let data_len = u32::from_le_bytes(data[pos..pos + 4].try_into().ok()?) as usize;
    pos += 4;

    if data.len() < pos + data_len + 32 {
        return None;
    }
    let account_data = data[pos..pos + data_len].to_vec();
    pos += data_len;

    let mut owner = [0u8; 32];
    owner.copy_from_slice(&data[pos..pos + 32]);

    Some(AccountData {
        pubkey,
        lamports,
        data: account_data,
        owner,
    })
}

// === Student implements strategies and property tests below ===

/// Strategy to generate a random [u8; 32] pubkey.
fn arb_pubkey() -> impl Strategy<Value = [u8; 32]> {
    // TODO: Generate a random 32-byte array
    // Hint: prop::array::uniform32(any::<u8>())
    todo!("Implement arb_pubkey strategy")
}

/// Strategy to generate a random AccountData.
fn arb_account() -> impl Strategy<Value = AccountData> {
    // TODO: Combine arb_pubkey, any::<u64>, prop::collection::vec, arb_pubkey
    // to generate an AccountData
    todo!("Implement arb_account strategy")
}

/// Strategy to generate a random Transaction.
fn arb_transaction() -> impl Strategy<Value = Transaction> {
    // TODO: Generate from, to, and amount
    todo!("Implement arb_transaction strategy")
}

proptest! {
    /// Property: transfer conserves total lamports.
    #[test]
    fn prop_transfer_conserves_total(
        // TODO: use arb_account() to generate two accounts and any::<u64>() for amount
        a_lamports in 0u64..1_000_000,
        b_lamports in 0u64..1_000_000,
        amount in 0u64..1_000_000,
    ) {
        // TODO: Implement property
        // 1. Create two accounts with given lamports
        // 2. Record total = a.lamports + b.lamports
        // 3. Apply transfer (ignore errors)
        // 4. Assert total is unchanged
        todo!("Implement prop_transfer_conserves_total")
    }

    /// Property: transfer fails when sender has insufficient funds.
    #[test]
    fn prop_transfer_fails_on_insufficient(
        balance in 0u64..1000,
        extra in 1u64..1000,
    ) {
        // TODO: Create account with `balance`, try to transfer `balance + extra`
        // Assert that it returns InsufficientFunds
        todo!("Implement prop_transfer_fails_on_insufficient")
    }

    /// Property: serialize then deserialize is identity.
    #[test]
    fn prop_serialize_roundtrip(
        lamports in any::<u64>(),
        data_len in 0usize..256,
    ) {
        // TODO: Create an account, serialize it, deserialize it, assert equality
        todo!("Implement prop_serialize_roundtrip")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_account() {
        let account = AccountData {
            pubkey: [0u8; 32],
            lamports: 1000,
            data: vec![0; 100],
            owner: [1u8; 32],
        };
        assert!(validate_account(&account));
    }

    #[test]
    fn test_apply_transfer_basic() {
        let mut from = AccountData { pubkey: [1u8; 32], lamports: 1000, data: vec![], owner: [0u8; 32] };
        let mut to = AccountData { pubkey: [2u8; 32], lamports: 500, data: vec![], owner: [0u8; 32] };
        apply_transfer(&mut from, &mut to, 300).unwrap();
        assert_eq!(from.lamports, 700);
        assert_eq!(to.lamports, 800);
    }

    #[test]
    fn test_self_transfer_rejected() {
        let mut acc = AccountData { pubkey: [1u8; 32], lamports: 1000, data: vec![], owner: [0u8; 32] };
        let mut acc2 = acc.clone();
        let result = apply_transfer(&mut acc, &mut acc2, 100);
        assert_eq!(result, Err(TransferError::SelfTransfer));
    }

    #[test]
    fn test_serialize_roundtrip() {
        let account = AccountData {
            pubkey: [42u8; 32],
            lamports: 123456,
            data: vec![1, 2, 3, 4, 5],
            owner: [99u8; 32],
        };
        let bytes = serialize_account(&account);
        let recovered = deserialize_account(&bytes).unwrap();
        assert_eq!(account, recovered);
    }
}
