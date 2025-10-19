/// System call and IPC subsystem
pub mod syscall;
pub mod ipc;
pub mod port;

use core::sync::atomic::{AtomicUsize, Ordering};

/// Kernel metrics for observability
pub struct KernelMetrics {
    pub ctx_switches: AtomicUsize,
    pub preemptions: AtomicUsize,
    pub syscall_count: [AtomicUsize; 5],
    pub ipc_sends: AtomicUsize,
    pub ipc_recvs: AtomicUsize,
    pub ipc_queue_full: AtomicUsize,
    pub sleep_count: AtomicUsize,
    pub wake_count: AtomicUsize,
    pub timer_ticks: AtomicUsize,
}

impl KernelMetrics {
    pub const fn new() -> Self {
        const ATOMIC_ZERO: AtomicUsize = AtomicUsize::new(0);
        Self {
            ctx_switches: ATOMIC_ZERO,
            preemptions: ATOMIC_ZERO,
            syscall_count: [ATOMIC_ZERO; 5],
            ipc_sends: ATOMIC_ZERO,
            ipc_recvs: ATOMIC_ZERO,
            ipc_queue_full: ATOMIC_ZERO,
            sleep_count: ATOMIC_ZERO,
            wake_count: ATOMIC_ZERO,
            timer_ticks: ATOMIC_ZERO,
        }
    }

    pub fn increment_syscall(&self, syscall_id: usize) {
        if syscall_id < 5 {
            self.syscall_count[syscall_id].fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Global kernel metrics instance
pub static METRICS: KernelMetrics = KernelMetrics::new();
