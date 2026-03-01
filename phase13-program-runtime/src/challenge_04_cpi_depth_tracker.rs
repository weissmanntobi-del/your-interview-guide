/// # Challenge 04: CPI Depth Tracker with Privilege Escalation Detection
///
/// Cross-Program Invocation (CPI) allows one Solana program to call another. The
/// runtime must enforce:
///
/// 1. **Maximum call depth** -- currently 4 on Solana mainnet. Exceeding this limit
///    is a fatal error.
/// 2. **No reentrancy** -- a program may not appear more than once on the call stack
///    (i.e., recursive / reentrant calls are forbidden).
/// 3. **Privilege de-escalation** -- the callee may NOT gain privileges that the
///    caller did not have. Specifically:
///    - An account that is not a signer in the caller context must not become a
///      signer in the callee context.
///    - An account that is not writable in the caller context must not become
///      writable in the callee context.
///
/// This challenge models the call stack and privilege checks.

/// Errors that can occur during CPI tracking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpiError {
    /// The maximum call depth has been exceeded.
    MaxDepthExceeded { max: usize },
    /// Attempted to exit (pop) an empty call stack.
    EmptyStack,
    /// The program being called is already on the call stack (reentrancy).
    ReentrantCall { program_id: [u8; 32] },
    /// The callee is attempting to escalate a privilege the caller did not grant.
    PrivilegeEscalation {
        account: [u8; 32],
        privilege: String,
    },
}

/// Represents a single CPI frame on the call stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpiCall {
    /// Program ID of the caller.
    pub caller: [u8; 32],
    /// Program ID of the callee being invoked.
    pub callee: [u8; 32],
    /// Instruction data passed to the callee.
    pub instruction_data: Vec<u8>,
}

/// Describes an account's privileges as seen by a program invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountPrivilege {
    pub key: [u8; 32],
    pub is_signer: bool,
    pub is_writable: bool,
}

/// Tracks CPI call depth and enforces reentrancy / depth limits.
pub struct CpiTracker {
    max_depth: usize,
    stack: Vec<CpiCall>,
}

impl CpiTracker {
    /// Create a new tracker with the given maximum call depth.
    pub fn new(max_depth: usize) -> Self {
        todo!()
    }

    /// Push a new CPI call onto the stack. Returns the new depth (1-indexed)
    /// on success.
    ///
    /// Fails if:
    /// - The new depth would exceed `max_depth`.
    /// - The callee program ID is already present on the stack (reentrancy).
    pub fn enter(&mut self, call: CpiCall) -> Result<usize, CpiError> {
        todo!()
    }

    /// Pop the most recent CPI call from the stack, returning it.
    ///
    /// Fails if the stack is empty.
    pub fn exit(&mut self) -> Result<CpiCall, CpiError> {
        todo!()
    }

    /// Return the current call depth (0 when no CPI is active).
    pub fn current_depth(&self) -> usize {
        todo!()
    }

    /// Return a read-only view of the current call stack.
    pub fn call_stack(&self) -> &[CpiCall] {
        todo!()
    }

    /// Check whether the given `program_id` already appears as a callee on
    /// the current call stack, which would indicate a reentrant call.
    pub fn is_reentrant(&self, program_id: &[u8; 32]) -> bool {
        todo!()
    }
}

/// Check that the `callee_accounts` do not escalate any privilege that the
/// `caller_accounts` did not already have.
///
/// For every account present in both lists (matched by `key`):
/// - If the callee has `is_signer = true` but the caller does not, that is an
///   escalation.
/// - If the callee has `is_writable = true` but the caller does not, that is an
///   escalation.
///
/// Accounts present in only one list are ignored.
pub fn check_privilege_escalation(
    caller_accounts: &[AccountPrivilege],
    callee_accounts: &[AccountPrivilege],
) -> Result<(), CpiError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(id: u8) -> [u8; 32] {
        [id; 32]
    }

    fn make_call(caller: u8, callee: u8) -> CpiCall {
        CpiCall {
            caller: pid(caller),
            callee: pid(callee),
            instruction_data: vec![caller, callee],
        }
    }

    #[test]
    fn test_basic_enter_exit() {
        let mut tracker = CpiTracker::new(4);
        assert_eq!(tracker.current_depth(), 0);

        let depth = tracker.enter(make_call(1, 2)).unwrap();
        assert_eq!(depth, 1);
        assert_eq!(tracker.current_depth(), 1);

        let call = tracker.exit().unwrap();
        assert_eq!(call.caller, pid(1));
        assert_eq!(call.callee, pid(2));
        assert_eq!(tracker.current_depth(), 0);
    }

    #[test]
    fn test_max_depth_enforced() {
        let mut tracker = CpiTracker::new(2);
        tracker.enter(make_call(1, 2)).unwrap();
        tracker.enter(make_call(2, 3)).unwrap();
        let result = tracker.enter(make_call(3, 4));
        assert_eq!(result, Err(CpiError::MaxDepthExceeded { max: 2 }));
    }

    #[test]
    fn test_exit_empty_stack() {
        let mut tracker = CpiTracker::new(4);
        assert_eq!(tracker.exit(), Err(CpiError::EmptyStack));
    }

    #[test]
    fn test_reentrancy_detected() {
        let mut tracker = CpiTracker::new(4);
        tracker.enter(make_call(1, 2)).unwrap();
        // Program 2 is already on the stack as a callee; calling it again is reentrant
        let result = tracker.enter(make_call(2, 2));
        assert_eq!(
            result,
            Err(CpiError::ReentrantCall { program_id: pid(2) })
        );
    }

    #[test]
    fn test_is_reentrant() {
        let mut tracker = CpiTracker::new(4);
        tracker.enter(make_call(1, 2)).unwrap();
        assert!(tracker.is_reentrant(&pid(2)));
        assert!(!tracker.is_reentrant(&pid(3)));
    }

    #[test]
    fn test_privilege_escalation_signer() {
        let caller = vec![AccountPrivilege {
            key: pid(1),
            is_signer: false,
            is_writable: true,
        }];
        let callee = vec![AccountPrivilege {
            key: pid(1),
            is_signer: true, // escalation!
            is_writable: true,
        }];
        let result = check_privilege_escalation(&caller, &callee);
        assert!(matches!(
            result,
            Err(CpiError::PrivilegeEscalation { account, privilege })
            if account == pid(1) && privilege.contains("signer")
        ));
    }

    #[test]
    fn test_privilege_escalation_writable() {
        let caller = vec![AccountPrivilege {
            key: pid(1),
            is_signer: true,
            is_writable: false,
        }];
        let callee = vec![AccountPrivilege {
            key: pid(1),
            is_signer: true,
            is_writable: true, // escalation!
        }];
        let result = check_privilege_escalation(&caller, &callee);
        assert!(matches!(
            result,
            Err(CpiError::PrivilegeEscalation { account, privilege })
            if account == pid(1) && privilege.contains("writable")
        ));
    }

    #[test]
    fn test_no_privilege_escalation() {
        let caller = vec![AccountPrivilege {
            key: pid(1),
            is_signer: true,
            is_writable: true,
        }];
        let callee = vec![AccountPrivilege {
            key: pid(1),
            is_signer: false, // de-escalation is fine
            is_writable: true,
        }];
        assert!(check_privilege_escalation(&caller, &callee).is_ok());
    }

    #[test]
    fn test_call_stack_view() {
        let mut tracker = CpiTracker::new(4);
        tracker.enter(make_call(1, 2)).unwrap();
        tracker.enter(make_call(2, 3)).unwrap();

        let stack = tracker.call_stack();
        assert_eq!(stack.len(), 2);
        assert_eq!(stack[0].callee, pid(2));
        assert_eq!(stack[1].callee, pid(3));
    }
}
