//! # Challenge 10.8: Unsafe Audit Challenge
//!
//! ## Problem
//! You are given 6 blocks of unsafe code. For each one, determine whether it is
//! sound (safe to call) or unsound (contains undefined behavior). Return a verdict
//! with your reasoning.
//!
//! ## Why This Matters
//! Interviewers show you unsafe code and ask if it's sound. This is a real interview
//! format at Anza, Jump, and other Solana infrastructure companies. Being able to
//! reason about unsafe invariants — aliasing, alignment, initialization, validity —
//! separates senior Rust engineers from everyone else.
//!
//! ## Requirements
//! - Each `unsafe_block_N` function is already implemented (NOT todo!)
//! - Each `audit_N` function must return `UnsafeVerdict { is_sound, reason }`
//! - You must correctly identify 3 sound and 3 unsound blocks
//!
//! ## Constraints
//! - Do NOT modify the unsafe_block functions
//! - Your audit functions must return the correct verdict
//! - Provide a meaningful reason string (not empty)
//!
//! ## Hints
//! - Check for: aliasing &mut, alignment, uninitialized reads, out-of-bounds
//! - Sound unsafe has valid invariants at every step
//! - Unsound unsafe violates Rust's safety guarantees even if it "works" in practice

pub struct UnsafeVerdict {
    pub is_sound: bool,
    pub reason: &'static str,
}

// ============================================================
// Block 1: Transmute between same-size integer types
// ============================================================
pub fn unsafe_block_1(value: u32) -> i32 {
    unsafe { std::mem::transmute::<u32, i32>(value) }
}

pub fn audit_1() -> UnsafeVerdict {
    todo!("Is block 1 sound? u32 → i32 transmute. Both are 4 bytes, same alignment.")
}

// ============================================================
// Block 2: Creating two mutable references to the same data
// ============================================================
pub fn unsafe_block_2() -> (i32, i32) {
    let mut value = 42i32;
    let ptr = &mut value as *mut i32;
    unsafe {
        let ref1 = &mut *ptr;
        let ref2 = &mut *ptr; // second &mut to same location
        *ref1 = 10;
        *ref2 = 20;
        (*ref1, *ref2)
    }
}

pub fn audit_2() -> UnsafeVerdict {
    todo!("Is block 2 sound? Two &mut references to the same data exist simultaneously.")
}

// ============================================================
// Block 3: Reading from a properly initialized slice via raw pointer
// ============================================================
pub fn unsafe_block_3(data: &[u8]) -> u8 {
    if data.is_empty() {
        return 0;
    }
    let ptr = data.as_ptr();
    unsafe { *ptr.add(0) }
}

pub fn audit_3() -> UnsafeVerdict {
    todo!("Is block 3 sound? Reads first byte via raw pointer after checking non-empty.")
}

// ============================================================
// Block 4: Misaligned pointer cast
// ============================================================
pub fn unsafe_block_4(data: &[u8; 5]) -> u32 {
    let ptr = &data[1] as *const u8 as *const u32; // offset 1 — likely misaligned
    unsafe { *ptr }
}

pub fn audit_4() -> UnsafeVerdict {
    todo!("Is block 4 sound? Casts a potentially misaligned pointer to *const u32.")
}

// ============================================================
// Block 5: Vec::from_raw_parts with correct parameters
// ============================================================
pub fn unsafe_block_5() -> Vec<u8> {
    let mut v = vec![1u8, 2, 3, 4, 5];
    let ptr = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    std::mem::forget(v); // prevent double free
    unsafe { Vec::from_raw_parts(ptr, len, cap) }
}

pub fn audit_5() -> UnsafeVerdict {
    todo!("Is block 5 sound? Reconstructs a Vec from its raw parts after forget.")
}

// ============================================================
// Block 6: Reading uninitialized memory
// ============================================================
pub fn unsafe_block_6() -> u64 {
    let value: u64;
    unsafe {
        let ptr = &value as *const u64;
        std::ptr::read(ptr)
    }
}

pub fn audit_6() -> UnsafeVerdict {
    todo!("Is block 6 sound? Reads from an uninitialized u64 variable.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_1() {
        let v = audit_1();
        assert!(!v.reason.is_empty(), "must provide reasoning");
        // u32 → i32 transmute: same size, same alignment, all bit patterns valid
        assert!(v.is_sound, "block 1 should be sound");
    }

    #[test]
    fn test_audit_2() {
        let v = audit_2();
        assert!(!v.reason.is_empty());
        // Two simultaneous &mut to same data = aliasing violation = UB
        assert!(!v.is_sound, "block 2 should be unsound");
    }

    #[test]
    fn test_audit_3() {
        let v = audit_3();
        assert!(!v.reason.is_empty());
        // Bounds-checked, properly aligned, data is initialized
        assert!(v.is_sound, "block 3 should be sound");
    }

    #[test]
    fn test_audit_4() {
        let v = audit_4();
        assert!(!v.reason.is_empty());
        // Misaligned read through *const u32 is UB
        assert!(!v.is_sound, "block 4 should be unsound");
    }

    #[test]
    fn test_audit_5() {
        let v = audit_5();
        assert!(!v.reason.is_empty());
        // Correct use of from_raw_parts with forget to prevent double free
        assert!(v.is_sound, "block 5 should be sound");
    }

    #[test]
    fn test_audit_6() {
        let v = audit_6();
        assert!(!v.reason.is_empty());
        // Reading uninitialized memory is UB
        assert!(!v.is_sound, "block 6 should be unsound");
    }

    // Verify the sound blocks actually produce correct results
    #[test]
    fn test_block_1_works() {
        assert_eq!(unsafe_block_1(42), 42);
        assert_eq!(unsafe_block_1(u32::MAX), -1);
    }

    #[test]
    fn test_block_3_works() {
        assert_eq!(unsafe_block_3(&[42, 1, 2]), 42);
        assert_eq!(unsafe_block_3(&[]), 0);
    }

    #[test]
    fn test_block_5_works() {
        let v = unsafe_block_5();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
