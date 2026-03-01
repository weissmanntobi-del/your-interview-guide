/// Challenge 03 - Program Test Harness (LiteSVM-style)
///
/// Build a simplified test harness for executing "programs" (functions) against
/// a set of accounts, similar to how Solana's LiteSVM or ProgramTest works.
///
/// The harness tracks accounts, validates signers and writable permissions,
/// and dispatches to registered program handlers.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TestAccount {
    pub pubkey: [u8; 32],
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: [u8; 32],
    pub executable: bool,
}

#[derive(Debug, Clone)]
pub struct TestInstruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<TestAccountMeta>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TestAccountMeta {
    pub pubkey: [u8; 32],
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramError {
    AccountNotFound,
    InvalidSigner,
    ReadonlyViolation,
    ProgramNotFound,
    Custom(u32),
}

/// A handler function receives: (program_id, accounts, instruction_data)
/// and returns Ok(modified_accounts) or Err(ProgramError).
type ProgramHandler = Box<
    dyn Fn(&[u8; 32], &mut [TestAccount], &[u8]) -> Result<(), ProgramError>,
>;

pub struct ProgramTestHarness {
    accounts: HashMap<[u8; 32], TestAccount>,
    programs: HashMap<[u8; 32], ProgramHandler>,
    current_slot: u64,
}

impl ProgramTestHarness {
    pub fn new() -> Self {
        ProgramTestHarness {
            accounts: HashMap::new(),
            programs: HashMap::new(),
            current_slot: 0,
        }
    }

    /// Add or update an account in the harness.
    pub fn add_account(&mut self, account: TestAccount) {
        self.accounts.insert(account.pubkey, account);
    }

    /// Register a program handler for a given program_id.
    pub fn add_program(&mut self, program_id: [u8; 32], handler: ProgramHandler) {
        self.programs.insert(program_id, handler);
    }

    /// Process a single instruction.
    /// 1. Look up the program handler (ProgramNotFound if missing).
    /// 2. Gather referenced accounts (AccountNotFound if missing).
    /// 3. Validate signers: all accounts marked is_signer must be present.
    /// 4. Call the handler with mutable account references.
    /// 5. After handler returns, check that non-writable accounts were not modified.
    /// 6. Persist modified accounts back to state.
    pub fn process_instruction(
        &mut self,
        instruction: &TestInstruction,
    ) -> Result<(), ProgramError> {
        // TODO: Implement instruction processing
        // 1. Check program exists
        // 2. Collect accounts referenced by instruction.accounts
        // 3. Validate all is_signer accounts exist
        // 4. Clone accounts for comparison (to detect readonly violations)
        // 5. Call handler
        // 6. Check no readonly account was modified (compare with clones)
        // 7. Write modified accounts back
        todo!("Implement process_instruction")
    }

    /// Get a reference to an account by pubkey.
    pub fn get_account(&self, pubkey: &[u8; 32]) -> Option<&TestAccount> {
        self.accounts.get(pubkey)
    }

    /// Advance the slot counter by 1.
    pub fn advance_slot(&mut self) -> u64 {
        self.current_slot += 1;
        self.current_slot
    }

    pub fn current_slot(&self) -> u64 {
        self.current_slot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn system_program_id() -> [u8; 32] { [0u8; 32] }
    fn alice() -> [u8; 32] { [1u8; 32] }
    fn bob() -> [u8; 32] { [2u8; 32] }

    /// A simple "transfer" program handler.
    fn transfer_handler() -> ProgramHandler {
        Box::new(|_program_id, accounts, data| {
            if data.len() < 8 {
                return Err(ProgramError::Custom(1));
            }
            let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
            if accounts.len() < 2 {
                return Err(ProgramError::Custom(2));
            }
            if accounts[0].lamports < amount {
                return Err(ProgramError::Custom(3)); // insufficient funds
            }
            accounts[0].lamports -= amount;
            accounts[1].lamports += amount;
            Ok(())
        })
    }

    #[test]
    fn test_simple_transfer() {
        let mut harness = ProgramTestHarness::new();
        harness.add_account(TestAccount {
            pubkey: alice(), lamports: 1000, data: vec![], owner: system_program_id(), executable: false,
        });
        harness.add_account(TestAccount {
            pubkey: bob(), lamports: 500, data: vec![], owner: system_program_id(), executable: false,
        });
        harness.add_program(system_program_id(), transfer_handler());

        let ix = TestInstruction {
            program_id: system_program_id(),
            accounts: vec![
                TestAccountMeta { pubkey: alice(), is_signer: true, is_writable: true },
                TestAccountMeta { pubkey: bob(), is_signer: false, is_writable: true },
            ],
            data: 300u64.to_le_bytes().to_vec(),
        };
        harness.process_instruction(&ix).unwrap();
        assert_eq!(harness.get_account(&alice()).unwrap().lamports, 700);
        assert_eq!(harness.get_account(&bob()).unwrap().lamports, 800);
    }

    #[test]
    fn test_signer_validation() {
        let mut harness = ProgramTestHarness::new();
        harness.add_account(TestAccount {
            pubkey: alice(), lamports: 1000, data: vec![], owner: system_program_id(), executable: false,
        });
        harness.add_program(system_program_id(), Box::new(|_, accounts, _| {
            // Program that requires a signer - just checks it was called
            Ok(())
        }));

        // Mark alice as NOT a signer but instruction requires it
        let ix = TestInstruction {
            program_id: system_program_id(),
            accounts: vec![
                TestAccountMeta { pubkey: alice(), is_signer: true, is_writable: false },
                // Reference a non-existent signer
                TestAccountMeta { pubkey: [99u8; 32], is_signer: true, is_writable: false },
            ],
            data: vec![],
        };
        let result = harness.process_instruction(&ix);
        assert_eq!(result, Err(ProgramError::AccountNotFound));
    }

    #[test]
    fn test_readonly_violation() {
        let mut harness = ProgramTestHarness::new();
        harness.add_account(TestAccount {
            pubkey: alice(), lamports: 1000, data: vec![], owner: system_program_id(), executable: false,
        });
        // Program that modifies the account regardless
        harness.add_program(system_program_id(), Box::new(|_, accounts, _| {
            accounts[0].lamports = 9999;
            Ok(())
        }));

        let ix = TestInstruction {
            program_id: system_program_id(),
            accounts: vec![
                TestAccountMeta { pubkey: alice(), is_signer: false, is_writable: false }, // readonly!
            ],
            data: vec![],
        };
        let result = harness.process_instruction(&ix);
        assert_eq!(result, Err(ProgramError::ReadonlyViolation));
    }

    #[test]
    fn test_program_not_found() {
        let mut harness = ProgramTestHarness::new();
        let ix = TestInstruction {
            program_id: [99u8; 32], // not registered
            accounts: vec![],
            data: vec![],
        };
        let result = harness.process_instruction(&ix);
        assert_eq!(result, Err(ProgramError::ProgramNotFound));
    }

    #[test]
    fn test_state_persists_across_instructions() {
        let mut harness = ProgramTestHarness::new();
        harness.add_account(TestAccount {
            pubkey: alice(), lamports: 1000, data: vec![], owner: system_program_id(), executable: false,
        });
        harness.add_account(TestAccount {
            pubkey: bob(), lamports: 0, data: vec![], owner: system_program_id(), executable: false,
        });
        harness.add_program(system_program_id(), transfer_handler());

        let ix = TestInstruction {
            program_id: system_program_id(),
            accounts: vec![
                TestAccountMeta { pubkey: alice(), is_signer: true, is_writable: true },
                TestAccountMeta { pubkey: bob(), is_signer: false, is_writable: true },
            ],
            data: 100u64.to_le_bytes().to_vec(),
        };
        // Two transfers of 100 each
        harness.process_instruction(&ix).unwrap();
        harness.process_instruction(&ix).unwrap();
        assert_eq!(harness.get_account(&alice()).unwrap().lamports, 800);
        assert_eq!(harness.get_account(&bob()).unwrap().lamports, 200);
    }

    #[test]
    fn test_advance_slot() {
        let mut harness = ProgramTestHarness::new();
        assert_eq!(harness.current_slot(), 0);
        assert_eq!(harness.advance_slot(), 1);
        assert_eq!(harness.advance_slot(), 2);
        assert_eq!(harness.current_slot(), 2);
    }
}
