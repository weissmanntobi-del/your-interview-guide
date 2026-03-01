/// # Challenge 01: Mini eBPF Virtual Machine Interpreter
///
/// eBPF (extended Berkeley Packet Filter) is the bytecode format used by Solana's
/// runtime to execute on-chain programs. This challenge implements a simplified eBPF
/// interpreter with a subset of the real instruction set.
///
/// ## Architecture
///
/// - **Registers**: 11 general-purpose 64-bit registers (r0-r10).
///   r10 is the read-only frame pointer and must not be written to.
/// - **Memory**: Fixed-size byte array (default 4096 bytes).
///   All loads and stores operate on 8-byte little-endian values.
/// - **Program Counter**: Advances by 1 after each instruction unless a jump occurs.
/// - **Arithmetic**: All arithmetic operations use wrapping (modular) u64 semantics.
/// - **Execution**: Terminates on `Exit` (returning r0) or when the instruction limit
///   is reached.

/// Errors that can occur during eBPF VM execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    /// Register index is out of the valid range (0..=10).
    InvalidRegister(u8),
    /// Memory access is out of bounds for the VM's memory region.
    OutOfBounds { addr: usize, len: usize },
    /// The maximum number of executed instructions was exceeded.
    InstructionLimitExceeded,
    /// The program counter points beyond the loaded program.
    InvalidInstruction,
    /// Attempted write to the read-only frame pointer register (r10).
    ReadOnlyRegister,
}

/// A single eBPF instruction in the simplified instruction set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EbpfInstruction {
    /// dst = dst + src (wrapping)
    Add(u8, u8),
    /// dst = dst - src (wrapping)
    Sub(u8, u8),
    /// dst = dst * src (wrapping)
    Mul(u8, u8),
    /// dst = src
    Mov(u8, u8),
    /// dst = imm
    LoadImm(u8, u64),
    /// dst = *(u64*)(addr_reg + offset)  -- 8-byte LE load from memory
    LoadMem(u8, u8, i16),
    /// *(u64*)(addr_reg + offset) = src  -- 8-byte LE store to memory
    StoreMem(u8, i16, u8),
    /// if reg_a == reg_b then pc += offset
    JumpEq(u8, u8, i16),
    /// if reg_a > reg_b then pc += offset
    JumpGt(u8, u8, i16),
    /// Halt execution and return r0.
    Exit,
}

/// The eBPF virtual machine state.
pub struct EbpfVm {
    pub registers: [u64; 11],
    pub memory: Vec<u8>,
    pub program: Vec<EbpfInstruction>,
    pub pc: usize,
}

impl EbpfVm {
    /// Create a new VM with the given memory size. r10 (frame pointer) is
    /// initialized to the top of memory.
    pub fn new(mem_size: usize) -> Self {
        todo!()
    }

    /// Load a program (list of instructions) into the VM.
    pub fn load_program(&mut self, instructions: Vec<EbpfInstruction>) {
        todo!()
    }

    /// Execute the loaded program for at most `max_instructions` steps.
    /// Returns the value in r0 on `Exit`, or an appropriate `VmError`.
    pub fn execute(&mut self, max_instructions: u64) -> Result<u64, VmError> {
        todo!()
    }

    /// Read a u64 from memory at the given address (8-byte LE).
    fn mem_load(&self, addr: usize) -> Result<u64, VmError> {
        todo!()
    }

    /// Write a u64 to memory at the given address (8-byte LE).
    fn mem_store(&mut self, addr: usize, value: u64) -> Result<(), VmError> {
        todo!()
    }

    /// Validate that a register index is in range and optionally writable.
    fn check_register(reg: u8, writable: bool) -> Result<(), VmError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        // r1 = 10, r2 = 20, r0 = r1 + r2 => return 30
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(1, 10),
            EbpfInstruction::LoadImm(2, 20),
            EbpfInstruction::Mov(0, 1),
            EbpfInstruction::Add(0, 2),
            EbpfInstruction::Exit,
        ]);
        assert_eq!(vm.execute(100).unwrap(), 30);
    }

    #[test]
    fn test_load_store_memory() {
        // Store 42 at address 0, load it back into r0
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(1, 42),
            EbpfInstruction::LoadImm(2, 0),   // address base = 0
            EbpfInstruction::StoreMem(2, 0, 1),
            EbpfInstruction::LoadMem(0, 2, 0),
            EbpfInstruction::Exit,
        ]);
        assert_eq!(vm.execute(100).unwrap(), 42);
    }

    #[test]
    fn test_conditional_jump() {
        // if r1 == r2 jump over the load of 999 into r0
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(1, 5),
            EbpfInstruction::LoadImm(2, 5),
            EbpfInstruction::LoadImm(0, 1),    // r0 = 1 (success path)
            EbpfInstruction::JumpEq(1, 2, 1),  // skip next instruction
            EbpfInstruction::LoadImm(0, 999),   // should be skipped
            EbpfInstruction::Exit,
        ]);
        assert_eq!(vm.execute(100).unwrap(), 1);
    }

    #[test]
    fn test_instruction_limit_exceeded() {
        // Infinite loop: jump to self
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(1, 1),
            EbpfInstruction::LoadImm(2, 1),
            EbpfInstruction::JumpEq(1, 2, -1), // jump back to self
        ]);
        assert_eq!(vm.execute(50), Err(VmError::InstructionLimitExceeded));
    }

    #[test]
    fn test_out_of_bounds_memory() {
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(1, 4096), // address at very end, no room for 8 bytes
            EbpfInstruction::LoadMem(0, 1, 0),
            EbpfInstruction::Exit,
        ]);
        assert!(matches!(vm.execute(100), Err(VmError::OutOfBounds { .. })));
    }

    #[test]
    fn test_factorial_loop() {
        // Compute 5! = 120 using a loop
        // r1 = n = 5, r0 = accumulator = 1
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(1, 5),     // r1 = n
            EbpfInstruction::LoadImm(0, 1),     // r0 = 1 (accumulator)
            EbpfInstruction::LoadImm(2, 1),     // r2 = 1 (decrement / compare)
            EbpfInstruction::LoadImm(3, 0),     // r3 = 0 (zero for comparison)
            // loop start (pc=4):
            EbpfInstruction::Mul(0, 1),         // r0 *= r1
            EbpfInstruction::Sub(1, 2),         // r1 -= 1
            EbpfInstruction::JumpGt(1, 2, -3),  // if r1 > 1 goto pc=4
            EbpfInstruction::Exit,
        ]);
        assert_eq!(vm.execute(200).unwrap(), 120);
    }

    #[test]
    fn test_frame_pointer_write_fails() {
        // r10 is read-only; writing to it must fail
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(10, 0),
            EbpfInstruction::Exit,
        ]);
        assert_eq!(vm.execute(100), Err(VmError::ReadOnlyRegister));
    }

    #[test]
    fn test_wrapping_arithmetic() {
        // u64::MAX + 1 should wrap to 0
        let mut vm = EbpfVm::new(4096);
        vm.load_program(vec![
            EbpfInstruction::LoadImm(0, u64::MAX),
            EbpfInstruction::LoadImm(1, 1),
            EbpfInstruction::Add(0, 1),
            EbpfInstruction::Exit,
        ]);
        assert_eq!(vm.execute(100).unwrap(), 0);
    }
}
