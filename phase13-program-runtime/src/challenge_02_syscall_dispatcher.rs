/// # Challenge 02: Solana Runtime Syscall Dispatcher
///
/// Solana programs running inside the BPF VM communicate with the runtime through
/// syscalls. Each syscall has a unique identifier, a compute cost, and reads/writes
/// the VM's memory to exchange data.
///
/// This challenge implements a syscall dispatcher that:
/// - Routes calls to the correct handler based on `SyscallId`.
/// - Enforces compute budget limits by deducting costs *before* execution.
/// - Provides `SolLog` (logging) and `SolGetClockSysvar` (clock data) handlers.
///
/// ## Compute Metering
///
/// Every syscall has a base cost. For `SolLog`, the cost is `100 + length` of the
/// logged string. The dispatcher must check that sufficient compute remains
/// **before** deducting; if not, it returns `InsufficientCompute` without modifying
/// the remaining budget.

/// Identifiers for supported syscalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyscallId {
    SolLog,
    SolLogPubkey,
    SolCreateProgramAddress,
    SolGetClockSysvar,
    SolInvoke,
    SolGetRentSysvar,
}

/// Errors returned by the syscall dispatcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallError {
    /// The requested syscall has not been registered.
    UnknownSyscall,
    /// Not enough compute budget remaining for this operation.
    InsufficientCompute { required: u64, remaining: u64 },
    /// Memory access is out of bounds.
    OutOfBounds { offset: u64, len: u64 },
    /// Data read from memory is not valid UTF-8 (for SolLog).
    InvalidUtf8,
    /// Generic execution failure with a description.
    ExecutionError(String),
}

pub type SyscallResult = Result<u64, SyscallError>;

/// Mutable context passed into every syscall invocation.
pub struct SyscallContext<'a> {
    /// The VM's linear memory.
    pub memory: &'a mut [u8],
    /// Remaining compute units; decremented on each call.
    pub compute_remaining: &'a mut u64,
    /// Collected log messages (populated by SolLog).
    pub log_messages: &'a mut Vec<String>,
}

/// Registration entry pairing a syscall with its base cost.
#[derive(Debug, Clone)]
struct SyscallEntry {
    id: SyscallId,
    base_cost: u64,
}

/// Dispatcher that routes syscall invocations to the appropriate handler.
pub struct SyscallDispatcher {
    pub registered: Vec<(SyscallId, u64)>,
}

impl SyscallDispatcher {
    /// Create a new dispatcher with no registered syscalls.
    pub fn new() -> Self {
        todo!()
    }

    /// Register a syscall with its base compute cost.
    pub fn register(&mut self, id: SyscallId, base_cost: u64) {
        todo!()
    }

    /// Dispatch a syscall. `args` is the standard 5-element register window.
    ///
    /// Steps:
    /// 1. Look up the syscall; return `UnknownSyscall` if not registered.
    /// 2. Compute the effective cost (may depend on args, e.g. SolLog adds len).
    /// 3. Check compute budget; return `InsufficientCompute` without deducting if short.
    /// 4. Deduct compute and execute the handler.
    pub fn dispatch(
        &self,
        id: SyscallId,
        args: [u64; 5],
        ctx: &mut SyscallContext,
    ) -> SyscallResult {
        todo!()
    }

    /// Compute the effective cost for a syscall given the base cost and arguments.
    fn effective_cost(&self, id: SyscallId, base_cost: u64, args: &[u64; 5]) -> u64 {
        todo!()
    }

    /// Handler for SolLog: reads a UTF-8 string from memory[offset..offset+len]
    /// and appends it to `ctx.log_messages`. Returns 0 on success.
    fn handle_sol_log(args: &[u64; 5], ctx: &mut SyscallContext) -> SyscallResult {
        todo!()
    }

    /// Handler for SolGetClockSysvar: writes synthetic clock data at memory[offset].
    /// Layout (little-endian): slot(8) + epoch(8) + unix_timestamp(8) = 24 bytes.
    /// Uses placeholder values: slot=100, epoch=2, timestamp=1_700_000_000.
    fn handle_get_clock(args: &[u64; 5], ctx: &mut SyscallContext) -> SyscallResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dispatcher() -> SyscallDispatcher {
        let mut d = SyscallDispatcher::new();
        d.register(SyscallId::SolLog, 100);
        d.register(SyscallId::SolGetClockSysvar, 100);
        d
    }

    fn make_context(
        memory: &mut [u8],
        compute: &mut u64,
        logs: &mut Vec<String>,
    ) -> SyscallContext {
        SyscallContext {
            memory,
            compute_remaining: compute,
            log_messages: logs,
        }
    }

    #[test]
    fn test_sol_log_reads_memory() {
        let dispatcher = make_dispatcher();
        let msg = b"hello world";
        let mut mem = vec![0u8; 256];
        mem[0..msg.len()].copy_from_slice(msg);
        let mut compute = 10_000u64;
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        let result = dispatcher.dispatch(
            SyscallId::SolLog,
            [0, msg.len() as u64, 0, 0, 0],
            &mut ctx,
        );
        assert_eq!(result.unwrap(), 0);
        assert_eq!(logs[0], "hello world");
    }

    #[test]
    fn test_compute_enforced() {
        let dispatcher = make_dispatcher();
        let mut mem = vec![0u8; 256];
        mem[0..5].copy_from_slice(b"hello");
        let mut compute = 50u64; // need 100 + 5 = 105
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        let result = dispatcher.dispatch(
            SyscallId::SolLog,
            [0, 5, 0, 0, 0],
            &mut ctx,
        );
        assert!(matches!(
            result,
            Err(SyscallError::InsufficientCompute { .. })
        ));
        // Compute must not have been deducted
        assert_eq!(*ctx.compute_remaining, 50);
    }

    #[test]
    fn test_unknown_syscall() {
        let dispatcher = make_dispatcher();
        let mut mem = vec![0u8; 64];
        let mut compute = 10_000u64;
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        let result = dispatcher.dispatch(
            SyscallId::SolInvoke,
            [0; 5],
            &mut ctx,
        );
        assert_eq!(result, Err(SyscallError::UnknownSyscall));
    }

    #[test]
    fn test_clock_sysvar_writes_memory() {
        let dispatcher = make_dispatcher();
        let mut mem = vec![0u8; 256];
        let mut compute = 10_000u64;
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        let result = dispatcher.dispatch(
            SyscallId::SolGetClockSysvar,
            [0, 0, 0, 0, 0], // offset = 0
            &mut ctx,
        );
        assert_eq!(result.unwrap(), 0);
        // Verify slot = 100 at offset 0
        let slot = u64::from_le_bytes(ctx.memory[0..8].try_into().unwrap());
        assert_eq!(slot, 100);
        // Verify epoch = 2 at offset 8
        let epoch = u64::from_le_bytes(ctx.memory[8..16].try_into().unwrap());
        assert_eq!(epoch, 2);
        // Verify timestamp at offset 16
        let ts = u64::from_le_bytes(ctx.memory[16..24].try_into().unwrap());
        assert_eq!(ts, 1_700_000_000);
    }

    #[test]
    fn test_multiple_calls_consume_compute() {
        let dispatcher = make_dispatcher();
        let mut mem = vec![0u8; 256];
        mem[0..2].copy_from_slice(b"hi");
        let mut compute = 500u64;
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        // Each SolLog("hi") costs 100 + 2 = 102
        for _ in 0..4 {
            dispatcher
                .dispatch(SyscallId::SolLog, [0, 2, 0, 0, 0], &mut ctx)
                .unwrap();
        }
        assert_eq!(*ctx.compute_remaining, 500 - 4 * 102);
        // Fifth call should fail: need 102, have 92
        let result = dispatcher.dispatch(SyscallId::SolLog, [0, 2, 0, 0, 0], &mut ctx);
        assert!(matches!(
            result,
            Err(SyscallError::InsufficientCompute { .. })
        ));
    }

    #[test]
    fn test_sol_log_out_of_bounds() {
        let dispatcher = make_dispatcher();
        let mut mem = vec![0u8; 64];
        let mut compute = 10_000u64;
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        let result = dispatcher.dispatch(
            SyscallId::SolLog,
            [60, 10, 0, 0, 0], // 60 + 10 > 64
            &mut ctx,
        );
        assert!(matches!(result, Err(SyscallError::OutOfBounds { .. })));
    }

    #[test]
    fn test_invalid_utf8() {
        let dispatcher = make_dispatcher();
        let mut mem = vec![0xFFu8; 64]; // invalid UTF-8
        let mut compute = 10_000u64;
        let mut logs = Vec::new();
        let mut ctx = make_context(&mut mem, &mut compute, &mut logs);

        let result = dispatcher.dispatch(
            SyscallId::SolLog,
            [0, 4, 0, 0, 0],
            &mut ctx,
        );
        assert_eq!(result, Err(SyscallError::InvalidUtf8));
    }
}
