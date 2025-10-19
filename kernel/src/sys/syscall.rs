//! System Call Interface
//!
//! This module implements the system call interface for userland-kernel communication.
//! It provides syscall entry point, dispatcher, and handler functions.

use crate::serial_println;
use crate::sys::METRICS;

/// Syscall entry point (naked function)
///
/// This function is called when userland invokes int 0x80.
/// It saves all registers, calls the dispatcher, and restores registers.
///
/// Register mapping (x86-64 System V ABI):
/// - RAX: Syscall number (input), return value (output)
/// - RDI: Argument 1
/// - RSI: Argument 2
/// - RDX: Argument 3
#[unsafe(naked)]
#[no_mangle]
pub extern "C" fn syscall_entry() {
    core::arch::naked_asm!(
        // The CPU has already pushed SS, RSP, RFLAGS, CS, RIP
        // We need to save all other registers
        
        // Save caller-saved registers
        "push rax",      // Syscall number
        "push rcx",
        "push rdx",      // Arg 3
        "push rsi",      // Arg 2
        "push rdi",      // Arg 1
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        
        // Save callee-saved registers
        "push rbx",
        "push rbp",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        
        // Clear direction flag (required by ABI)
        "cld",
        
        // Prepare arguments for syscall_dispatcher
        // RDI = syscall_id (from RAX)
        // RSI = arg1 (from RDI)
        // RDX = arg2 (from RSI)
        // RCX = arg3 (from RDX)
        "mov rdi, rax",           // syscall_id
        "mov rsi, [rsp + 120]",   // arg1 (original RDI, saved on stack)
        "mov rdx, [rsp + 112]",   // arg2 (original RSI, saved on stack)
        "mov rcx, [rsp + 104]",   // arg3 (original RDX, saved on stack)
        
        // Call the dispatcher
        "call {dispatcher}",
        
        // RAX now contains the return value
        // Save it temporarily
        "mov r15, rax",
        
        // Restore callee-saved registers
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbp",
        "pop rbx",
        
        // Restore caller-saved registers (except RAX which has return value)
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "add rsp, 8",    // Skip saved RAX
        
        // Restore return value to RAX
        "mov rax, r15",
        
        // Return from interrupt (pops RIP, CS, RFLAGS, RSP, SS)
        "iretq",
        
        dispatcher = sym syscall_dispatcher_wrapper,
    )
}

/// Wrapper for syscall_dispatcher to match calling convention
///
/// This function converts the register arguments to Rust function arguments.
#[no_mangle]
extern "C" fn syscall_dispatcher_wrapper(
    syscall_id: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> isize {
    syscall_dispatcher(syscall_id, arg1, arg2, arg3)
}

/// Syscall numbers
pub const SYS_WRITE: usize = 0;
pub const SYS_EXIT: usize = 1;
pub const SYS_SLEEP: usize = 2;
pub const SYS_IPC_SEND: usize = 3;
pub const SYS_IPC_RECV: usize = 4;

/// Syscall dispatcher
///
/// Routes syscall ID to appropriate handler and increments metrics.
///
/// # Arguments
/// * `syscall_id` - Syscall number (from RAX)
/// * `arg1` - First argument (from RDI)
/// * `arg2` - Second argument (from RSI)
/// * `arg3` - Third argument (from RDX)
///
/// # Returns
/// Result value (0 or positive on success, -1 on error)
pub fn syscall_dispatcher(
    syscall_id: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> isize {
    // Increment metrics counter for this syscall
    METRICS.increment_syscall(syscall_id);
    
    // Dispatch to appropriate handler
    match syscall_id {
        SYS_WRITE => sys_write(arg1, arg2, arg3),
        SYS_EXIT => sys_exit(arg1),
        SYS_SLEEP => sys_sleep(arg1),
        SYS_IPC_SEND => sys_ipc_send(arg1, arg2, arg3),
        SYS_IPC_RECV => sys_ipc_recv(arg1, arg2, arg3),
        _ => {
            serial_println!("[SYSCALL] Invalid syscall ID: {}", syscall_id);
            -1 // Invalid syscall
        }
    }
}

/// sys_write handler - Write data to serial output
///
/// # Arguments
/// * `fd` - File descriptor (only 0/stdout supported in Phase 4)
/// * `buf_ptr` - Pointer to buffer
/// * `len` - Length of data to write
///
/// # Returns
/// Number of bytes written, or -1 on error
fn sys_write(fd: usize, buf_ptr: usize, len: usize) -> isize {
    // Validate file descriptor (only stdout supported)
    if fd != 0 {
        serial_println!("[SYSCALL] sys_write: Invalid fd {}", fd);
        return -1;
    }
    
    // Phase 4: No pointer validation, assume kernel-accessible
    // Phase 5 will add copy_from_user() validation
    
    if buf_ptr == 0 || len == 0 {
        return 0; // Nothing to write
    }
    
    // Convert pointer to slice
    let buffer = unsafe {
        core::slice::from_raw_parts(buf_ptr as *const u8, len)
    };
    
    // Convert to string (lossy for non-UTF8)
    let s = core::str::from_utf8(buffer).unwrap_or("[invalid UTF-8]");
    
    // Write to serial
    serial_println!("[USERLAND] {}", s);
    
    len as isize
}

/// sys_exit handler - Terminate current task
///
/// # Arguments
/// * `code` - Exit code
///
/// # Returns
/// Never returns
fn sys_exit(code: usize) -> ! {
    serial_println!("[SYSCALL] sys_exit: Task exiting with code {}", code);
    
    // TODO: Mark task as terminated and remove from all queues
    // For now, just loop forever
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// sys_sleep handler - Put task to sleep for specified ticks
///
/// # Arguments
/// * `ticks` - Number of ticks to sleep
///
/// # Returns
/// 0 on success, -1 on error
fn sys_sleep(ticks: usize) -> isize {
    // Validate tick count
    if ticks == 0 {
        return 0; // Sleep for 0 ticks is a no-op
    }
    
    // Get current task ID and priority from scheduler
    let (task_id, priority) = match crate::sched::get_current_task_info() {
        Some(info) => info,
        None => {
            serial_println!("[SYSCALL] sys_sleep: No current task");
            return -1;
        }
    };
    
    serial_println!("[SYSCALL] sys_sleep: Task {} sleeping for {} ticks", task_id, ticks);
    
    // Call scheduler to put task to sleep
    if !crate::sched::sleep_current_task(ticks as u64, priority) {
        serial_println!("[SYSCALL] sys_sleep: Failed to sleep task");
        return -1;
    }
    
    // Increment sleep counter metric
    use core::sync::atomic::Ordering;
    METRICS.sleep_count.fetch_add(1, Ordering::Relaxed);
    
    // Trigger scheduler to select next task
    // This will context switch away from the current task
    crate::sched::yield_now();
    
    // When we wake up, we return here
    0
}

/// sys_ipc_send handler - Send message to port
///
/// # Arguments
/// * `port_id` - Target port ID
/// * `_buf_ptr` - Pointer to message buffer (unused in Phase 4)
/// * `len` - Length of message
///
/// # Returns
/// 0 on success, -1 on error
fn sys_ipc_send(port_id: usize, _buf_ptr: usize, len: usize) -> isize {
    serial_println!("[SYSCALL] sys_ipc_send: port={}, len={}", port_id, len);
    
    // TODO: Implement IPC send
    // For now, return not implemented
    -1
}

/// sys_ipc_recv handler - Receive message from port (blocking)
///
/// # Arguments
/// * `port_id` - Source port ID
/// * `_buf_ptr` - Pointer to receive buffer (unused in Phase 4)
/// * `len` - Maximum length to receive
///
/// # Returns
/// Number of bytes received, or -1 on error
fn sys_ipc_recv(port_id: usize, _buf_ptr: usize, len: usize) -> isize {
    serial_println!("[SYSCALL] sys_ipc_recv: port={}, max_len={}", port_id, len);
    
    // TODO: Implement IPC receive
    // For now, return not implemented
    -1
}
