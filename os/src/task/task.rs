//! Types related to task management

use super::TaskContext;
use crate::syscall::NUM_IMPLEMENTED_SYSCALLS;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The numbers of syscall called by task
    ///
    /// Indexed by [crate::syscall::dense_id_syscall]
    pub dense_syscall_times: [u32; NUM_IMPLEMENTED_SYSCALLS],
    /// The time when this task start
    pub start_time: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
