//! CPU Context and Context Switching
//!
//! This module defines the CPU context structure and implements context switching
//! using inline assembly. It handles saving and restoring CPU registers during
//! task switches.

/// CPU Context structure
/// 
/// Contains all callee-saved registers according to x86_64 System V ABI.
/// The layout must match the order in which registers are pushed/popped
/// in the context_switch assembly code.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CpuContext {
    /// Callee-saved registers (must be preserved across function calls)
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    
    /// Stack pointer - points to the top of the task's stack
    pub rsp: u64,
}

impl CpuContext {
    /// Create a new zeroed context
    pub const fn new() -> Self {
        Self {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbp: 0,
            rbx: 0,
            rsp: 0,
        }
    }
}
