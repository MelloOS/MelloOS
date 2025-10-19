/// Priority-based task scheduler
/// Provides three-level priority scheduling with sleep/wake support

/// Task priority levels
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

impl TaskPriority {
    /// Convert priority to queue index
    pub const fn as_index(self) -> usize {
        self as usize
    }
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
}
