/// IPC subsystem module
/// Provides message passing between tasks

/// IPC error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    InvalidPort,
    QueueFull,
    InvalidBuffer,
    PortNotFound,
    MessageTooLarge,
    NotImplemented,
}
