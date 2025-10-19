/// System call interface module
/// Provides syscall numbers and dispatcher

/// Syscall numbers
pub const SYS_WRITE: usize = 0;
pub const SYS_EXIT: usize = 1;
pub const SYS_SLEEP: usize = 2;
pub const SYS_IPC_SEND: usize = 3;
pub const SYS_IPC_RECV: usize = 4;
