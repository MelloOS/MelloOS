/// Priority-based task scheduler
/// Provides three-level priority scheduling with sleep/wake support

use super::task::TaskId;

/// Maximum number of tasks per queue
const MAX_TASKS: usize = 64;

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

/// Simple circular queue for task IDs (reused from mod.rs)
struct TaskQueue {
    tasks: [TaskId; MAX_TASKS],
    head: usize,
    tail: usize,
    count: usize,
}

impl TaskQueue {
    const fn new() -> Self {
        Self {
            tasks: [0; MAX_TASKS],
            head: 0,
            tail: 0,
            count: 0,
        }
    }
    
    fn push_back(&mut self, task_id: TaskId) -> bool {
        if self.count >= MAX_TASKS {
            return false;
        }
        
        self.tasks[self.tail] = task_id;
        self.tail = (self.tail + 1) % MAX_TASKS;
        self.count += 1;
        true
    }
    
    fn pop_front(&mut self) -> Option<TaskId> {
        if self.count == 0 {
            return None;
        }
        
        let task_id = self.tasks[self.head];
        self.head = (self.head + 1) % MAX_TASKS;
        self.count -= 1;
        Some(task_id)
    }
    
    fn len(&self) -> usize {
        self.count
    }
    
    fn is_empty(&self) -> bool {
        self.count == 0
    }
}

/// Sleeping task entry
#[derive(Copy, Clone)]
struct SleepingTask {
    task_id: TaskId,
    wake_tick: u64,
    priority: TaskPriority,
    valid: bool, // Whether this slot is occupied
}

impl SleepingTask {
    const fn empty() -> Self {
        Self {
            task_id: 0,
            wake_tick: 0,
            priority: TaskPriority::Normal,
            valid: false,
        }
    }
}

/// Priority scheduler with three ready queues
pub struct PriorityScheduler {
    /// Ready queues for each priority level [Low, Normal, High]
    ready_queues: [TaskQueue; 3],
    
    /// Bitmap tracking non-empty queues for O(1) selection
    /// Bits 0-2 correspond to Low/Normal/High priorities
    non_empty_queues: u8,
    
    /// Array of sleeping tasks (fixed size for no_std)
    sleeping_tasks: [SleepingTask; MAX_TASKS],
    
    /// Current tick count
    current_tick: u64,
    
    /// Preemption disable counter (0 = preemption enabled)
    preempt_disable_count: usize,
}

impl PriorityScheduler {
    /// Create a new priority scheduler
    pub const fn new() -> Self {
        Self {
            ready_queues: [TaskQueue::new(), TaskQueue::new(), TaskQueue::new()],
            non_empty_queues: 0,
            sleeping_tasks: [SleepingTask::empty(); MAX_TASKS],
            current_tick: 0,
            preempt_disable_count: 0,
        }
    }
    
    /// Add task to appropriate priority queue
    pub fn enqueue_task(&mut self, task_id: TaskId, priority: TaskPriority) -> bool {
        let index = priority.as_index();
        let success = self.ready_queues[index].push_back(task_id);
        
        if success {
            // Set the bit for this priority level
            self.non_empty_queues |= 1 << index;
        }
        
        success
    }
    
    /// Select next task to run (highest priority first)
    /// Returns None if all queues are empty
    pub fn select_next(&mut self) -> Option<TaskId> {
        // Check queues from highest to lowest priority
        // High = 2, Normal = 1, Low = 0
        for priority_index in (0..=2).rev() {
            // Check if this queue has tasks using bitmap
            if (self.non_empty_queues & (1 << priority_index)) != 0 {
                if let Some(task_id) = self.ready_queues[priority_index].pop_front() {
                    // Update bitmap if queue is now empty
                    if self.ready_queues[priority_index].is_empty() {
                        self.non_empty_queues &= !(1 << priority_index);
                    }
                    return Some(task_id);
                } else {
                    // Queue was marked as non-empty but pop failed - clear the bit
                    self.non_empty_queues &= !(1 << priority_index);
                }
            }
        }
        
        None
    }
    
    /// Check if all queues are empty
    pub fn is_empty(&self) -> bool {
        self.non_empty_queues == 0
    }
    
    /// Get total number of tasks across all queues
    pub fn len(&self) -> usize {
        self.ready_queues[0].len() + self.ready_queues[1].len() + self.ready_queues[2].len()
    }
    
    /// Put task to sleep for specified ticks
    /// Task will be removed from ready queue and added to sleeping list
    pub fn sleep_task(&mut self, task_id: TaskId, ticks: u64, priority: TaskPriority) -> bool {
        let wake_tick = self.current_tick + ticks;
        
        // Find an empty slot in sleeping_tasks array
        for slot in &mut self.sleeping_tasks {
            if !slot.valid {
                *slot = SleepingTask {
                    task_id,
                    wake_tick,
                    priority,
                    valid: true,
                };
                return true;
            }
        }
        
        // No empty slots available
        false
    }
    
    /// Wake tasks whose sleep time has elapsed
    /// Returns number of tasks woken (for logging)
    pub fn wake_sleeping_tasks(&mut self) -> usize {
        let mut woken_count = 0;
        let current_tick = self.current_tick;
        
        // First pass: collect tasks to wake
        let mut tasks_to_wake = [(0usize, TaskPriority::Normal); MAX_TASKS];
        let mut wake_index = 0;
        
        for slot in &mut self.sleeping_tasks {
            if slot.valid && slot.wake_tick <= current_tick {
                if wake_index < MAX_TASKS {
                    tasks_to_wake[wake_index] = (slot.task_id, slot.priority);
                    wake_index += 1;
                }
                slot.valid = false;
                woken_count += 1;
            }
        }
        
        // Second pass: re-enqueue woken tasks
        for i in 0..wake_index {
            let (task_id, priority) = tasks_to_wake[i];
            self.enqueue_task(task_id, priority);
        }
        
        woken_count
    }
    
    /// Update tick counter and wake tasks
    pub fn tick(&mut self) {
        self.current_tick += 1;
    }
    
    /// Get current tick count
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }
    
    /// Disable preemption (for critical sections)
    pub fn preempt_disable(&mut self) {
        self.preempt_disable_count += 1;
    }
    
    /// Enable preemption
    pub fn preempt_enable(&mut self) {
        if self.preempt_disable_count > 0 {
            self.preempt_disable_count -= 1;
        }
    }
    
    /// Check if preemption is allowed
    pub fn can_preempt(&self) -> bool {
        self.preempt_disable_count == 0
    }
}

/// Global preemption disable function
/// 
/// Disables preemption by incrementing the disable counter.
/// Must be called before acquiring spinlocks in IPC operations.
pub fn preempt_disable() {
    use crate::sched::SCHED;
    if let Some(sched) = SCHED.get() {
        let mut sched = sched.lock();
        sched.priority_sched.preempt_disable();
    }
}

/// Global preemption enable function
/// 
/// Enables preemption by decrementing the disable counter.
/// Must be called after releasing spinlocks in IPC operations.
pub fn preempt_enable() {
    use crate::sched::SCHED;
    if let Some(sched) = SCHED.get() {
        let mut sched = sched.lock();
        sched.priority_sched.preempt_enable();
    }
}
